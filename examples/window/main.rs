use chugine::{Runner, InputSystem};

fn main() {
  let mut runner = Runner::new("Window", false, 800, 600);
  let mut input = InputSystem::new();

  runner.run(input);
}
