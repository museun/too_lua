use too::RunConfig;

fn main() -> std::io::Result<()> {
    too_lua::Application::new("./too_lua/examples/hello.lua")
        .reload_keybind('r')
        .config(RunConfig {
            debug: too::view::DebugMode::PerFrame,
            ..RunConfig::default()
        })
        .watch_timeout(std::time::Duration::from_secs(1))
        .run()
}
