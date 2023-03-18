use crate::{gui_factory::GUIFactory, button::Button, checkbox::Checkbox};

use super::{button::WinButton, checkbox::WinCheckbox};

pub struct WinFactory;

impl GUIFactory for WinFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WinButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WinCheckbox)
    }
}