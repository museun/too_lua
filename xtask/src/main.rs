use std::io::Write as _;

use too_lua::{Bindings, Proxies};

fn main() -> std::io::Result<()> {
    let task = std::env::args().nth(1);
    match task.as_deref() {
        Some("generate") => {
            generate()?;
        }
        _ => eprintln!("{HELP}"),
    }

    Ok(())
}

static HELP: &str = "Tasks:
    generate        generates lua annotations";

fn generate() -> std::io::Result<()> {
    if std::fs::rename("_.lua", "_.lua.bak").is_ok() {
        eprintln!("renamed _.lua to _.lua.bak")
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("_.lua")?;

    eprintln!("creating a new default _.lua");

    let annotations = too_lua::generate(
        &Proxies::default_proxies(), //
        &Bindings::default_bindings(),
    );
    writeln!(&mut file, "{annotations}",)
}
