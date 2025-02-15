use std::{path::PathBuf, time::Duration};

use mlua::{MaybeSend, UserData};
use too::{backend::Keybind, debug, view::Debug, RunConfig};

use crate::{
    Context, Errors, Mapping, Script, Tree, {Notification, Notifications},
};

pub struct Unit;
impl UserData for Unit {}

pub struct Application<T = Unit> {
    path: PathBuf,
    user_data: Option<T>,
    timeout: Option<Duration>,
    reload: Option<Keybind>,
    config: RunConfig,
}

impl Application<Unit> {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            user_data: None,
            timeout: None,
            reload: None,
            config: RunConfig::default(),
        }
    }

    pub fn user_data<T>(self, user_data: T) -> Application<T>
    where
        T: UserData + MaybeSend + 'static,
    {
        Application {
            user_data: Some(user_data),
            path: self.path,
            timeout: self.timeout,
            reload: self.reload,
            config: self.config,
        }
    }
}

impl<T> Application<T>
where
    T: UserData + MaybeSend + 'static,
{
    pub fn watch_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn reload_keybind(mut self, reload: impl Into<Keybind>) -> Self {
        self.reload = Some(reload.into());
        self
    }

    pub fn config(mut self, config: RunConfig) -> Self {
        self.config = config;
        self
    }

    pub fn run(self) -> std::io::Result<()> {
        let lua = mlua::Lua::new();
        lua.set_app_data(Tree::new(&lua).unwrap());

        if let Some(user_data) = self.user_data {
            lua.globals()
                .set("__USER_STATE", user_data)
                .expect("create user state")
        }

        lua.globals()
            .set(
                "debug",
                mlua::Function::wrap(|data: String| {
                    debug(data);
                    Ok(())
                }),
            )
            .unwrap();

        crate::setup_proxies(&lua).unwrap();

        // TODO make this fail less hard
        let mut script = match Script::new(self.path, self.timeout, &lua) {
            Ok(script) => script,
            Err(err) => {
                eprintln!("cannot load script: {err}");
                return Err(std::io::Error::other(err.to_string()));
            }
        };

        let mapping = Mapping::new();
        let debug_mode = self.config.debug;

        too::application2(
            self.config,
            (Errors::default(), Notifications::new()),
            |surface, palette, (errors, notifications)| {
                notifications.render(0, palette, surface);
                errors.render_errors(surface);
            },
            |ui, (errors, notifications)| {
                let mut was_manually_reloaded = false;

                if ui.key_pressed(Keybind::from_char('d')) {
                    Debug::clear();
                }

                if ui.key_pressed(Keybind::from_char('t')) {
                    let mode = if Debug::is_enabled() {
                        too::view::DebugMode::Off
                    } else {
                        debug_mode
                    };
                    Debug::set_debug_mode(mode);
                }

                if let Some(reload) = self.reload {
                    was_manually_reloaded ^= ui.key_pressed(reload);
                }

                if was_manually_reloaded || script.should_reload() {
                    Debug::clear();
                    if let Err(err) = script.reload(&lua) {
                        errors.handle_lua_error("cannot load", err);
                        return;
                    }

                    notifications.push(Notification::new(
                        "loaded new script",
                        Duration::from_secs(3),
                    ));
                }

                if let Err(err) = script.update(&lua) {
                    errors.handle_lua_error("cannot build ui", err);
                    return;
                }

                let tree = lua.app_data_ref::<Tree>().unwrap();

                let ctx = Context::new(
                    &lua, //
                    &tree,
                    &tree.map[tree.root],
                    tree.root,
                );
                mapping.evaluate(ui, ctx);
            },
        )
    }
}
