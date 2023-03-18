use crate::checkbox::Checkbox;

pub struct MacCheckbox;

impl Checkbox for MacCheckbox {
    fn render(&self) {
        println!("Render MacCheckbox");
    }
}
