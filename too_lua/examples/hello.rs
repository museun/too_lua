use too::RunConfig;
use too_lua::{Bindings, Proxies};

fn main() -> std::io::Result<()> {
    // let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
    // let _puffin_server = puffin_http::Server::new(&server_addr).unwrap();
    // profiling::puffin::set_scopes_on(true);

    too_lua::Application::new("./too_lua/examples/hello.lua")
        .reload_keybind('r')
        .with_bindings(Bindings::default_bindings())
        .with_proxies(Proxies::default_proxies())
        .config(RunConfig {
            debug: too::view::DebugMode::Rolling,
            debug_anchor: too::layout::Anchor2::LEFT_TOP,
            ..RunConfig::default()
        })
        .watch_timeout(std::time::Duration::from_secs(1))
        .run()
}
