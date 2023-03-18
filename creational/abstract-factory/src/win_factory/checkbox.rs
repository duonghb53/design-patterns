use crate::checkbox::Checkbox;

pub struct WinCheckbox;

impl Checkbox for WinCheckbox {
    fn render(&self) {
        println!("Render WinCheckbox");
    }
}