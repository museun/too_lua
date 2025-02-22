use std::{collections::HashSet, path::Path};

use crate::{Arguments, Bindings};

pub fn write_annotations(file: impl AsRef<Path>, bindings: &Bindings) -> std::io::Result<()> {
    use std::io::Write as _;

    let path = file.as_ref();

    let bak = format!("{path}.bak", path = path.display());
    if std::fs::rename(path, &bak).is_ok() {
        eprintln!("renamed {path} to {bak}", path = path.display())
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    eprintln!(
        "creating an annotations file: {path}",
        path = path.display()
    );

    let annotations = self::generate(bindings);
    writeln!(&mut file, "{annotations}")
}

pub fn generate(bindings: &Bindings) -> String {
    use std::io::Write as _;

    let mut seen = HashSet::new();
    let mut out = Vec::new();

    _ = writeln!(
        &mut out,
        "--- lazily evaluates a function\n\
        ---@generic T\n\
        ---@param args table<fun(): T>\n\
        ---@return T\n\
        ---@diagnostic disable-next-line: lowercase-global, missing-return\n\
        function lazy(args) end"
    );
    _ = writeln!(&mut out);

    // we can't sort the proxies because their lexical order is important
    for proxy in &bindings.proxies {
        _ = anno_lua::generate_type(&mut out, &(proxy.ty)());
    }

    let mut bindings = bindings.bindings.clone();
    bindings.sort_by_cached_key(|(spec, _)| spec.name);
    bindings.dedup_by_key(|(spec, _)| spec.name);

    for (spec, _) in &bindings {
        if let Some(params) = spec.params() {
            if seen.insert(params) {
                _ = anno_lua::generate_type(&mut out, &params);
            }
        }

        if let Some(style) = spec.style() {
            if seen.insert(style) {
                _ = anno_lua::generate_type(&mut out, &style);
            }
        }

        for associated in spec.associated {
            _ = anno_lua::generate_type(&mut out, &associated());
        }

        for proxy in spec.proxies {
            _ = anno_lua::generate_type(&mut out, &(proxy.ty)());
        }
    }

    _ = writeln!(&mut out, "---@class ui");

    for (spec, _) in &bindings {
        for doc in spec.docs {
            _ = writeln!(&mut out, "--- {doc}");
        }
        _ = write!(&mut out, "---@field {name} fun(", name = spec.name);
        match spec.args {
            Arguments::Any => _ = write!(&mut out, "args: any"),
            Arguments::Named(name) => _ = write!(&mut out, "args: {name}"),
            Arguments::None => {}
        }
        _ = writeln!(&mut out, "): nil")
    }
    _ = writeln!(&mut out, "ui = {{ }}");

    String::from_utf8(out).unwrap()
}
