extern crate glutin;

use glutin::KeyboardInput;
use glutin::VirtualKeyCode;
use glutin::Event::Suspended;
use glutin::DeviceId;
use glutin::Event::DeviceEvent;
use glutin::WindowId;
use glutin::Event;
use glutin::Event::WindowEvent;
use glutin::EventsLoop;
use glutin::WindowBuilder;
use glutin::ContextBuilder;
use glutin::GlWindow;
use glutin::ControlFlow;
use glutin::dpi::LogicalSize;

fn main() {
  let mut event_loop = EventsLoop::new();
  let window = WindowBuilder::new()
    .with_title("Hello world!")
    .with_dimensions(LogicalSize::new(1024.0, 768.0));
  let context = ContextBuilder::new();
  let _gl_window = GlWindow::new(window, context, &event_loop).unwrap();

  event_loop.run_forever(handle_event);
}

fn handle_event (e: Event) -> ControlFlow {
  match e {
    WindowEvent { window_id, event } => handle_window_event(window_id, event),
    DeviceEvent { device_id, event } => handle_device_event(device_id, event),
    Event::Awakened => ControlFlow::Continue,
    Suspended(_) => ControlFlow::Continue
  }
}

fn handle_window_event (_w: WindowId, event: glutin::WindowEvent) -> ControlFlow {
  match event {
    glutin::WindowEvent::KeyboardInput {
      input, ..
    } => handle_keyboard_input(input),
    glutin::WindowEvent::CloseRequested => ControlFlow::Break,
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

fn handle_device_event (_d: DeviceId, _event: glutin::DeviceEvent) -> ControlFlow {
  ControlFlow::Continue
}
