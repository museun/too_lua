use mlua::AnyUserData;
use too::view::{Style as _, Ui, ViewExt as _};

use crate::{
    bindings::Value,
    mapping::{BindingSpec, BindingView},
    Context, LuaType, Mapping,
};

use super::Color;

make_class! {
    class TodoClass is "Todo" ; too::views::TodoStyle {
        /// The default style
        Default  = "default" ; too::views::TodoStyle::default
    }
}

make_style! {
    manual style TodoStyle is "TodoStyle" ; too::views::TodoStyle {
        /// When selected, the text should be bold
        bold          = Option<bool> ; "boolean?"
        /// When selected, the text should be faint
        faint         = Option<bool> ; "boolean?"
        /// When selected, the text should be italic
        italic        = Option<bool> ; "boolean?"
        /// When selected, the text should be underline
        underline     = Option<bool> ; "boolean?"
        /// When selected, the text should be blink
        blink         = Option<bool> ; "boolean?"
        /// When selected, the text should be reverse
        reverse       = Option<bool> ; "boolean?"
        /// When selected, the text should be strikeout
        strikeout     = Option<bool> ; "boolean?"
        //
        /// The color of the text
        text_color    = Option<Color> ; "Color?"
        /// The color of the text, when hovered
        hovered_color = Option<Color> ; "Color?"
    }
}

make_struct! {
    struct TodoParams is "TodoParams" {
        /// The class of the selected value
        class = Option<TodoClass> ; "Todo?"
        /// The style of the selected value
        style = Option<TodoStyle> ; "TodoStyle?"
        /// The text of the selected value
        text = String             ; "string"
        /// The state of the selected value, a boolean
        value = AnyUserData       ; "Value"
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TodoValue;

impl BindingView for TodoValue {
    const SPEC: BindingSpec = binding! {
        /// A selected value
        "todo_value" => TodoParams::NAME
    };

    type Params = TodoParams;
    type Style = TodoStyle;

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        let Ok(params) = ctx.params::<TodoParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "todo", "params");
        };

        let Some(Ok(text)) = ctx.params_field::<String>("text") else {
            return Mapping::report_missing_data(ui, ctx.id, "todo", "text");
        };

        let Some(mut value) = ctx.value_mut() else {
            return Mapping::report_missing_data(ui, ctx.id, "todo", "value");
        };

        let Value::Bool(value) = &mut *value else {
            return Mapping::report_missing(ui, ctx.id, "bool value");
        };

        let default = <too::views::TodoStyle as too::view::Style>::default;

        let view = too::views::todo_value(value, text);
        let class = params
            .class
            .and_then(|class| {
                #[allow(unreachable_patterns)]
                let val = match class {
                    TodoClass::Default => default,
                    _ => return None,
                };
                Some(val)
            })
            .unwrap_or(default);

        let mut attr = None;
        let mut text_color = None;
        let mut hovered_color = None;

        use too::renderer::Attribute;
        if let Some(style) = params.style {
            let new = [
                (style.italic, Attribute::ITALIC),
                (style.bold, Attribute::BOLD),
                (style.underline, Attribute::UNDERLINE),
                (style.faint, Attribute::FAINT),
                (style.blink, Attribute::BLINK),
                (style.strikeout, Attribute::STRIKEOUT),
            ]
            .into_iter()
            .filter_map(|(c, a)| c.unwrap_or_default().then_some(a))
            .fold(Attribute::RESET, |l, a| l | a);

            attr = Some(new).filter(|c| !c.is_reset());
            text_color = style.text_color.map(|c| c.0);
            hovered_color = style.hovered_color.map(|c| c.0);
        }

        ui.show(view.class(move |palette, options| {
            let mut style = class(palette, options);
            if let Some(attr) = attr {
                style.selected = attr;
            }
            if let Some(text_color) = text_color {
                style.text_color = text_color;
            }
            if let Some(hovered_color) = hovered_color {
                style.hovered_color = Some(hovered_color);
            }
            style
        }));
    }
}
