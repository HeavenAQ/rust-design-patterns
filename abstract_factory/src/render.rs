use crate::gui::GuiFactory;

pub fn render(factory: impl GuiFactory) {
    let button = factory.create_button();
    let checkbox = factory.create_checkbox();

    use crate::gui::{Button, CheckBox};
    button.press();
    checkbox.switch();
}
