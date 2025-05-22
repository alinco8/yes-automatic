use std::{
    fs,
    path::Path,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::spawn,
};

use anyhow::Context;
use crossbeam::channel as ch;
use device_query::{DeviceState, Keycode};
use enigo::{Direction, Enigo, Key, Keyboard, Mouse, Settings};
use mlua::{Function, Lua, LuaOptions, StdLib, VmState};

use super::model::{ButtonSend, Coordinate, KeySend};

pub enum LuaEvent {
    KeyboardPress {
        key: KeySend,
    },
    KeyboardRelease {
        key: KeySend,
    },
    KeyboardClick {
        key: KeySend,
    },
    KeyboardIsPressing {
        key: Keycode,
        res: ch::Sender<bool>,
    },
    KeyboardCharPress {
        char: char,
    },
    KeyboardCharRelease {
        char: char,
    },
    KeyboardCharClick {
        char: char,
    },
    MouseGetPos {
        res: ch::Sender<(i32, i32)>,
    },
    MouseMove {
        x: i32,
        y: i32,
        coordinate: Coordinate,
    },
    MousePress {
        button: ButtonSend,
    },
    MouseRelease {
        button: ButtonSend,
    },
    MouseClick {
        button: ButtonSend,
    },
    MouseIsPressing {
        button: u32,
        res: ch::Sender<bool>,
    },
}

pub struct LuaInstance {
    pub lua: Lua,
    pub exit_flag: Arc<AtomicBool>,
}
impl LuaInstance {
    pub fn create_from_file<FP: AsRef<Path>, SP: AsRef<Path>>(
        file_path: FP,
        std_path: SP,
    ) -> anyhow::Result<Self> {
        let lua = Lua::new_with(StdLib::ALL, LuaOptions::default())?;
        let (sender, receiver) = ch::unbounded::<LuaEvent>();
        register_builtins(&lua, Arc::new(sender), std_path.as_ref())?;
        spawn(move || {
            let mut enigo = Enigo::new(&Settings::default())?;
            let state = DeviceState::new();

            while let Ok(event) = receiver.recv() {
                match event {
                    LuaEvent::KeyboardPress { key } => {
                        enigo.key(key.into(), Direction::Press)?;
                    }
                    LuaEvent::KeyboardRelease { key } => {
                        enigo.key(key.into(), Direction::Release)?;
                    }
                    LuaEvent::KeyboardClick { key } => {
                        enigo.key(key.into(), Direction::Click)?;
                    }
                    LuaEvent::KeyboardIsPressing { key, res } => {
                        res.send(state.query_keymap().contains(&key))?;
                    }

                    LuaEvent::KeyboardCharPress { char } => {
                        enigo.key(Key::Unicode(char), Direction::Press)?;
                    }
                    LuaEvent::KeyboardCharRelease { char } => {
                        enigo.key(Key::Unicode(char), Direction::Release)?;
                    }
                    LuaEvent::KeyboardCharClick { char } => {
                        enigo.key(Key::Unicode(char), Direction::Click)?;
                    }
                    LuaEvent::MouseGetPos { res, .. } => {
                        res.send(state.query_pointer().coords)?;
                    }
                    LuaEvent::MouseMove { x, y, coordinate } => {
                        enigo.move_mouse(x, y, coordinate.into())?;
                    }
                    LuaEvent::MousePress { button } => {
                        enigo.button(button.into(), Direction::Press)?;
                    }
                    LuaEvent::MouseRelease { button } => {
                        enigo.button(button.into(), Direction::Release)?;
                    }
                    LuaEvent::MouseClick { button } => {
                        enigo.button(button.into(), Direction::Click)?;
                    }
                    LuaEvent::MouseIsPressing { button, res } => {
                        res.send(
                            *state
                                .query_pointer()
                                .button_pressed
                                .get(button as usize)
                                .ok_or(mlua::Error::RuntimeError(format!(
                                    "Invalid button index: {}",
                                    button
                                )))?,
                        )?;
                    }
                }
            }

            Ok::<(), anyhow::Error>(())
        });

        let exit_flag = Arc::new(AtomicBool::new(false));
        {
            let exit_flag = Arc::clone(&exit_flag);
            lua.set_interrupt(move |_| {
                if exit_flag.load(Ordering::SeqCst) {
                    Err(mlua::Error::RuntimeError("interrupted".to_string()))
                } else {
                    Ok(VmState::Continue)
                }
            });
        }
        lua.load(std::fs::read(&file_path)?).exec()?;

        Ok(LuaInstance { lua, exit_flag })
    }
    pub fn execute(&self) -> mlua::Result<()> {
        let entry: Function = self.lua.globals().get("Main")?;
        let thread = self.lua.create_thread(entry)?;
        self.exit_flag.store(false, Ordering::SeqCst);
        thread.resume::<()>(())?;
        Ok(())
    }

    pub fn stop(&self) -> anyhow::Result<()> {
        self.exit_flag.store(true, Ordering::SeqCst);
        Ok(())
    }
}

fn register_builtins<P: AsRef<Path>>(
    lua: &Lua,
    channel: Arc<ch::Sender<LuaEvent>>,
    std_path: P,
) -> anyhow::Result<()> {
    let std_path = std_path.as_ref().to_string_lossy().to_string();

    macro_rules! declare_function {
        (
            $name:expr, $event:expr,
            $(
                $arg:ident: $arg_type:ty
            ),*
        ) => {{
            let channel = Arc::clone(&channel);
            (
                $name,
                lua.create_function(move |_, $($arg: $arg_type),*| {
                    channel.send($event).map_err(|e| {
                        mlua::Error::RuntimeError(format!("Failed to send event: {}", e))
                    })?;
                    Ok(())
                })?,
            )
        }};
    }

    lua.load(format!(
        r#"package.path = package.path .. ";{}/?.lua""#,
        std_path
    ))
    .exec()
    .context("Failed to set package.path")?;
    lua.load(fs::read(format!("{}/entry.lua", std_path))?)
        .exec()
        .context("Failed to load entry.lua")?;

    let globals = lua.globals();

    declare_function!(
        "press",
        LuaEvent::KeyboardPress {
            key: KeySend::from_str(&key).map_err(|_| {
                mlua::Error::RuntimeError(format!("Invalid key: {}", key))
            })?,
        },
        key: String
    );

    globals.set("keyboard", {
        let keyboard = lua.create_table_from([
            declare_function!(
                "press",
                LuaEvent::KeyboardPress {
                    key: KeySend::from_str(&key).map_err(|_| {
                        mlua::Error::RuntimeError(format!("Invalid key: {}", key))
                    })?,
                },
                key: String
            ),
            declare_function!(
                "release",
                LuaEvent::KeyboardRelease {
                    key: KeySend::from_str(&key).map_err(|_| {
                        mlua::Error::RuntimeError(format!("Invalid key: {}", key))
                    })?,
                },
                key: String
            ),
            declare_function!(
                "click",
                LuaEvent::KeyboardClick {
                    key: KeySend::from_str(&key).map_err(|_| {
                        mlua::Error::RuntimeError(format!("Invalid key: {}", key))
                    })?,
                },
                key: String
            ),
            {
                let channel = channel.clone();
                (
                    "is_pressing",
                    lua.create_function(move |_, key: String| {
                        let (sender, receiver) = ch::unbounded();
                        channel
                            .send(LuaEvent::KeyboardIsPressing {
                                key: Keycode::from_str(&key).map_err(|_| {
                                    mlua::Error::RuntimeError(format!("Invalid key: {}", key))
                                })?,
                                res: sender,
                            })
                            .map_err(|e| {
                                mlua::Error::RuntimeError(format!("Failed to send event: {}", e))
                            })?;
                        Ok(receiver.recv().unwrap())
                    })?,
                )
            },
        ])?;
        keyboard.set(
            "char",
            lua.create_table_from([
                declare_function!(
                    "press",
                    LuaEvent::KeyboardCharPress {
                        char: char.chars().next().ok_or_else(|| {
                            mlua::Error::RuntimeError("Invalid character".to_string())
                        })?,
                    },
                    char: String
                ),
                declare_function!(
                    "release",
                    LuaEvent::KeyboardCharRelease {
                        char: char.chars().next().ok_or_else(|| {
                            mlua::Error::RuntimeError("Invalid character".to_string())
                        })?,
                    },
                    char: String
                ),
                declare_function!(
                    "click",
                    LuaEvent::KeyboardCharClick {
                        char: char.chars().next().ok_or_else(|| {
                            mlua::Error::RuntimeError("Invalid character".to_string())
                        })?,
                    },
                    char: String
                ),
            ])?,
        )?;

        keyboard
    })?;
    globals.set(
        "mouse",
        lua.create_table_from([
            {
                let channel = Arc::clone(&channel);
                (
                    "get_pos",
                    lua.create_function(move |_, ()| {
                        let (sender, receiver) = ch::bounded(1);
                        channel
                            .send(LuaEvent::MouseGetPos { res: sender })
                            .map_err(|e| {
                                mlua::Error::RuntimeError(format!("Failed to send event: {}", e))
                            })?;
                        Ok(receiver.recv().unwrap())
                    })?,
                )
            },
            {
                let channel = Arc::clone(&channel);
                (
                    "move",
                    lua.create_function(move |_, (x, y, coordinate): (i32, i32, String)| {
                        channel
                            .send(LuaEvent::MouseMove {
                                x,
                                y,
                                coordinate: Coordinate::from_str(&coordinate).map_err(|_| {
                                    mlua::Error::RuntimeError(format!(
                                        "Invalid coordinate: {}",
                                        coordinate
                                    ))
                                })?,
                            })
                            .map_err(|e| {
                                mlua::Error::RuntimeError(format!("Failed to send event: {}", e))
                            })?;
                        Ok(())
                    })?,
                )
            },
            declare_function!(
                "press",
                LuaEvent::MousePress {
                    button: ButtonSend::from_str(&button).map_err(|_| {
                        mlua::Error::RuntimeError(format!("Invalid button: {}", button))
                    })?,
                },
                button: String
            ),
            declare_function!(
                "release",
                LuaEvent::MouseRelease {
                    button: ButtonSend::from_str(&button).map_err(|_| {
                        mlua::Error::RuntimeError(format!("Invalid button: {}", button))
                    })?,
                },
                button: String
            ),
            declare_function!(
                "click",
                LuaEvent::MouseClick {
                    button: ButtonSend::from_str(&button).map_err(|_| {
                        mlua::Error::RuntimeError(format!("Invalid button: {}", button))
                    })?,
                },
                button: String
            ),
            {
                let channel = Arc::clone(&channel);
                (
                    "is_pressing",
                    lua.create_function(move |_, button: String| {
                        let (sender, receiver) = ch::bounded(1);
                        channel
                            .send(LuaEvent::MouseIsPressing {
                                button: u32::from_str(&button).map_err(|_| {
                                    mlua::Error::RuntimeError(format!("Invalid button: {}", button))
                                })?,
                                res: sender,
                            })
                            .map_err(|e| {
                                mlua::Error::RuntimeError(format!("Failed to send event: {}", e))
                            })?;
                        Ok(receiver.recv().unwrap())
                    })?,
                )
            },
        ])?,
    )?;

    globals.set(
        "sleep",
        lua.create_function(move |_, ms: u64| {
            std::thread::sleep(std::time::Duration::from_millis(ms));
            Ok(())
        })?,
    )?;

    lua.load(fs::read(format!("{}/post.lua", std_path))?)
        .exec()
        .context("Failed to load post.lua")?;

    Ok(())
}
