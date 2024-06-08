use abstract_factory::macos_gui::factory::MacFactory;
use abstract_factory::render::render;
use abstract_factory::windows_gui::factory::WindowsFactory;

fn main() {
    let os = "macos";
    match os {
        "windows" => render(WindowsFactory {}),
        "mac" => render(MacFactory {}),
        _ => panic!("Unsupported OS"),
    };
}
