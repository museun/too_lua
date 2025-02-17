use crate::{Binding, Indirect};

#[doc(hidden)]
#[derive(Default)]
pub struct Bindings {
    bindings: Vec<(Binding, Indirect)>,
}

impl<'a> IntoIterator for &'a Bindings {
    type Item = &'a (Binding, Indirect);
    type IntoIter = std::slice::Iter<'a, (Binding, Indirect)>;
    fn into_iter(self) -> Self::IntoIter {
        self.bindings.iter()
    }
}

impl Bindings {
    const DEFAULT_TOO_BINDINGS: &[(Binding, Indirect)] = &[
        (Aligned::binding(), Aligned::view),
        (Background::binding(), Background::view),
        (Border::binding(), Border::view),
        (Button::binding(), Button::view),
        (Center::binding(), Center::view),
        (Checkbox::binding(), Checkbox::view),
        (Constrained::binding(), Constrained::view),
        (Container::binding(), Container::view),
        (ExpandAxis::binding(), ExpandAxis::view),
        (Fill::binding(), Fill::view),
        (Flex::binding(), Flex::view),
        (Frame::binding(), Frame::view),
        (Horizontal::binding(), Horizontal::view),
        (Label::binding(), Label::view),
        (Margin::binding(), Margin::view),
        (Progress::binding(), Progress::view),
        (Selected::binding(), Selected::view),
        (Separator::binding(), Separator::view),
        (Slider::binding(), Slider::view),
        (TodoValue::binding(), TodoValue::view),
        (Toggle::binding(), Toggle::view),
        (ToggleSwitch::binding(), ToggleSwitch::view),
        (Unconstrained::binding(), Unconstrained::view),
        (Vertical::binding(), Vertical::view),
    ];

    pub fn default_bindings() -> Self {
        Self::default().with_many(Self::DEFAULT_TOO_BINDINGS.iter().copied())
    }

    pub fn with_many(self, many: impl IntoIterator<Item = (Binding, Indirect)>) -> Self {
        many.into_iter()
            .fold(self, |this, (binding, func)| this.with(binding, func))
    }

    pub fn with(mut self, binding: Binding, value: Indirect) -> Self {
        // TODO rfind to de-dupe
        self.bindings.push((binding, value));
        self
    }
}

mod aligned;
pub use aligned::{Aligned, AlignedParams};

mod background;
pub use background::Background;

mod border;
pub use border::{Border, BorderClass, BorderKind, BorderParams, BorderStyle};

mod button;
pub use button::{Button, ButtonClass, ButtonParams, ButtonStyle};

mod center;
pub use center::Center;

mod checkbox;
pub use checkbox::{Checkbox, CheckboxClass, CheckboxParams, CheckboxStyle};

mod constrained;
pub use constrained::{Constrained, Constraint, ConstraintKind};

mod container;
pub use container::Container;

mod expand_axis;
pub use expand_axis::ExpandAxis;

mod fill;
pub use fill::Fill;

mod flex;
pub use flex::Flex;

mod frame;
pub use frame::Frame;

mod horizontal;
pub use horizontal::Horizontal;

mod label;
pub use label::{Label, LabelClass, LabelParams, LabelStyle};

mod margin;
pub use margin::Margin;

mod progress;
pub use progress::{Progress, ProgressClass, ProgressParams, ProgressStyle};

mod selected;
pub use selected::{Selected, SelectedClass, SelectedParams, SelectedStyle};

mod separator;
pub use separator::Separator;

mod toggle;
pub use toggle::Toggle;

mod slider;
pub use slider::{Slider, SliderClass, SliderParams, SliderStyle};

mod todo_value;
pub use todo_value::{TodoClass, TodoParams, TodoStyle, TodoValue};

mod toggle_switch;
pub use toggle_switch::{ToggleClass, ToggleParams, ToggleStyle, ToggleSwitch};

mod unconstrained;
pub use unconstrained::Unconstrained;

mod vertical;
pub use vertical::Vertical;

mod list;

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
