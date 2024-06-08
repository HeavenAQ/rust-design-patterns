use crate::gui;

pub struct MacButton {}
impl gui::Button for MacButton {
    fn press(&self) {
        println!("MacButton pressed");
    }
}
