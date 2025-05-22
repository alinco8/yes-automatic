use std::{
    path::Path,
    sync::Arc,
    thread::{spawn, JoinHandle},
};

use super::LuaInstance;

pub struct LuaManager {
    current: Option<Arc<LuaInstance>>,
    current_thread: Option<JoinHandle<()>>,
}
impl LuaManager {
    pub fn new() -> Self {
        LuaManager {
            current: None,
            current_thread: None,
        }
    }
    pub fn execute_from_file<FP: AsRef<Path>, SP: AsRef<Path>, F>(
        &mut self,
        file_path: FP,
        std_path: SP,
        f: F,
    ) -> anyhow::Result<()>
    where
        F: FnOnce(mlua::Error) + Send + 'static,
    {
        self.stop_current()?;

        let instance = Arc::new(LuaInstance::create_from_file(file_path, std_path)?);
        self.current = Some(Arc::clone(&instance));
        self.current_thread = Some(spawn(move || {
            if let Err(err) = instance.execute() {
                if let mlua::Error::CallbackError { cause, .. } = &err {
                    if let mlua::Error::RuntimeError(cause) = &**cause {
                        if cause == "interrupted" {
                            return;
                        }
                    }
                }

                f(err);
            };
        }));

        Ok(())
    }
    pub fn stop_current(&mut self) -> anyhow::Result<()> {
        if let Some(curr) = &self.current.take() {
            curr.stop()?;

            if let Some(handle) = self.current_thread.take() {
                handle
                    .join()
                    .map_err(|_| anyhow::anyhow!("Failed to join Lua thread"))?;
            }
        }

        Ok(())
    }
}
