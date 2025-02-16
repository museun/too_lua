use std::io::Write as _;

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
    let _ = std::fs::rename("_.lua", "_.lua.bak");

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("_.lua")?;
    eprintln!("creating a new default _.lua");

    writeln!(&mut file, "{}", too_lua::params::generate())
}
