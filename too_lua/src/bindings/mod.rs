use crate::{
    binding::{Proxy, Register, Spec, View as _},
    Indirect,
};

#[doc(hidden)]
#[derive(Default, Debug)]
pub struct Bindings {
    pub(crate) bindings: Vec<(Spec, Indirect)>,
    pub(crate) proxies: Vec<Proxy>,
}

impl Bindings {
    const DEFAULT_TOO_PROXIES: &[fn() -> Proxy] = &[
        Value::proxy, //
        AlignedKind::proxy,
        Axis::proxy,
        Align::proxy,
        CrossAlign::proxy,
        Justify::proxy,
        BorderClass::proxy,
        BorderKind::proxy,
        ButtonClass::proxy,
        CheckboxClass::proxy,
        Constraint::proxy,
        LabelClass::proxy,
        ProgressClass::proxy,
        SelectedClass::proxy,
        SliderClass::proxy,
        TodoClass::proxy,
        ToggleSwitchClass::proxy,
    ];

    const DEFAULT_TOO_BINDINGS: &[(fn() -> Spec, Indirect)] = &[
        (Aligned::spec, Aligned::view),
        (Background::spec, Background::view),
        (Border::spec, Border::view),
        (Button::spec, Button::view),
        (Center::spec, Center::view),
        (Checkbox::spec, Checkbox::view),
        (Constrained::spec, Constrained::view),
        (Container::spec, Container::view),
        (ExpandAxis::spec, ExpandAxis::view),
        (Fill::spec, Fill::view),
        (Flex::spec, Flex::view),
        (Frame::spec, Frame::view),
        (Horizontal::spec, Horizontal::view),
        (Label::spec, Label::view),
        (Margin::spec, Margin::view),
        (Progress::spec, Progress::view),
        (Selected::spec, Selected::view),
        (Separator::spec, Separator::view),
        (Slider::spec, Slider::view),
        (TodoValue::spec, TodoValue::view),
        (Toggle::spec, Toggle::view),
        (ToggleSwitch::spec, ToggleSwitch::view),
        (Unconstrained::spec, Unconstrained::view),
        (Vertical::spec, Vertical::view),
    ];

    pub fn default_bindings() -> Self {
        Self::default()
            .with_many_spec(
                Self::DEFAULT_TOO_BINDINGS
                    .iter()
                    .copied()
                    .map(|(spec, view)| (spec(), view)),
            )
            .with_many_proxies(
                Self::DEFAULT_TOO_PROXIES
                    .iter()
                    .copied()
                    .map(|proxy| proxy()),
            )
    }

    pub fn with_many_spec(self, many: impl IntoIterator<Item = (Spec, Indirect)>) -> Self {
        many.into_iter()
            .fold(self, |this, (binding, func)| this.with_spec(binding, func))
    }

    pub fn with_spec(mut self, spec: Spec, value: Indirect) -> Self {
        // TODO rfind to de-dupe
        self.bindings.push((spec, value));
        self
    }

    pub fn with_many_proxies(self, many: impl IntoIterator<Item = Proxy>) -> Self {
        many.into_iter()
            .fold(self, |this, proxy| this.with_proxy(proxy))
    }

    pub fn with_proxy(mut self, proxy: Proxy) -> Self {
        self.proxies.push(proxy);
        self
    }
}

mod aligned;
pub use aligned::{Aligned, AlignedKind, AlignedParams};

mod background;
pub use background::{Background, BackgroundParams};

mod border;
pub use border::{Border, BorderClass, BorderKind, BorderParams, BorderStyle};

mod button;
pub use button::{Button, ButtonClass, ButtonParams, ButtonStyle};

mod center;
pub use center::Center;

mod checkbox;
pub use checkbox::{Checkbox, CheckboxClass, CheckboxParams, CheckboxStyle};

mod constrained;
pub use constrained::{Constrained, ConstrainedParams, Constraint, ConstraintKind};

mod container;
pub use container::Container;

mod expand_axis;
pub use expand_axis::ExpandAxis;

mod fill;
pub use fill::{Fill, FillParams};

mod flex;
pub use flex::{Flex, FlexParams};

mod frame;
pub use frame::{Frame, FrameParams};

mod horizontal;
pub use horizontal::Horizontal;

mod label;
pub use label::{Label, LabelClass, LabelParams, LabelStyle};

mod margin;
pub use margin::{Margin, MarginParams};

mod progress;
pub use progress::{Progress, ProgressClass, ProgressParams, ProgressStyle};

mod selected;
pub use selected::{Selected, SelectedClass, SelectedParams, SelectedStyle};

mod separator;
pub use separator::Separator;

mod slider;
pub use slider::{Slider, SliderClass, SliderParams, SliderStyle};

mod todo_value;
pub use todo_value::{TodoClass, TodoParams, TodoStyle, TodoValue};

mod toggle;
pub use toggle::{Toggle, ToggleParams};

mod toggle_switch;
pub use toggle_switch::{ToggleSwitch, ToggleSwitchClass, ToggleSwitchParams, ToggleSwitchStyle};

mod unconstrained;
pub use unconstrained::{Unconstrained, UnconstrainedParams};

mod vertical;
pub use vertical::Vertical;

mod list;
pub use list::ListParams;

mod align;
pub use align::Align;

mod axis;
pub use axis::Axis;

mod justify;
pub use justify::Justify;

mod cross_align;
pub use cross_align::CrossAlign;

mod color;
pub use color::Color;

mod value;
pub use value::Value;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, serde::Deserialize)]
#[serde(untagged)]
pub enum Either<Left, Right> {
    Left(Left),
    Right(Right),
}
