use chugine::Runner;

fn main() {
  let title = String::from("Window");
  let mut runner = Runner::new(title, false, 800, 600);

  runner.run();
}
