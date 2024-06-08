use crate::gui;

pub struct MacCheckBox {}
impl gui::CheckBox for MacCheckBox {
    fn switch(&self) {
        println!("MacCheckBox switched");
    }
}
