extern crate winit;
extern crate vulkano_win;

use std::sync::Arc;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::ApplicationInfo;
use vulkano::instance::Version;
use winit::WindowEvent;
use winit::WindowBuilder;
use winit::Event;
use winit::EventsLoop;
use winit::KeyboardInput;
use winit::VirtualKeyCode;
use winit::ControlFlow;
use winit::dpi::LogicalSize;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

fn main() {
  let mut events_loop = EventsLoop::new();
  let _window = WindowBuilder::new()
    .with_title("RustHero")
    .with_dimensions(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
    .build(&events_loop);

  let _inst = create_instance();

  let mut done = false;

  while !done {
    events_loop.poll_events(|ev| {
      match ev {
        Event::WindowEvent { event: win_event, .. } => {
          if handle_window_event(win_event) == ControlFlow::Break {
            done = true;
          }
        },
        _ => ()
      }
    });
  }
}

fn handle_window_event (event: WindowEvent) -> ControlFlow {
  match event {
    WindowEvent::KeyboardInput {
      input, ..
    } => handle_keyboard_input(input),
    WindowEvent::CloseRequested => ControlFlow::Break,
    _ => ControlFlow::Continue
  }
}

fn handle_keyboard_input(input: KeyboardInput) -> ControlFlow {
  if let Some(key) = input.virtual_keycode {
    if key == VirtualKeyCode::Escape {
      ControlFlow::Break
    } else {
      ControlFlow::Continue
    }
  } else {
    ControlFlow::Continue
  }
}

fn create_instance() -> Arc<Instance> {
  let supported_extensions = InstanceExtensions::supported_by_core()
    .expect("failed to retrieve supported extensions");

  let app_info = ApplicationInfo {
    application_name: Some("RustHero".into()),
    application_version: Some(Version { major: 0, minor: 1, patch: 0 }),
    engine_name: Some("no egine".into()),
    engine_version: Some(Version { major: 0, minor: 1, patch: 0 }),
  };

  let required_extensions = vulkano_win::required_extensions();
  Instance::new(Some(&app_info), &required_extensions, None)
    .expect("failed to create Vulkan instance")
}