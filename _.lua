---@alias Color string #RGB | #RGBA | #RRGGBB | #RRGGBBAA hex string

--- A shared value between lua and rust
---@class (exact) Value
--- create a new value
---@field new fun(value: integer|number|boolean|string): Value
--- create a new value, persisted and accessible via `id`
---@field persist fun(id: string, value: integer|number|boolean|string): Value
--- destroys a persisted value `id`, if it exists
---@field destroy fun(id: string): boolean
--- get the inner value
---@field value integer|number|boolean|string
Value = { }

---@enum Aligned
Aligned = {
    --- Align to the horizontal left and vertical top
    left_top = Aligned,
    --- Align to the horizontal center and vertical top
    center_top = Aligned,
    --- Align to the horizontal right and vertical top
    right_top = Aligned,
    --- Align to the horizontal left and vertical center
    left_center = Aligned,
    --- Align to the horizontal center and vertical center
    center = Aligned,
    --- Align to the horizontal right and vertical center
    right_center = Aligned,
    --- Align to the horizontal left and vertical bottom
    left_bottom = Aligned,
    --- Align to the horizontal center and vertical bottom
    center_bottom = Aligned,
    --- Align to the horizontal right and vertical bottom
    right_bottom = Aligned,
}

---@enum Axis
Axis = {
    --- The vertical axis
    vertical = Axis,
    --- The horizontal axis
    horizontal = Axis,
}

---@enum Align
Align = {
    --- Align to the start of the area
    min = Align,
    --- Align to the middle of the area
    middle = Align,
    --- Align to the end of the area
    max = Align,
}

---@enum CrossAlign
CrossAlign = {
    --- Alignment begins at the 'start' of the area
    min = CrossAlign,
    --- Alignment begins at the 'end' of the area
    max = CrossAlign,
    --- Alignment is in the middle, extra space is applied before and after
    center = CrossAlign,
    --- The elements stretch to fill the area
    stretch = CrossAlign,
    --- The elements fill the entire area
    fill = CrossAlign,
}

---@enum Justify
Justify = {
    --- The extra space is applied to the end
    min = Justify,
    --- The extra space is applied to the start
    max = Justify,
    --- The extra space is applied to the start and end
    center = Justify,
    --- Place the space between the elements
    space_between = Justify,
    --- Place the space around the elements
    space_around = Justify,
    --- Evenly space the elements
    space_evenly = Justify,
}

---@enum Border
Border = {
    --- The default style
    default = Border,
    --- An interactive style
    interactive = Border,
}

---@enum BorderKind
BorderKind = {
    --- No border
    empty = BorderKind,
    --- A thin border
    thin = BorderKind,
    --- A thin, but wide border
    thin_wide = BorderKind,
    --- A rounded border
    rounded = BorderKind,
    --- A double-line border
    double = BorderKind,
    --- A thick border
    thick = BorderKind,
    --- A thick, but tall border
    thick_tall = BorderKind,
    --- A thick, but wide border
    thick_wide = BorderKind,
}

---@enum Button
Button = {
    default = Button,
    --- Denotes this button is for success
    success = Button,
    --- Denotes this button is for information
    info = Button,
    --- Denotes this button is for warning
    warning = Button,
    --- Denotes this button is for danger
    danger = Button,
}

---@enum Checkbox
Checkbox = {
    --- The default style
    default = Checkbox,
    --- A markdown inspired style
    markdown = Checkbox,
    --- An ascii checkbox style
    ascii = Checkbox,
}

--- A constraint
---@class (exact) Constraint
--- The view has an exact size
---@field exact_size fun(w: integer, h: integer): Constraint
--- The view has an exact height
---@field exact_height fun(h: integer): Constraint
--- The view has an exact width
---@field exact_width fun(w: integer): Constraint
--- The view has a max size
---@field max_size fun(w: integer, h: integer): Constraint
--- The view has a max height
---@field max_height fun(h: integer): Constraint
--- The view has a max width
---@field max_width fun(w: integer): Constraint
--- The view has a min size
---@field min_size fun(w: integer, h: integer): Constraint
--- The view has a min width
---@field min_width fun(w: integer): Constraint
--- The view has a min height
---@field min_height fun(h: integer): Constraint
Constraint = { }

---@enum Label
Label = {
    --- The default style
    default = Label,
    --- Denotes this label is for information
    info = Label,
    --- Denotes this label is for warning
    warning = Label,
    --- Denotes this label is for success
    success = Label,
    --- Denotes this label is for danger
    danger = Label,
}

---@enum Progress
Progress = {
    --- Default style
    default = Progress,
    --- A medium filled style
    medium_filled = Progress,
    --- A full filled style
    filled = Progress,
    --- A thin style
    thin = Progress,
    --- A thick style
    thick = Progress,
    --- A thin, but dashed style
    thin_dashed = Progress,
    --- A thick, but dashed style
    thick_dashed = Progress,
}

---@enum Selected
Selected = {
    --- The default style
    default = Selected,
    --- This element reacts to hovers
    hovered = Selected,
}

---@enum Slider
Slider = {
    --- The default style
    default = Slider,
    --- Small track and rounded knob
    small_rounded = Slider,
    --- Small track and diamond knob
    small_diamond = Slider,
    --- Small track and square knob
    small_square = Slider,
    --- Medium track and large knob
    large = Slider,
    --- Large track and large knob
    large_filled = Slider,
}

---@enum Todo
Todo = {
    --- The default style
    default = Todo,
}

---@enum Toggle
Toggle = {
    --- The default style
    default = Toggle,
    --- A large knob
    large = Toggle,
    --- A small rounded knob
    small_rounded = Toggle,
    --- A small diamond knob
    small_diamond = Toggle,
    --- A small square knob
    small_square = Toggle,
}

--- Parameter for `ui.aligned`
---@class (exact) AlignedParams
--- Alignment for its children
---@field align Aligned
AlignedParams = { }

---@class BackgroundParams
--- The background color for the children
---@field background string
BackgroundParams = { }

---@class (exact) BorderParams
--- The style of the border
---@field style BorderStyle?
--- The class of the border
---@field class Border?
--- The border to use
---@field border BorderKind
BorderParams = { }

---@class (exact) BorderStyle
--- The frame title color
---@field title Color?
--- The color of the border
---@field border Color?
--- The color of the border, when focused
---@field border_focused Color?
--- The color of the border, when hovered
---@field border_hovered Color?
BorderStyle = { }

---@class (exact) ButtonParams
--- The style of the button
---@field style ButtonStyle?
--- The class of the button
---@field class Button?
--- The text of the button
---@field text string
--- Function to call when the button is clicked
---@field handler fun(): nil
ButtonParams = { }

---@class (exact) ButtonStyle
--- The button text color
---@field text_color Color?
--- The button background color
---@field background Color?
ButtonStyle = { }

---@class CheckboxParams
--- The style of the checkbox
---@field style CheckboxStyle?
--- The class of the checkbox
---@field class Checkbox?
--- The text of the checkbox
---@field text string
--- The state of the checkbox, a boolean
---@field value Value
CheckboxParams = { }

---@class (exact) CheckboxStyle
--- The character to use when checked
---@field checked string?
--- The character to use when unchecked
---@field unchecked string?
--- The color of the text
---@field text_color Color?
--- The color of the text, when hovered
---@field hovered_color Color?
CheckboxStyle = { }

---@class ConstrainedParams
--- The constraint to use
---@field constraint Constraint
ConstrainedParams = { }

---@class (exact) FillParams
--- Use this color to fill the area
---@field background string
--- Optional width to allocate
---@field width integer?
--- Optional height to allocate
---@field height integer?
FillParams = { }

---@class (exact) FlexParams
--- Tight constraint (ratio between 0.0 and 1.0)
---@field tight number?
--- Loose constraint (ratio between 0.0 and 1.0)
---@field loose number?
FlexParams = { }

---@class (exact) FrameParams
--- The style of the border
---@field style BorderStyle?
--- The class of the border
---@field class Border?
--- The border to use
---@field border BorderKind
--- Alignment for the title
---@field align Align?
--- A string to place in the title
---@field title string
FrameParams = { }

---@class (exact) ListParams
--- Axis for the list
---@field axis Axis?
--- Justification for children on the vertical axis
---@field justify Justify?
--- Alignment for children on the horizontal axis
---@field cross_align CrossAlign?
--- Gap between children
---@field gap integer?
--- Should this be scrollable?
---@field scrollable boolean?
ListParams = { }

---@class (exact) LabelParams
--- The style of the label
---@field style LabelStyle?
--- The class of the label
---@field class Label?
--- The text of the label
---@field text string
LabelParams = { }

---@class (exact) LabelStyle
--- The foreground text color
---@field foreground Color?
--- The text should be italic
---@field italic boolean?
--- The text should be bold
---@field bold boolean?
--- The text should be underline
---@field underline boolean?
--- The text should be faint
---@field faint boolean?
--- The text should be blink
---@field blink boolean?
--- The text should be strikeout
---@field strikeout boolean?
LabelStyle = { }

---@class (exact) MarginParams
--- Padding to the left of the view
---@field left integer?
--- Padding to the right of the view
---@field right integer?
--- Padding to the top of the view
---@field top integer?
--- Padding to the bottom of the view
---@field bottom integer?
--- Padding on both left and right of the view
---@field horizontal integer?
--- Padding on both top and bottom of the view
---@field vertical integer?
--- Padding on each side of the view
---@field all integer?
MarginParams = { }

---@class (exact) ProgressParams
--- The style of the progress bar
---@field style ProgressStyle?
--- The class of the progress bar
---@field class Progress?
--- Axis to use for layout
---@field axis Axis?
--- The value to use (an f32 in the range of 0.0 ..= 1.0)
---@field value Value
ProgressParams = { }

---@class (exact) ProgressStyle
--- The unfilled color
---@field unfilled_color Color?
--- The filled color
---@field filled_color Color?
--- The unfilled color, when hovered
---@field unfilled_hovered Color?
--- The filled color, when hovered
---@field filled_hovered Color?
--- The character to use for the unfilled portion
---@field unfilled string?
--- The character to use for the filled portion
---@field filled string?
ProgressStyle = { }

---@class (exact) SelectedParams
--- The style of the selected value
---@field style SelectedStyle?
--- The class of the selected value
---@field class Selected?
--- The text of the selected value
---@field text string
--- The state of the selected value, a boolean
---@field value Value
SelectedParams = { }

---@class (exact) SelectedStyle
--- The background color
---@field background Color?
--- The text color
---@field text_color Color?
--- The background color, when selected
---@field selected_background Color?
--- The text color, when hovered
---@field hovered_text Color?
--- The background color, when hovered
---@field hovered_background Color?
SelectedStyle = { }

---@class (exact) SliderParams
--- The style of the slider
---@field style SliderStyle?
--- The class of the slider
---@field class Slider?
--- Axis to use for layout
---@field axis Axis?
--- The value to use (an f32 in the range of 0.0 ..= 1.0)
---@field value Value
SliderParams = { }

---@class (exact) SliderStyle
--- The color of the track
---@field track_color Color?
--- The color of the knob
---@field knob_color Color?
--- The color of the track, when hovered
---@field track_hovered Color?
--- The color of the knob, when hovered
---@field knob_hovered Color?
--- The character to use for the knob
---@field knob string?
--- The character to use for the track
---@field track string?
SliderStyle = { }

---@class (exact) TodoParams
--- The class of the selected value
---@field class Todo?
--- The style of the selected value
---@field style TodoStyle?
--- The text of the selected value
---@field text string
--- The state of the selected value, a boolean
---@field value Value
TodoParams = { }

---@class (exact) TodoStyle
--- When selected, the text should be bold
---@field bold boolean?
--- When selected, the text should be faint
---@field faint boolean?
--- When selected, the text should be italic
---@field italic boolean?
--- When selected, the text should be underline
---@field underline boolean?
--- When selected, the text should be blink
---@field blink boolean?
--- When selected, the text should be reverse
---@field reverse boolean?
--- When selected, the text should be strikeout
---@field strikeout boolean?
--- The color of the text
---@field text_color Color?
--- The color of the text, when hovered
---@field hovered_color Color?
TodoStyle = { }

---@class (exact) ToggleParams
--- The boolean state to use
---@field value Value
ToggleParams = { }

---@class (exact) ToggleSwitchParams
--- The class of the selected value
---@field class Toggle?
--- The style of the selected value
---@field style ToggleSwitchStyle?
--- The state of the selected value, a boolean
---@field value Value
--- Axis for the toggle switch
---@field axis Axis?
ToggleSwitchParams = { }

---@class (exact) ToggleSwitchStyle
--- The character to use for the track
---@field track string?
--- The color of the track
---@field track_color Color?
--- The color of the track, when hovered
---@field track_hovered Color?
--- The character to use for the knob when its "on"
---@field on_knob string?
--- The color to use for the knob when its "on"
---@field on_knob_color Color?
--- The character to use for the knob when its "off"
---@field off_knob string?
--- The color to use for the knob when its "off"
---@field off_knob_color Color?
--- The color to use for the knob when its "on" and hovered
---@field on_knob_hovered Color?
--- The color to use for the knob when its "off" and hovered
---@field off_knob_hovered Color?
ToggleSwitchStyle = { }

---@class (exact) UnconstrainedParams
--- Unconstrain the horizontal axis
---@field horizontal boolean?
--- Unconstrain the vertical axis
---@field vertical boolean?
--- Unconstrain both axis
---@field both boolean?
UnconstrainedParams = { }

---@class ui
---  Align its children at a specific anchor
---@field aligned fun(args: AlignedParams): nil
---  Applies a background color to this children
---@field background fun(args: BackgroundParams): nil
---  Border to surround its children
---@field border fun(args: BorderParams): nil
---  A button to click
---@field button fun(args: ButtonParams): nil
---  Center aligns its children
---@field center fun(args: any): nil
---  A checkbox to toggle a boolean
---@field checkbox fun(args: CheckboxParams): nil
---  Specifically constrain a view
---@field constrained fun(args: ConstrainedParams): nil
---  A container that just groups multiple calls into one parent
---@field container fun(args: any): nil
---  A view that expands the remainder of the space on the axis
---@field expand_axis fun(): nil
---  Fill the childrens area, with an optional size constraint
---@field fill fun(args: FillParams): nil
---  Give a flex constraint to its children
---@field flex fun(args: FlexParams): nil
---  A frame, with a title, to surround its children
---@field frame fun(args: FrameParams): nil
---  Horizontal layout of children
---@field horizontal fun(args: ListParams): nil
---  Label displays some text
---@field label fun(args: LabelParams | string): nil
---  Margin applies padding to a view
---@field margin fun(args: MarginParams): nil
---  A progress bar
---@field progress fun(args: ProgressParams): nil
---  A selected boolean value
---@field selected fun(args: SelectedParams): nil
---  Separator to divide some area
---@field separator fun(): nil
---  A slider to adjust a value
---@field slider fun(args: SliderParams): nil
---  A selected value
---@field todo_value fun(args: TodoParams): nil
---  Conditionally show or hide a view
---@field toggle fun(args: ToggleParams): nil
---  A switch that is toggled when clicked
---@field toggle_switch fun(args: ToggleSwitchParams): nil
---  Specifically unconstrained a view
---@field unconstrained fun(args: UnconstrainedParams): nil
---  Vertical layout of children
---@field vertical fun(args: ListParams): nil
ui = { }

