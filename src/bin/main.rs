extern crate winit;
extern crate vulkano_win;

use std::sync::Arc;
use vulkano::instance::Instance;
use vulkano::instance::ApplicationInfo;
use vulkano::instance::Version;
use vulkano::instance::PhysicalDevice;
use winit::WindowEvent;
use winit::WindowBuilder;
use winit::Event;
use winit::EventsLoop;
use winit::KeyboardInput;
use winit::VirtualKeyCode;
use winit::ControlFlow;

fn main() {
  let mut events_loop = EventsLoop::new();
  let window = WindowBuilder::new()
    .with_title("RustHero")
    .with_resizable(false)
    .build(&events_loop)
    .unwrap();

  // Get the primary monitor, and make the window fullscreen there
  let primary_monitor = window.get_primary_monitor();
  window.set_fullscreen(Some(primary_monitor));

  let inst = create_instance();
  for device in PhysicalDevice::enumerate(&inst) {
    print_device_info(&device);
  }

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
    println!("Key pressed: {:?}", key);
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
  let required_extensions = vulkano_win::required_extensions();

  let app_info = ApplicationInfo {
    application_name: Some("RustHero".into()),
    application_version: Some(Version { major: 0, minor: 1, patch: 0 }),
    engine_name: Some("no engine".into()),
    engine_version: Some(Version { major: 0, minor: 1, patch: 0 }),
  };

  let mut layers = vec![];
  #[cfg(not(release))]
  {
    for layer in vulkano::instance::layers_list().unwrap() {
      if layer.name() == "VK_LAYER_LUNARG_standard_validation" {
        println!("Enabling layer: \"{}\"", "VK_LAYER_LUNARG_standard_validation");
        layers.push("VK_LAYER_LUNARG_standard_validation");
      }
    }
  }

  Instance::new(Some(&app_info), &required_extensions, layers)
    .expect("failed to create Vulkan instance")
}

fn print_device_info (d: &PhysicalDevice) {
  println!("Device name: {:?}", d.name());
  println!("Device type: {:?}", d.ty());
  println!("API version: {:?}", d.api_version());
  println!("Driver version: {:?}", d.driver_version());

  let lim = d.limits();
  println!("Device memory: {}", lim.max_memory_allocation_count());
}
