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
        too_lua::Application::new("./too_lua/examples/hello.lua")
            .reload_keybind('r')
            .with_bindings(Bindings::default_bindings())
            .config(too::RunConfig {
                debug: too::view::DebugMode::Rolling,
                debug_anchor: too::layout::Anchor2::LEFT_TOP,
                ..too::RunConfig::default()
            })
            .watch_timeout(std::time::Duration::from_secs(1))
            .run()
            .await
    })
}
