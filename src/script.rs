use std::{
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use mlua::AnyUserData;

use crate::{Tree, UiBuilder};

pub struct Script {
    path: PathBuf,
    update: mlua::Function,
    events: std::sync::mpsc::Receiver<()>,
    _handle: Option<std::thread::JoinHandle<()>>,
}

impl Script {
    pub fn new(
        path: impl Into<PathBuf>,
        timeout: Option<Duration>,
        lua: &mlua::Lua,
    ) -> mlua::Result<Self> {
        let path = path.into();
        let data = std::fs::read_to_string(&path)?;

        let (tx, events) = std::sync::mpsc::channel();

        Ok(Self {
            update: lua.load(data).eval()?,
            events,
            _handle: timeout.map(|timeout| Self::watch_for_changes(tx, path.clone(), timeout)),
            path,
        })
    }

    pub fn update(&self, lua: &mlua::Lua) -> mlua::Result<()> {
        lua.set_app_data(Tree::new(lua).unwrap());
        let data = lua.globals().get::<AnyUserData>("__USER_STATE").ok();
        self.update.call::<()>((UiBuilder, data))
    }

    pub fn reload(&mut self, lua: &mlua::Lua) -> mlua::Result<()> {
        let data = std::fs::read_to_string(&self.path)?;
        self.update = lua.load(data).eval()?;
        Ok(())
    }

    pub fn should_reload(&mut self) -> bool {
        self.events.try_recv().is_ok()
    }

    fn watch_for_changes(
        tx: std::sync::mpsc::Sender<()>,
        path: PathBuf,
        timeout: Duration,
    ) -> std::thread::JoinHandle<()> {
        // TODO if we're going to support require we should read all of the
        // files in the dir and sort by the their modified time
        fn last_modified(path: &Path) -> Option<SystemTime> {
            let md = std::fs::metadata(path).ok()?;
            md.is_file().then_some(md.modified().ok()?)
        }

        std::thread::spawn(move || {
            let mut last = SystemTime::now();
            loop {
                if let Some((elapsed, next)) = last_modified(&path)
                    .and_then(|md| md.duration_since(last).ok().map(|e| (e, md)))
                {
                    if elapsed >= Duration::from_millis(100) {
                        last = next;
                        if tx.send(()).is_err() {
                            return;
                        }
                    }
                }
                std::thread::sleep(timeout);
            }
        })
    }
}
