pub fn get_table<T>(
    value: mlua::Value,
    extract: impl FnOnce(mlua::Table) -> mlua::Result<T>,
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
