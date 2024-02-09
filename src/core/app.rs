use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

use crate::InputSystem;

pub struct Process {}
pub struct PhysicsProcess {}

pub struct App {
    glfw: Glfw,
    window: PWindow,
    input: InputSystem,
    events: GlfwReceiver<(f64, WindowEvent)>,
    systems: Option<System>,
}

pub enum SystemType {
    Process(Process),
    PhysicsProcess(PhysicsProcess),
}

struct System {
    system_type: SystemType,
    func: fn(),
}

impl App {
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

        let input = InputSystem::new();

        Self {
            glfw,
            window,
            input,
            events,
            systems: None,
        }
    }

    pub fn add_system(&mut self, system_type: SystemType, func: fn()) {
        if self.systems.is_none() {
            self.systems = Some(System { 
                system_type: system_type, 
                func: func,
            });
        } else {
            panic!("Failed to create System.");
        }
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            self.window.swap_buffers();
            self.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    WindowEvent::Key(key, _, action, _) => {
                        self.input.update(key, action);
                    }

                    _ => {},
                }
            }

            (self.systems.as_ref().unwrap().func)();
        }
    }

    pub fn get_glfw(&mut self) -> &mut Glfw {
        &mut self.glfw
    }

    pub fn get_window(&mut self) -> &mut PWindow {
        &mut self.window
    }
    
    pub fn get_input(&mut self) -> &mut InputSystem {
        &mut self.input
    }

    pub fn get_events(&mut self) -> &mut GlfwReceiver<(f64, WindowEvent)> {
        &mut self.events
    }
}
