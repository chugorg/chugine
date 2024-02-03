extern crate chugine;

use chugine::Window;
use std::string::String;

fn main() {
    let title = String::from("Window");
    let _window = Window::new(title, false, 800, 600);
}
