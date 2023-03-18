use crate::button::Button;

pub struct WinButton;

impl Button for WinButton {
    fn render(&self) {
        println!("Render WinButton");
    }
}