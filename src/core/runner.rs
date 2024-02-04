use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

pub struct Runner {
  glfw: Glfw,
  window: PWindow,
  events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Runner {
  pub fn new(title: String, fullscreen: bool, width: u32, height: u32) -> Self {
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

  pub fn get_glfw(&mut self) -> &mut Glfw {
    &mut self.glfw
  }

  pub fn get_window(&mut self) -> &mut PWindow {
    &mut self.window
  }

  pub fn get_events(&self) -> &GlfwReceiver<(f64, WindowEvent)> {
    &self.events
  }
}