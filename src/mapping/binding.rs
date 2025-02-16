#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Binding {
    pub name: &'static str,
    pub doc: &'static str,
    pub fields: &'static [Field],
    pub args: Option<&'static str>,
}

impl Binding {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            doc: "",
            fields: &[],
            args: Some(name),
        }
    }

    pub const fn doc(mut self, doc: &'static str) -> Self {
        self.doc = doc;
        self
    }

    pub const fn fields(mut self, args: &'static [Field]) -> Self {
        self.fields = args;
        self
    }

    pub const fn args(mut self, args: &'static str) -> Self {
        self.args = if args.is_empty() { None } else { Some(args) };
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Field {
    pub name: &'static str,
    pub ty: &'static str,
    pub doc: &'static str,
}

impl Field {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            doc: "",
            ty: "",
        }
    }

    pub const fn ty(mut self, ty: &'static str) -> Self {
        self.ty = ty;
        self
    }

    pub const fn doc(mut self, doc: &'static str) -> Self {
        self.doc = doc;
        self
    }
}

macro_rules! binding {
    ($(
        #[doc = $doc:expr]
        $ident:ident is $name:expr => $args:tt {
            $(
                #[doc = $field_doc:expr]
                $field:tt $ty:expr
            )*
        }
    )*) => {
        $(
            #[doc = $doc]
            pub struct $ident;
            impl $ident {
                pub const BINDING: Binding =
                    Binding::new($name).doc($doc).args($args).fields(&[$(
                        Field::new(stringify!($field))
                            .ty($ty)
                            .doc($field_doc)
                    ),*]);
            }
        )*
    };

    ($(
        #[doc = $doc:expr]
        $ident:ident is $name:expr => {
            $(
                #[doc = $field_doc:expr]
                $field:tt $ty:expr
            )*
        }
    )*) => {
        $(
            #[doc = $doc]
            pub struct $ident;
            impl $ident {
                pub const BINDING: Binding =
                    Binding::new($name).doc($doc).args("").fields(&[$(
                        Field::new(stringify!($field))
                            .ty($ty)
                            .doc($field_doc)
                    ),*]);
            }
        )*
    };
}

binding! {
    /// Align its children at a specific anchor
    Aligned is "aligned" => "aligned" {
        /// Alignment for the children
        align "Aligned"
    }
}

binding! {
    /// Background of its children
    Background is "background" => "background" {
        /// The background color for the children
        background "string"
    }
}

binding! {
    /// Border to surround its children
    Border is "border" => "border" {
        /// The style of the border
        style "BorderStyle?"
        /// The class of the border
        class "Border?"
        /// The border to use
        border "BorderKind"
    }
}

binding! {
    /// A button to click
    Button is "button" => "button" {
        /// The style of the button
        style "ButtonStyle?"
        /// The class of the button
        class "Button?"
        /// The text of the button
        text "string"
        /// Function to call when the button is clicked
        handler "fun(): nil"
    }
}

binding! {
    /// Center a view in the current layout
    Center is "center" => "any" { }
}

binding! {
    /// A checkbox to toggle a boolean
    Checkbox is "checkbox" => "checkbox" {
        /// The style of the checkbox
        style "CheckboxStyle?"
        /// The class of the checkbox
        class "Checkbox?"
        /// The text of the checkbox
        text "string"
        /// The state of the checkbox, a boolean
        value "Value"
    }
}

binding! {
    /// Specifically constrain a view
    Constrained is "constrained" => "constrained" {
        /// The constraint to use
        constraint "Constraint"
    }
}

binding! {
    /// A container that just groups multiple calls into one parent
    Container is "container" => "any" { }
}

binding! {
    /// A view that expands the remainder of the space on the axis
    ExpandAxis is "expand_axis" => { }
}

binding! {
    /// Fill the childrens area, with an optional size constraint
    Fill is "fill" => "fill" {
        /// Use this color to fill the area
        background "string"
        /// Optional space to allocate
        space "{width: integer?, height: integer?}"
    }
}

binding! {
    /// Give a flex constraint to its children
    Flex is "flex" => "flex" {
        /// Tight constraint (ratio between 0.0 and 1.0)
        tight "number?"
        /// Loose constraint (ratio between 0.0 and 1.0)
        loose "number?"
    }
}

binding! {
    /// Frame is a border with a title
    Frame is "frame" => "frame" {
       /// The style of the frame
       style "BorderStyle?"
       /// The class of the frame
       class "Border?"
       /// The border to use
       border "BorderKind"
       /// Alignment for the title
       align "Align?"
       /// A string to place in the title
       title "string"
    }
}

binding! {
    /// Horizontal layout of children
    Horizontal is "horizontal" => "horizontal" {
        // TODO this can be styled
        /// Justification for children on the horizontal axis
        justify "Justify?"
        /// Alignment for children on the vertical axis
        cross_align "CrossAlign?"
        /// Gap between children
        gap "integer?"
        /// Should this be scrollable?
        scrollable "boolean?"
    }
}

binding! {
    /// Label displays some text
    Label is "label" => "string | label" {
        /// The style of the label
        style "LabelStyle?"
        /// The class of the label
        class "Label?"
        /// The text of the label
        text "string"
    }
}

binding! {
    /// Margin applies padding to a view
    Margin is "margin" => "margin" {
        /// Padding to the left of the view
        left "integer?"
        /// Padding to the right of the view
        right "integer?"
        /// Padding to the top of the view
        top "integer?"
        /// Padding to the bottom of the view
        bottom "integer?"
        /// Padding on both left and right of the view
        horizontal "integer?"
        /// Padding on both top and bottom of the view
        vertical "integer?"
        /// Padding on each side of the view
        all "integer?"
    }
}

binding! {
    /// A progress bar
    Progress is "progress" => "Value | progress" {
        /// The style of the progress bar
        style "ProgressStyle?"
        /// The class of the progress bar
        class "Progress?"
        /// Axis to use for layout
        axis "Axis?"
        /// The value to use (an f32 in the range of 0.0 ..= 1.0)
        value "Value"
    }
}

binding! {
    /// A selected boolean value
    Selected is "selected" => "selected" {
      /// The style of the selected value
      style "SelectedStyle?"
      /// The class of the selected value
      class "Selected?"
      /// The text of the selected value
      text "string"
      /// The state of the selected value, a boolean
      value "Value"
    }
}

binding! {
    /// Separator to divide some area
    Separator is "separator" => { }
}

binding! {
    /// Conditionally show or hide a view
    Toggle is "toggle" => "toggle" {
        /// The boolean state to use
        value "Value"
    }
}

binding! {
    /// A slider to adjust a value
    Slider is "slider" => "Value | slider" {
        /// The style of the slider
        style "SliderStyle?"
        /// The class of the slider
        class "Slider?"
        /// Axis to use for layout
        axis "Axis?"
        /// The value to use (an f32 in the range of 0.0 ..= 1.0)
        value "Value"
    }
}

binding! {
    /// A selected value
    TodoValue is "todo_value" => "todo_value" {
        /// The style of the selected value
        style "TodoStyle?"
        /// The class of the selected value
        class "Todo?"
        /// The text of the selected value
        text "string"
        /// The state of the selected value, a boolean
        value "Value"
    }
}

binding! {
    /// A switch that is toggled when clicked
    ToggleSwitch is "toggle_switch" => "toggle_switch" {
        /// The style of the selected value
        style "ToggleStyle?"
        /// The class of the selected value
        class "Toggle?"
        /// The state of the selected value, a boolean
        value "Value"
    }
}

binding! {
    /// Specifically unconstrained a view
    Unconstrained is "unconstrained" => "unconstrained" {
        /// Which axis to remove the constraints for
        constraint "{horizontal: boolean?, vertical: boolean?, both: boolean?}"
    }
}

binding! {
    /// Vertical layout of children
    Vertical is "vertical" => "vertical" {
      /// Justification for children on the vertical axis
      justify "Justify?"
      /// Alignment for children on the horizontal axis
      cross_align "CrossAlign?"
      /// Gap between children
      gap "integer?"
      /// Should this be scrollable?
      scrollable "boolean?"
    }
}
