use chugine::Runner;
use glfw::Context;

fn main() {
    let title = String::from("Window");
    let mut runner = Runner::new(title, false, 800, 600);

    while !runner.get_window().should_close() {
        runner.get_window().swap_buffers();
        runner.get_glfw().poll_events();
    }
}