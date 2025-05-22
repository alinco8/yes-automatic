use std::{
    fs, io,
    path::Path,
    sync::{Arc, Mutex},
};

use lua::LuaManager;
use tauri::{
    menu::{CheckMenuItem, MenuBuilder},
    path::BaseDirectory,
    tray::TrayIconBuilder,
    ActivationPolicy, App, AppHandle, Manager, Runtime,
};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

mod lua;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            if let Err(err) = setup_app(app) {
                app.dialog()
                    .message(format!("アプリの起動中に問題が発生しました。\n\n{}", err))
                    .title("エラーが発生しました")
                    .kind(tauri_plugin_dialog::MessageDialogKind::Error)
                    .show(|_| {});

                return Err(err.into());
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_app(app: &mut App) -> anyhow::Result<()> {
    let config_dir = app.path().app_config_dir()?;

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }
    let vscode_dir = app
        .path()
        .resolve("data/.vscode", BaseDirectory::Resource)?;
    copy_dir_all(vscode_dir, config_dir.join(".vscode"))?;

    #[cfg(target_os = "macos")]
    {
        app.set_activation_policy(ActivationPolicy::Accessory);
    }

    let mut items = Vec::new();

    let mut menu_builder = MenuBuilder::new(app);
    for path in fs::read_dir(app.path().app_config_dir()?)?.filter_map(|entry| {
        entry.ok().and_then(|entry| {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "lua") {
                Some(path)
            } else {
                None
            }
        })
    }) {
        let content = fs::read_to_string(&path)?;
        let title = content
            .lines()
            .next()
            .and_then(|line| line.split_once("--[["))
            .and_then(|(_, header)| header.split_once("]]"))
            .map(|(header, _)| header.trim());
        if let Some(title) = title {
            let item = CheckMenuItem::new(app, title, true, false, None::<String>)?;
            menu_builder = menu_builder.item(&item);

            items.push((item, title.to_string(), path.to_string_lossy().to_string()));
        } else {
            eprintln!("Invalid file format: {}", path.display());
        }
    }
    let menu = menu_builder
        .separator()
        .text("open-scripts", "Scriptsフォルダを開く")
        .quit_with_text("終了")
        .build()?;

    let ctx = Arc::new(Mutex::new((LuaManager::new(), Some(String::new()))));
    let tray = TrayIconBuilder::new()
        .icon(
            app.default_window_icon()
                .expect("Failed to get default window icon")
                .clone(),
        )
        .on_menu_event(move |app, e| {
            if let Err(err) = on_menu_event(app, e, &items, ctx.clone()) {
                app.dialog()
                    .message(format!(
                        "メニューの処理中にエラーが発生しました。\n\n{}",
                        err
                    ))
                    .title("エラーが発生しました")
                    .kind(tauri_plugin_dialog::MessageDialogKind::Error)
                    .show(|_| {});
            }
        })
        .build(app)?;
    tray.set_menu(Some(menu))?;

    Ok(())
}

fn on_menu_event(
    app: &AppHandle,
    e: tauri::menu::MenuEvent,
    items: &[(CheckMenuItem<impl Runtime>, String, String)],
    ctx: Arc<Mutex<(LuaManager, Option<String>)>>,
) -> anyhow::Result<()> {
    if e.id.0.as_str() == "open-scripts" {
        app.opener().open_path(
            app.path().app_config_dir()?.to_string_lossy().to_string(),
            None::<&str>,
        )?;
    }

    let (manager, active) = &mut *ctx.lock().unwrap();
    *active = if let Some((item, _, _)) = items.iter().find(|(item, _, _)| item.id() == e.id()) {
        if active.as_ref() == Some(&item.id().0) {
            None
        } else {
            Some(item.id().0.clone())
        }
    } else {
        None
    };
    for (item, _, _) in items.iter() {
        item.set_checked(active.as_ref() == Some(&item.id().0))?;
    }

    if let Some(active) = active.as_ref() {
        let (_, _, path) = items
            .iter()
            .find(|(item, _, _)| active == &item.id().0)
            .ok_or(anyhow::anyhow!("Failed to find active item"))?;

        let app = app.clone();
        manager.execute_from_file(
            path,
            app.path().app_config_dir()?.join(".vscode/yam-docs"),
            move |err| {
                app.dialog()
                    .message(format!(
                        "Luaスクリプトの実行中にエラーが発生しました。\n\n{}",
                        err,
                    ))
                    .title("エラーが発生しました")
                    .kind(tauri_plugin_dialog::MessageDialogKind::Error)
                    .show(|_| {});
            },
        )?;
    } else {
        manager.stop_current()?;
    }

    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(&src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.as_ref().join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(entry.path(), &dst_path)?;
        } else if !dst_path.exists() {
            fs::copy(entry.path(), &dst_path)?;
        }
    }
    Ok(())
}
