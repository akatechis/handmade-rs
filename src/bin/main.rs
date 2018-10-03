extern crate handmade_rs;

use handmade_rs::application::Application;

fn main() {
  let mut app = Application::new();

  if let Err(cause) = app.main_loop() {
    println!("Error: {}", cause);
  } else {
    println!("All done.");
  }
}
