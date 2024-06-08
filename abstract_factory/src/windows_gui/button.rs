use crate::gui;

pub struct WindowsButton {}

impl gui::Button for WindowsButton {
    fn press(&self) {
        println!("WindowsButton pressed");
    }
}
