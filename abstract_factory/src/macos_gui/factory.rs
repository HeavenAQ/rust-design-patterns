use crate::gui;

use super::{button::MacButton, checkbox::MacCheckBox};

pub struct MacFactory {}

impl gui::GuiFactory for MacFactory {
    type B = MacButton;
    type C = MacCheckBox;

    fn create_button(&self) -> Self::B {
        MacButton {}
    }

    fn create_checkbox(&self) -> Self::C {
        MacCheckBox {}
    }
}
