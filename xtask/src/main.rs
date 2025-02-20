use std::{borrow::Cow, io::Write as _};

use too_lua::Bindings;

fn main() -> std::io::Result<()> {
    let task = std::env::args().nth(1);
    let path = std::env::args()
        .nth(2)
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("_.lua"));

    match task.as_deref() {
        Some("generate") => {
            generate(&path)?;
        }
        _ => eprintln!("{HELP}"),
    }

    Ok(())
}

static HELP: &str = "Tasks:
    generate <file_name?>   generates lua annotations";

// TODO use an actual path here
fn generate(path: &str) -> std::io::Result<()> {
    if std::fs::rename(path, format!("{path}.bak")).is_ok() {
        eprintln!("renamed {path} to {path}.bak")
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    eprintln!("creating a new default {path}");

    let annotations = too_lua::generate(&Bindings::default_bindings());
    writeln!(&mut file, "{annotations}",)
}
