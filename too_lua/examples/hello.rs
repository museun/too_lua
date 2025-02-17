use too::RunConfig;
use too_lua::{Bindings, Proxies};

fn main() -> std::io::Result<()> {
    too_lua::Application::new("./too_lua/examples/hello.lua")
        .reload_keybind('r')
        .with_bindings(Bindings::default_bindings())
        .with_proxies(Proxies::default_proxies())
        .config(RunConfig {
            debug: too::view::DebugMode::PerFrame,
            debug_anchor: too::layout::Anchor2::LEFT_TOP,
            ..RunConfig::default()
        })
        .watch_timeout(std::time::Duration::from_secs(1))
        .run()
}
