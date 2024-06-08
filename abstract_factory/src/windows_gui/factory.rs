use crate::gui::GuiFactory;

use super::{button::WindowsButton, checkbox::WindowsCheckBox};

pub struct WindowsFactory {}

impl GuiFactory for WindowsFactory {
    type B = WindowsButton;
    type C = WindowsCheckBox;

    fn create_button(&self) -> Self::B {
        WindowsButton {}
    }

    fn create_checkbox(&self) -> Self::C {
        WindowsCheckBox {}
    }
}
