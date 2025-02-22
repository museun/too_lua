pub fn expect_table<T>(
    value: &mlua::Value,
    extract: impl FnOnce(&mlua::Table) -> mlua::Result<T>,
) -> mlua::Result<T> {
    let mlua::Value::Table(table) = value else {
        return Err(mlua::Error::runtime(format!(
            "expected {}, got: {}",
            too::helpers::short_name(std::any::type_name::<T>()),
            value.type_name()
        )));
    };
    extract(table)
}

pub fn expect_string(value: &mlua::Value) -> mlua::Result<mlua::String> {
    value.as_string().cloned().ok_or_else(|| {
        mlua::Error::runtime(format!("expected a string, got: {}", value.type_name()))
    })
}
