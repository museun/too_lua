use std::collections::HashSet;

use crate::Bindings;

pub fn generate(bindings: &Bindings) -> String {
    use std::io::Write as _;

    let mut seen = HashSet::new();

    let mut out = Vec::new();
    _ = writeln!(
        &mut out,
        "---@alias Color string #RGB | #RGBA | #RRGGBB | #RRGGBBAA hex string"
    );
    _ = writeln!(&mut out);

    for proxy in &bindings.proxies {
        _ = anno_lua::generate_type(&mut out, &((proxy.ty)()));
    }

    for (spec, _) in &bindings.bindings {
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
    }

    _ = writeln!(&mut out, "---@class ui");
    for (spec, _) in &bindings.bindings {
        for doc in spec.docs {
            _ = writeln!(&mut out, "--- {doc}");
        }
        _ = write!(&mut out, "---@field {name} fun(", name = spec.name);
        match spec.args {
            crate::Arguments::Any => _ = write!(&mut out, "args: any"),
            crate::Arguments::None => {}
            crate::Arguments::Named(name) => _ = write!(&mut out, "args: {name}"),
        }
        _ = writeln!(&mut out, "): nil")
    }
    _ = writeln!(&mut out, "ui = {{ }}");

    String::from_utf8(out).unwrap()
}
