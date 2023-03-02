use crate::gui::{Button, Dialog};

pub struct WindowsButton;

impl Button for WindowsButton {
    fn render(&self) {
        println!("Drawing a Windows Button");
        self.on_click();
    }

    fn on_click(&self) {
        println!("Click! Hello, Windows!");
    }
}

pub struct WindowsDialog;

impl Dialog for WindowsDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
}

