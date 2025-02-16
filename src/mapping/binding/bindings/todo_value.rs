pub use crate::mapping::binding::{Binding, Field};

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
