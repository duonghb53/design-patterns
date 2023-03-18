use crate::button::Button;

pub struct MacButton;

impl Button for MacButton {
    fn render(&self) {
        println!("Render MacButton");
    }
}