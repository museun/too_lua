use std::future::Future;

use too_lua::Bindings;

fn start<R>(f: impl Future<Output = R> + Send + Sync + 'static) -> R {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(f)
}

fn main() -> std::io::Result<()> {
    start(async move {
        let puffin = tokio::task::spawn_blocking(move || {
            let addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
            let server = puffin_http::Server::new(&addr).unwrap();
            profiling::puffin::set_scopes_on(true);
            server
        });

        let app = too_lua::Application::new("./too_lua/examples/hello.lua")
            .reload_keybind('r')
            .with_bindings(Bindings::default_bindings())
            .config(too::RunConfig {
                debug: too::view::DebugMode::Rolling,
                debug_anchor: too::layout::Anchor2::LEFT_TOP,
                ..too::RunConfig::default()
            })
            .watch_timeout(std::time::Duration::from_secs(1))
            .run();

        let _ = futures_util::join!(puffin, app);
        Ok(())
    })
}
