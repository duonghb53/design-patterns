use crate::gui_factory::GUIFactory;

use super::{button::MacButton, checkbox::MacCheckbox};
pub struct MacFactory;

impl GUIFactory for MacFactory {
    fn create_button(&self) -> Box<dyn crate::button::Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn crate::checkbox::Checkbox> {
        Box::new(MacCheckbox)
    }
}
