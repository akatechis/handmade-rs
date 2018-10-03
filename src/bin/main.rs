extern crate winit;
extern crate vulkano_win;
#[macro_use]
extern crate vulkano;

use std::sync::Arc;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::instance::Features;
use vulkano::instance::debug::Message;
use vulkano::instance::debug::DebugCallback;
use vulkano::instance::debug::MessageTypes;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Queue;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::buffer::BufferUsage;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use winit::WindowEvent;
use winit::WindowBuilder;
use winit::Event;
use winit::EventsLoop;
use winit::KeyboardInput;
use winit::VirtualKeyCode;
use winit::ControlFlow;

const LUNARG_VALIDATION_LAYER: &'static str =
  "VK_LAYER_LUNARG_standard_validation";

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

  let (instance, _debug_callback) = create_instance();

  let phys_device = select_physical_device(&instance);
  let (device, queue) = create_device(phys_device);

  let data_iter = 0..65536;
  let src_buf = CpuAccessibleBuffer::from_iter(
    device.clone(),
    BufferUsage::all(),
    data_iter
  )
  .expect("failed to create buffer");

  let command_buf = AutoCommandBufferBuilder::new(
    device.clone(),
    queue.family()
  )
  .unwrap();

  let _surface = vulkano_win::create_vk_surface(window, instance);

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

fn create_instance() -> (Arc<Instance>, Option<DebugCallback>) {
  let enabled_extensions = collect_vulkan_extensions();
  let enabled_layers = collect_vulkan_layers();

  let app_info = app_info_from_cargo_toml!();
  // println!("Application info: {:?}", app_info);

  #[cfg(not(release))]
  {
    println!("================");
    println!("Creating Vulkan instance: \n");
    println!("Extensions: {:?}", enabled_extensions);
    println!("Layers: {:?}", enabled_layers);
    println!("================");
  }

  let instance = Instance::new(
    Some(&app_info),
    &enabled_extensions,
    enabled_layers
  )
  .expect("failed to create Vulkan instance");

  let debug_callback = create_vulkan_debug_callback(&instance);

  (instance, debug_callback)
}

fn collect_vulkan_extensions () -> InstanceExtensions {
  let mut required_extensions = vulkano_win::required_extensions();

  #[cfg(not(release))]
  {
    required_extensions.ext_debug_report = true;
  }

  required_extensions
}

fn collect_vulkan_layers<'a> () -> Vec<&'a str> {
  let mut layers = vec![];
  #[cfg(not(release))]
  {
    for layer in vulkano::instance::layers_list().unwrap() {
      if layer.name() == LUNARG_VALIDATION_LAYER {
        layers.push(LUNARG_VALIDATION_LAYER);
      }
    }
  }

  layers
}

fn inspect_device_info (d: &PhysicalDevice) {
  println!("Device name: {:?}", d.name());
  println!("Device type: {:?}", d.ty());
  println!("API version: {:?}", d.api_version());
  println!("Driver version: {:?}", d.driver_version());

  let lim = d.limits();
  println!("Device memory: {}", lim.max_memory_allocation_count());

  for family in d.queue_families() {
    println!("Queue family: id {}", family.id());
    println!("Queues: {}", family.queues_count());
    println!("Compute? {}", if family.supports_compute() { "YES" } else { "NO" });
    println!("Graphics? {}", if family.supports_graphics() { "YES" } else { "NO" });
    println!("Transfer? {}", if family.supports_transfers() { "YES" } else { "NO" });
    println!("Sparse Binding? {}", if family.supports_sparse_binding() { "YES" } else { "NO" });
  }
}

fn select_physical_device (instance: &Arc<Instance>) -> PhysicalDevice {
  let device = PhysicalDevice::enumerate(&instance).next()
    .expect("No physical device to select");

  println!("================");
  println!("Selected device:\n");
  inspect_device_info(&device);
  println!("================");

  device
}

fn create_device (phys_device: PhysicalDevice) -> (Arc<Device>, Arc<Queue>) {
  let queue_family = phys_device.queue_families()
    .find(|&q| q.supports_graphics())
    .expect("couldn't find a graphical queue family");
  let (device, mut queues) = Device::new(
    phys_device,
    &Features::none(),
    &DeviceExtensions::none(),
    [(queue_family, 0.5)].iter().cloned()
  )
  .expect("failed to create device");

  (device, queues.next().unwrap())
}

fn create_vulkan_debug_callback (
  instance: &Arc<Instance>
) -> Option<DebugCallback> {

  #[cfg(not(release))]
  {
    let messages = MessageTypes {
      error: true,
      warning: true,
      performance_warning: true,
      information: false,
      debug: true,
    };
    Some(DebugCallback::new(instance, messages, vulkan_debug_message_received).unwrap())
  }

  #[cfg(release)]
  {
    None
  }
}

fn vulkan_debug_message_received (msg: &Message) {
  println!("[{}]: {}", msg.layer_prefix, msg.description);
}
