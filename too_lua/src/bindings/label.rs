use too::view::{Ui, ViewExt as _};

use crate::{
    bindings::Color,
    mapping::{Binding, Field},
    Context, Mapping,
};

crate::make_proxy! {
    LabelParams {
        class:
        LabelClass is "Label" {
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

        manual style:
        LabelStyle => too::views::LabelStyle {
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
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Label;

impl Label {
    binding! {
        /// Label displays some text
        "label" => "string | label" {
            /// The style of the label
            style "LabelStyle?"
            /// The class of the label
            class "Label?"
            /// The text of the label
            text "string"
        }
    }

    #[profiling::function]
    pub fn view(_mapping: &Mapping, ui: &Ui, ctx: Context) {
        {
            profiling::scope!("label text only");
            if let Some(text) = ctx.text_ref() {
                ui.label(text);
                return;
            }
        }
        let Some(Ok(text)) = ctx.params_field::<String>("text") else {
            return Mapping::report_missing_data(ui, ctx.id, "label", "text");
        };

        let Ok(params) = ctx.params::<LabelParams>() else {
            return Mapping::report_missing_data(ui, ctx.id, "label", "params");
        };

        use too::views::Label as L;
        type Apply = fn(L) -> L;

        let mut label = {
            profiling::scope!("make label");
            too::views::label(text)
        };

        let mut fg = None;
        if let Some(style) = params.style {
            profiling::scope!("label style");
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
                L::italic as Apply,
                L::bold,
                L::underline,
                L::faint,
                L::blink,
                L::strikeout,
            ])
            .filter_map(|(v, a)| v.then_some(a))
            .fold(label, |l, a| a(l))
        }

        let class = params
            .class
            .and_then(|class| {
                profiling::scope!("label class");
                let val = match class {
                    LabelClass::Info => too::views::LabelStyle::info,
                    LabelClass::Warning => too::views::LabelStyle::warning,
                    LabelClass::Success => too::views::LabelStyle::success,
                    LabelClass::Danger => too::views::LabelStyle::danger,
                    _ => return None,
                };
                Some(val)
            })
            .unwrap_or(<too::views::LabelStyle as too::view::Style>::default);

        let label = if let Some(fg) = fg {
            label.fg(fg)
        } else {
            label.class(class)
        };

        profiling::scope!("show label");
        ui.show(label);
    }
}
