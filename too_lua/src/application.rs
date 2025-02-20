use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use mlua::{MaybeSend, UserData};
use too::{
    backend::{Backend as _, EventReader, Keybind},
    format_str,
    renderer::Surface,
    term::{Config as TermConfig, Term},
    view::{CroppedSurface, Debug, State},
    RunConfig,
};

use crate::{
    runtime::{RunningTasks, Runtime},
    Bindings, Context, Errors, Mapping, Notification, Notifications, Script, Tree,
};

pub struct Unit;
impl UserData for Unit {}

pub struct Application<T = Unit> {
    path: PathBuf,
    user_data: Option<T>,
    timeout: Option<Duration>,
    reload: Option<Keybind>,
    config: RunConfig,
    bindings: Bindings,
}

impl Application<Unit> {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            user_data: None,
            timeout: None,
            reload: None,
            config: RunConfig::default(),
            bindings: Bindings::default(),
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
            bindings: self.bindings,
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

    pub fn with_bindings(mut self, bindings: Bindings) -> Self {
        self.bindings = bindings;
        self
    }

    pub async fn run(self) -> std::io::Result<()> {
        tokio::task::spawn_blocking(move || self.run_inner())
            .await
            .unwrap()
    }

    fn run_inner(self) -> std::io::Result<()> {
        let lua = init_lua(&self.bindings).map_err(std::io::Error::other)?;
        if let Some(user_data) = self.user_data {
            lua.globals()
                .set("__USER_STATE", user_data)
                .expect("create user state")
        }

        // TODO make this fail less hard
        let mut script = match Script::new(self.path, self.timeout, &lua) {
            Ok(script) => script,
            Err(err) => {
                eprintln!("cannot load script: {err}");
                return Err(std::io::Error::other(err.to_string()));
            }
        };

        if let Err(err) = script.update(&lua) {
            eprintln!("cannot evaluate script: {err}");
            return Err(std::io::Error::other(err.to_string()));
        }

        let mapping = Mapping::from_bindings(self.bindings);

        let mut errors = Errors::default();
        let mut notifications = Notifications::default();

        let mut term = Term::setup(
            TermConfig::default()
                .hook_panics(self.config.hook_panics)
                .ctrl_c_quits(self.config.ctrl_c_quits)
                .ctrl_z_switches(self.config.ctrl_z_switches),
        )?;
        let mut surface = Surface::new(term.size());

        let mut state = State::new(self.config.palette, self.config.animation);
        Debug::set_debug_mode(self.config.debug);
        Debug::set_debug_anchor(self.config.debug_anchor);

        let fps = self.config.fps.max(1.0);
        let target = Duration::from_secs_f32(1.0 / fps);
        let max_budget = (target / 2).max(Duration::from_millis(1));

        let mut should_render = false;
        let mut last_resize = None;

        run_loop(fps, |_fr, dt| {
            state.update(dt);

            let mut was_manually_reloaded = false;
            let start = Instant::now();
            while let Some(ev) = term.try_read_event() {
                if ev.is_quit() {
                    return Ok(false);
                }

                if start.elapsed() >= max_budget {
                    break;
                }

                if let Some(reload) = self.reload {
                    was_manually_reloaded ^= ev.is_keybind_pressed(reload);
                }

                if let too::backend::Event::Resize(size) = ev {
                    last_resize = Some(size);
                    continue;
                }

                surface.update(&ev);
                state.event(&ev);
                should_render = true;
            }

            if was_manually_reloaded || script.should_reload() {
                Debug::clear();
                profiling::scope!("reload script");

                if let Err(err) = script.reload(&lua) {
                    errors.handle_lua_error("cannot load", err);
                    return Ok(true);
                }

                // this can be blocking for a very long time
                if let Err(err) = script.update(&lua) {
                    errors.handle_lua_error("cannot evaluate", err);
                    return Ok(true);
                }

                notifications.push(Notification::new(
                    "loaded new script",
                    Duration::from_secs(3),
                ));
            }

            if let Some(size) = last_resize {
                let ev = too::backend::Event::Resize(size);
                surface.update(&ev);
                state.event(&ev);
                should_render = true;
            }

            let _ = lua.app_data_mut::<Tree>().unwrap().evaluate_lazies();

            state.build(surface.rect(), |ui| {
                let tree = lua.app_data_ref::<Tree>().unwrap();
                let ctx = Context::new(
                    &lua, //
                    &tree,
                    &tree.map[tree.root],
                    tree.root,
                );
                profiling::scope!("evaluate tree");
                mapping.evaluate(ui, ctx);
            });

            state.render(&mut CroppedSurface::new(surface.rect(), &mut surface));

            notifications.render(0, &state.palette(), &mut surface);
            errors.render(&mut surface);

            surface.render(&mut term.writer())?;

            Ok(true)
        })
    }
}

pub(crate) fn init_lua(bindings: &Bindings) -> mlua::Result<mlua::Lua> {
    let lua = mlua::Lua::new();
    lua.set_app_data(Tree::new(&lua)?);
    lua.set_app_data(RunningTasks::default());

    let globals = lua.globals();

    globals.set("lazy", lua.create_function(lazy)?)?;
    globals.set("debug", mlua::Function::wrap(debug))?;
    globals.set("rt", lua.create_proxy::<Runtime>()?)?;

    hook_require(&lua)?;

    for proxy in &bindings.proxies {
        (proxy.register)(&globals, &lua)?;
    }

    Ok(lua)
}

fn debug(data: mlua::Value) -> mlua::Result<()> {
    too::debug(format_str!("{data:?}"));
    Ok(())
}

fn lazy(lua: &mlua::Lua, table: mlua::Table) -> mlua::Result<()> {
    let lazy = table.get::<mlua::Function>(1)?;
    lua.app_data_mut::<Tree>().unwrap().add_lazy(lazy);
    Ok(())
}

fn hook_require(lua: &mlua::Lua) -> mlua::Result<()> {
    let globals = lua.globals();

    let require = globals.get::<mlua::Function>("require")?;
    let loaded = lua.create_table()?;
    globals.set("__TOO_LOADED", loaded)?;
    let require = lua.create_function(move |lua, name: mlua::String| {
        lua.globals()
            .get::<mlua::Table>("__TOO_LOADED")?
            .set(&name, true)?;
        require.call::<mlua::Value>(name)
    })?;
    globals.set("require", require)
}

fn run_loop<E>(target: f32, mut frame: impl FnMut(u64, f32) -> Result<bool, E>) -> Result<(), E> {
    const EMA_ALPHA: f32 = 0.1;
    let mut ema_avg = 1.0 / target;
    let update = |value, avg| EMA_ALPHA * value + (1.0 - EMA_ALPHA) * avg;

    let mut fr = 0;
    let mut prev = Instant::now();
    let mut next = prev;

    loop {
        profiling::finish_frame!();

        let now = Instant::now();
        let dt = (now - prev).as_secs_f32();
        prev = now;

        ema_avg = update(dt, ema_avg);
        next += Duration::from_secs_f32(ema_avg);
        if !frame(fr, dt)? {
            return Ok(());
        }

        let sleep = next.saturating_duration_since(Instant::now());
        if !sleep.is_zero() {
            std::thread::park_timeout(sleep);
        } else {
            next = Instant::now();
        }
        fr += 1;
    }
}
