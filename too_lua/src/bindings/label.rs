use too::view::{Style, Ui, ViewExt as _};

use crate::{
    bindings::Color,
    mapping::{BindingSpec, BindingView},
    Context, LuaType, Mapping,
};

make_class! {
    class LabelClass is "Label" ; too::views::LabelStyle {
        /// The default style
        Default = "default" ; too::views::LabelStyle::default
        /// Denotes this label is for information
        Info    = "info"    ; too::views::LabelStyle::info
        /// Denotes this label is for warning
        Warning = "warning" ; too::views::LabelStyle::warning
        /// Denotes this label is for success
        Success = "success" ; too::views::LabelStyle::success
        /// Denotes this label is for danger
        Danger  = "danger"  ; too::views::LabelStyle::danger
    }
}

make_style! {
    manual style LabelStyle is "LabelStyle" ; too::views::LabelStyle {
        /// The foreground text color
        foreground = Option<Color> ; "Color?"
        /// The text should be italic
        italic     = Option<bool>  ; "boolean?"
        /// The text should be bold
        bold       = Option<bool>  ; "boolean?"
        /// The text should be underline
        underline  = Option<bool>  ; "boolean?"
        /// The text should be faint
        faint      = Option<bool>  ; "boolean?"
        /// The text should be blink
        blink      = Option<bool>  ; "boolean?"
        /// The text should be strikeout
        strikeout  = Option<bool>  ; "boolean?"
    }
}

make_struct! {
    struct LabelParams is "LabelParams" {
        /// The style of the label
        style = Option<LabelStyle> ; "LabelStyle?"
        /// The class of the label
        class = Option<LabelClass> ; "Label?"
        /// The text of the label
        text = String              ; "string"
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Label;

impl BindingView for Label {
    const SPEC: BindingSpec = binding! {
        /// Label displays some text
        "label" => LabelParams::NAME
    };

    type Params = LabelParams;
    type Style = LabelStyle;

    fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        use too::views::{label, Label};

        type Apply = fn(Label) -> Label;
        type Class = fn(&too::view::Palette, too::view::StyleOptions) -> too::views::LabelStyle;

        if let Some(text) = ctx.text_ref() {
            ui.show(label(text));
            return;
        }

        let Some(Ok(text)) = ctx.params_field::<Box<str>>("text") else {
            return Mapping::report_missing_data(ui, ctx.id, "label", "text");
        };

        let Ok(params) = ctx.params::<LabelParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "label", "params");
        };

        let mut label = label(text);

        let mut fg = None;
        if let Some(style) = params.style {
            if let Some(Color(new)) = style.foreground {
                fg = Some(new)
            }

            label = [
                style.italic,
                style.bold,
                style.underline,
                style.faint,
                style.blink,
                style.strikeout,
            ]
            .into_iter()
            .map(|c| c.unwrap_or_default())
            .zip([
                Label::italic as Apply,
                Label::bold,
                Label::underline,
                Label::faint,
                Label::blink,
                Label::strikeout,
            ])
            .filter_map(|(v, a)| v.then_some(a))
            .fold(label, |l, a| a(l))
        }

        let mut class: Class = <too::views::LabelStyle as too::view::Style>::default;

        if let Some(params) = params.class {
            match params {
                LabelClass::Info => class = too::views::LabelStyle::info,
                LabelClass::Warning => class = too::views::LabelStyle::warning,
                LabelClass::Success => class = too::views::LabelStyle::success,
                LabelClass::Danger => class = too::views::LabelStyle::danger,
                _ => {}
            };
        }

        let label = if let Some(fg) = fg {
            label.fg(fg)
        } else {
            label.class(class)
        };

        ui.show(label);
    }
}
