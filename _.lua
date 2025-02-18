---@alias Color string #RGB | #RGBA | #RRGGBB | #RRGGBBAA hex string

---@type fun(func: table<fun(): string>) lazily generate a string
---@diagnostic disable-next-line: lowercase-global
lazy = function(func) end
---@alias lazy_args nil

---@class rt
rt = {
    --- sleep for `millis`
    ---@async
    ---@params millis: integer
    ---@return nil
    sleep_ms = function(millis) end,
    --- spawns an async tasks, returning its id
    ---@params task: fun():nil | thread
    ---@return integer
    spawn = function(task) end,
    --- attempts to stop a running task
    ---@params id: integer?
    ---@return boolean
    stop = function(id) end,
}

---@class (exact) Constraint
---@field exact_size fun(w: integer, h: integer): Constraint The view has an exact size
---@field exact_height fun(h: integer): Constraint The view has an exact height
---@field exact_width fun(w: integer): Constraint The view has an exact width
---@field max_size fun(w: integer, h: integer): Constraint The view has a max size
---@field max_height fun(h: integer): Constraint The view has a max height
---@field max_width fun(w: integer): Constraint The view has a max width
---@field min_size fun(w: integer, h: integer): Constraint The view has a min size
---@field min_width fun(w: integer): Constraint The view has a min width
---@field min_height fun(h: integer): Constraint The view has a min height
Constraint = {}

---@class (exact) Value
---@field new fun(value: integer|number|boolean|string): Value create a new value
---@field persist fun(id: string, value: integer|number|boolean|string): Value create a new value, persisted and accessible via `id`
---@field destroy fun(id: string): boolean destroys a persisted value `id`, if it exists
---@field value integer|number|boolean|string get the inner value
Value = {}

---@enum Align
Align = {
    ---  Align to the start of the area
    min    = 0,
    ---  Align to the middle of the area
    middle = 1,
    ---  Align to the end of the area
    max    = 2,
}

---@enum Aligned
Aligned = {
    ---  Align to the horizontal left and vertical top
    left_top      = 0,
    ---  Align to the horizontal center and vertical top
    center_top    = 1,
    ---  Align to the horizontal right and vertical top
    right_top     = 2,
    ---  Align to the horizontal left and vertical center
    left_center   = 3,
    ---  Align to the horizontal center and vertical center
    center        = 4,
    ---  Align to the horizontal right and vertical center
    right_center  = 5,
    ---  Align to the horizontal left and vertical bottom
    left_bottom   = 6,
    ---  Align to the horizontal center and vertical bottom
    center_bottom = 7,
    ---  Align to the horizontal right and vertical bottom
    right_bottom  = 8,
}

---@enum Axis
Axis = {
    ---  The vertical axis
    vertical   = 0,
    ---  The horizontal axis
    horizontal = 1,
}

---@class (exact) BorderStyle
---@field title Color?  The frame title color
---@field border Color?  The color of the border
---@field border_focused Color?  The color of the border, when focused
---@field border_hovered Color?  The color of the border, when hovered
BorderStyle = {}

---@enum Border
Border = {
    ---  The default style
    default     = 0,
    ---  An interactive style
    interactive = 1,
}

---@enum BorderKind
BorderKind = {
    ---  No border
    empty      = 0,
    ---  A thin border
    thin       = 1,
    ---  A thin, but wide border
    thin_wide  = 2,
    ---  A rounded border
    rounded    = 3,
    ---  A double-line border
    double     = 4,
    ---  A thick border
    thick      = 5,
    ---  A thick, but tall border
    thick_tall = 6,
    ---  A thick, but wide border
    thick_wide = 7,
}

---@class (exact) ButtonStyle
---@field text_color Color?  The button text color
---@field background Color?  The button background color
ButtonStyle = {}

---@enum Button
Button = {
    ---  The default style
    default = 0,
    ---  Denotes this button is for success
    success = 1,
    ---  Denotes this button is for information
    info    = 2,
    ---  Denotes this button is for warning
    warning = 3,
    ---  Denotes this button is for danger
    danger  = 4,
}

---@class (exact) CheckboxStyle
---@field checked string?  The character to use when checked
---@field unchecked string?  The character to use when unchecked
---@field text_color Color?  The color of the text
---@field hovered_color Color?  The color of the text, when hovered
CheckboxStyle = {}

---@enum Checkbox
Checkbox = {
    ---  The default style
    default  = 0,
    ---  A markdown inspired style
    markdown = 1,
    ---  An ascii checkbox style
    ascii    = 2,
}

---@enum CrossAlign
CrossAlign = {
    ---  Alignment begins at the 'start' of the area
    min     = 0,
    ---  Alignment begins at the 'end' of the area
    max     = 1,
    ---  Alignment is in the middle, extra space is applied before and after
    center  = 2,
    ---  The elements stretch to fill the area
    stretch = 3,
    ---  The elements fill the entire area
    fill    = 4,
}

---@enum Justify
Justify = {
    ---  The extra space is applied to the end
    min           = 0,
    ---  The extra space is applied to the start
    max           = 1,
    ---  The extra space is applied to the start and end
    center        = 2,
    ---  Place the space between the elements
    space_between = 3,
    ---  Place the space around the elements
    space_around  = 4,
    ---  Evenly space the elements
    space_evenly  = 5,
}

---@class (exact) LabelStyle
---@field foreground Color?  The foreground text color
---@field italic boolean?  The text should be italic
---@field bold boolean?  The text should be bold
---@field underline boolean?  The text should be underline
---@field faint boolean?  The text should be faint
---@field blink boolean?  The text should be blink
---@field strikeout boolean?  The text should be strikeout
LabelStyle = {}

---@enum Label
Label = {
    ---  The default style
    default = 0,
    ---  Denotes this label is for information
    info    = 1,
    ---  Denotes this label is for warning
    warning = 2,
    ---  Denotes this label is for success
    success = 3,
    ---  Denotes this label is for danger
    danger  = 4,
}

---@class (exact) ProgressStyle
---@field unfilled_color Color?  The unfilled color
---@field filled_color Color?  The filled color
---@field unfilled_hovered Color?  The unfilled color, when hovered
---@field filled_hovered Color?  The filled color, when hovered
---@field unfilled string?  The character to use for the unfilled portion
---@field filled string?  The character to use for the filled portion
ProgressStyle = {}

---@enum Progress
Progress = {
    ---  Default style
    default       = 0,
    ---  A medium filled style
    medium_filled = 1,
    ---  A full filled style
    filled        = 2,
    ---  A thin style
    thin          = 3,
    ---  A thick style
    thick         = 4,
    ---  A thin, but dashed style
    thin_dashed   = 5,
    ---  A thick, but dashed style
    thick_dashed  = 6,
}

---@class (exact) SelectedStyle
---@field background Color?  The background color
---@field text_color Color?  The text color
---@field selected_background Color?  The background color, when selected
---@field hovered_text Color?  The text color, when hovered
---@field hovered_background Color?  The background color, when hovered
SelectedStyle = {}

---@enum Selected
Selected = {
    ---  The default style
    default = 0,
    ---  This element reacts to hovers
    hovered = 1,
}

---@class (exact) SliderStyle
---@field track_color Color?  The color of the track
---@field knob_color Color?  The color of the knob
---@field track_hovered Color?  The color of the track, when hovered
---@field knob_hovered Color?  The color of the knob, when hovered
---@field knob string?  The character to use for the knob
---@field track string?  The character to use for the track
SliderStyle = {}

---@enum Slider
Slider = {
    ---  The default style
    default       = 0,
    ---  Small track and rounded knob
    small_rounded = 1,
    ---  Small track and diamond knob
    small_diamond = 2,
    ---  Small track and square knob
    small_square  = 3,
    ---  Medium track and large knob
    large         = 4,
    ---  Large track and large knob
    large_filled  = 5,
}

---@class (exact) TodoStyle
---@field bold boolean?  When selected, the text should be bold
---@field faint boolean?  When selected, the text should be faint
---@field italic boolean?  When selected, the text should be italic
---@field underline boolean?  When selected, the text should be underline
---@field blink boolean?  When selected, the text should be blink
---@field reverse boolean?  When selected, the text should be reverse
---@field strikeout boolean?  When selected, the text should be strikeout
---@field text_color Color?  The color of the text
---@field hovered_color Color?  The color of the text, when hovered
TodoStyle = {}

---@enum Todo
Todo = {
    ---  The default style
    default = 0,
}

---@class (exact) ToggleStyle
---@field track string?  The character to use for the track
---@field track_color Color?  The color of the track
---@field track_hovered Color?  The color of the track, when hovered
---@field on_knob string?  The character to use for the knob when its "on"
---@field on_knob_color Color?  The color to use for the knob when its "on"
---@field off_knob string?  The character to use for the knob when its "off"
---@field off_knob_color Color?  The color to use for the knob when its "off"
---@field on_knob_hovered Color?  The color to use for the knob when its "on" and hovered
---@field off_knob_hovered Color?  The color to use for the knob when its "off" and hovered
ToggleStyle = {}

---@enum Toggle
Toggle = {
    ---  The default style
    default       = 0,
    ---  A large knob
    large         = 1,
    ---  A small rounded knob
    small_rounded = 2,
    ---  A small diamond knob
    small_diamond = 3,
    ---  A small square knob
    small_square  = 4,
}

---@class aligned  Align its children at a specific anchor
---@field align Aligned  Alignment for the children

---@class background  Background of its children
---@field background string  The background color for the children

---@class border  Border to surround its children
---@field style BorderStyle?  The style of the border
---@field class Border?  The class of the border
---@field border BorderKind  The border to use

---@class button  A button to click
---@field style ButtonStyle?  The style of the button
---@field class Button?  The class of the button
---@field text string | lazy_args  The text of the button
---@field handler fun(): nil  Function to call when the button is clicked

---@class center  Center a view in the current layout

---@class checkbox  A checkbox to toggle a boolean
---@field style CheckboxStyle?  The style of the checkbox
---@field class Checkbox?  The class of the checkbox
---@field text string | lazy_args  The text of the checkbox
---@field value Value  The state of the checkbox, a boolean

---@class constrained  Specifically constrain a view
---@field constraint Constraint  The constraint to use

---@class container  A container that just groups multiple calls into one parent

---@class expand_axis  A view that expands the remainder of the space on the axis

---@class fill  Fill the childrens area, with an optional size constraint
---@field background string  Use this color to fill the area
---@field space {width: integer?, height: integer?}  Optional space to allocate

---@class flex  Give a flex constraint to its children
---@field tight number?  Tight constraint (ratio between 0.0 and 1.0)
---@field loose number?  Loose constraint (ratio between 0.0 and 1.0)

---@class frame  Frame is a border with a title
---@field style BorderStyle?  The style of the frame
---@field class Border?  The class of the frame
---@field border BorderKind  The border to use
---@field align Align?  Alignment for the title
---@field title string | lazy_args  A string to place in the title

---@class horizontal  Horizontal layout of children
---@field justify Justify?  Justification for children on the horizontal axis
---@field cross_align CrossAlign?  Alignment for children on the vertical axis
---@field gap integer?  Gap between children
---@field scrollable boolean?  Should this be scrollable?

---@class label  Label displays some text
---@field style LabelStyle?  The style of the label
---@field class Label?  The class of the label
---@field text string | lazy_args  The text of the label

---@class margin  Margin applies padding to a view
---@field left integer?  Padding to the left of the view
---@field right integer?  Padding to the right of the view
---@field top integer?  Padding to the top of the view
---@field bottom integer?  Padding to the bottom of the view
---@field horizontal integer?  Padding on both left and right of the view
---@field vertical integer?  Padding on both top and bottom of the view
---@field all integer?  Padding on each side of the view

---@class progress  A progress bar
---@field style ProgressStyle?  The style of the progress bar
---@field class Progress?  The class of the progress bar
---@field axis Axis?  Axis to use for layout
---@field value Value | number  The value to use (an f32 in the range of 0.0 ..= 1.0)

---@class selected  A selected boolean value
---@field style SelectedStyle?  The style of the selected value
---@field class Selected?  The class of the selected value
---@field text string | lazy_args  The text of the selected value
---@field value Value  The state of the selected value, a boolean

---@class separator  Separator to divide some area

---@class slider  A slider to adjust a value
---@field style SliderStyle?  The style of the slider
---@field class Slider?  The class of the slider
---@field axis Axis?  Axis to use for layout
---@field value Value  The value to use (an f32 in the range of 0.0 ..= 1.0)

---@class todo_value  A selected value
---@field style TodoStyle?  The style of the selected value
---@field class Todo?  The class of the selected value
---@field text string | lazy_args  The text of the selected value
---@field value Value  The state of the selected value, a boolean

---@class toggle  Conditionally show or hide a view
---@field value Value  The boolean state to use

---@class toggle_switch  A switch that is toggled when clicked
---@field style ToggleStyle?  The style of the selected value
---@field class Toggle?  The class of the selected value
---@field value Value  The state of the selected value, a boolean

---@class unconstrained  Specifically unconstrained a view
---@field constraint {horizontal: boolean?, vertical: boolean?, both: boolean?}  Which axis to remove the constraints for

---@class vertical  Vertical layout of children
---@field justify Justify?  Justification for children on the vertical axis
---@field cross_align CrossAlign?  Alignment for children on the horizontal axis
---@field gap integer?  Gap between children
---@field scrollable boolean?  Should this be scrollable?

---@class ui
---@field aligned fun(args: aligned): nil  Align its children at a specific anchor
---@field background fun(args: background): nil  Background of its children
---@field border fun(args: border): nil  Border to surround its children
---@field button fun(args: button): nil  A button to click
---@field center fun(args: any): nil  Center a view in the current layout
---@field checkbox fun(args: checkbox): nil  A checkbox to toggle a boolean
---@field constrained fun(args: constrained): nil  Specifically constrain a view
---@field container fun(args: any): nil  A container that just groups multiple calls into one parent
---@field expand_axis fun(): nil  A view that expands the remainder of the space on the axis
---@field fill fun(args: fill): nil  Fill the childrens area, with an optional size constraint
---@field flex fun(args: flex): nil  Give a flex constraint to its children
---@field frame fun(args: frame): nil  Frame is a border with a title
---@field horizontal fun(args: horizontal): nil  Horizontal layout of children
---@field label fun(args: string | lazy_args | label): nil  Label displays some text
---@field margin fun(args: margin): nil  Margin applies padding to a view
---@field progress fun(args: Value | number | progress): nil  A progress bar
---@field selected fun(args: selected): nil  A selected boolean value
---@field separator fun(): nil  Separator to divide some area
---@field slider fun(args: Value | slider): nil  A slider to adjust a value
---@field todo_value fun(args: todo_value): nil  A selected value
---@field toggle fun(args: toggle): nil  Conditionally show or hide a view
---@field toggle_switch fun(args: Value | toggle_switch): nil  A switch that is toggled when clicked
---@field unconstrained fun(args: unconstrained): nil  Specifically unconstrained a view
---@field vertical fun(args: vertical): nil  Vertical layout of children
ui = { }


