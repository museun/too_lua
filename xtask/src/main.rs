use std::borrow::Cow;

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

fn generate(path: &str) -> std::io::Result<()> {
    too_lua::write_annotations(path, &Bindings::default_bindings())
}
