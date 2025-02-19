use mlua::AnyUserData;
use too::view::Ui;

use crate::{BindingSpec, BindingView, Context, Mapping};

//  #[derive(Debug, Clone)]
// struct RadioValue {
//     value: AnyUserData,
//     display: mlua::String,
// }

// impl FromLua for RadioValue {
//     fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
//         let mlua::Value::Table(table) = value else {
//             return Err(mlua::Error::runtime("expected table"));
//         };
//         Ok(Self {
//             value: table.get("value")?,
//             display: table.get("display")?,
//         })
//     }
// }

make_struct! {
    struct RadioParams is "RadioParams" {
        /// The current value -- this gets updated when the radio is changed
        current = mlua::Value ; "any"
        /// b
        choice = mlua::Value ; "any"
        /// a
        text = String ; "string"
    }
}

pub struct Radio;

impl BindingView for Radio {
    const SPEC: BindingSpec = binding! {
        /// A radio selection over multiple values
        "radio" => "RadioParams"
    };

    type Params = RadioParams;
    type Style = ();

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Some(table) = ctx.params::<mlua::Table>() else {
            return Mapping::report_missing_data(ui, ctx.id, "radio", "params");
        };

        let table = table.get::<mlua::Table>(1).unwrap();

        let mut current = table.get::<mlua::Value>("current").unwrap();
        let choice = table.get::<mlua::Value>("choice").unwrap();
        let text = table.get::<String>("text").unwrap();

        let old = current.clone();
        ui.radio(choice.clone(), &mut current, text);
        if current != old && current != choice {
            table.set("current", choice).unwrap();
        }
    }
}
