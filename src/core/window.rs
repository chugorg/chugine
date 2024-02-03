extern crate glfw;

use glfw::Context;
use std::string::String;

pub struct Window {}

impl Window {
  pub fn new(title: String, fullscreen: bool, width: u32, height: u32) -> Self {
    let monitor = glfw::Monitor::from_primary();

    let window_mode = if fullscreen {
      glfw::WindowMode::FullScreen(&monitor)
    } else {
      glfw::WindowMode::Windowed
    };

    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, _) = glfw.create_window(width, height, &title, window_mode)
      .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(false);

    while !window.should_close() {
      window.swap_buffers();
      glfw.poll_events();
    }

    Self {}
  }
}