use crate::gui;

pub struct WindowsCheckBox {}

impl gui::CheckBox for WindowsCheckBox {
    fn switch(&self) {
        println!("WindowsCheckBox switched");
    }
}
