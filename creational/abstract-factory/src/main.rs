use gui_factory::GUIFactory;

mod button;
mod checkbox;
mod gui_factory;
mod mac_factory;
mod win_factory;

use mac_factory::factory::MacFactory;
use win_factory::factory::WinFactory;

fn main() {
    let factory: &dyn GUIFactory = if cfg!(target_os = "windows") {
        &WinFactory
    } else {
        &MacFactory
    };

    let button = factory.create_button();
    let checkbox = factory.create_checkbox();

    button.render();
    checkbox.render()
}
