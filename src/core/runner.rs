use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

use crate::InputSystem;

pub struct Runner {
  glfw: Glfw,
  window: PWindow,
  events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Runner {
  pub fn new(title: &str, fullscreen: bool, width: u32, height: u32) -> Self {
    let monitor = glfw::Monitor::from_primary();

    let window_mode = if fullscreen {
      glfw::WindowMode::FullScreen(&monitor)
    } else {
      glfw::WindowMode::Windowed
    };

    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw.create_window(width, height, &title, window_mode)
      .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    Self {
      glfw,
      window,
      events,
    }
  }

  pub fn run(&mut self, mut input: InputSystem) {
    while !self.window.should_close() {
        self.window.swap_buffers();
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    input.update(key, action);
                }

                _ => {},
            }
        }
    }
  }

  pub fn get_glfw(&mut self) -> &mut Glfw {
    &mut self.glfw
  }

  pub fn get_window(&mut self) -> &mut PWindow {
    &mut self.window
  }

  pub fn get_events(&mut self) -> &mut GlfwReceiver<(f64, WindowEvent)> {
    &mut self.events
  }
}
