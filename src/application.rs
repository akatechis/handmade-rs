use std::sync::Arc;
use vulkano::sync::GpuFuture;
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
use vulkano::pipeline::ComputePipeline;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBuffer;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use winit::Event;
use winit::Window;
use winit::EventsLoop;
use winit::WindowEvent;
use winit::KeyboardInput;
use winit::WindowBuilder;
use winit::VirtualKeyCode;
use super::shaders::compute;

const LUNARG_VALIDATION_LAYER: &'static str =
  "VK_LAYER_LUNARG_standard_validation";

pub struct Application {
  config: Configuration,
  events_loop: EventsLoop,
  window: Window,
  device: Arc<Device>,
  queue: Arc<Queue>,
}

impl Application {
  pub fn with_configuration (config: Configuration) -> Application {
    let events_loop = EventsLoop::new();
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
    let data_buf = CpuAccessibleBuffer::from_iter(
      device.clone(),
      BufferUsage::all(),
      data_iter
    )
    .expect("failed to create buffer");

    let shader = compute::Shader::load(device.clone())
      .expect("failed to create shader module");

    let compute_pipeline = Arc::new(ComputePipeline::new(
      device.clone(),
      &shader.main_entry_point(),
      &())
    .expect("failed to create compute pipeline"));

    let set = Arc::new(PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
      .add_buffer(data_buf.clone()).unwrap()
      .build().unwrap()
    );

    let command_buf = AutoCommandBufferBuilder::new(
      device.clone(),
      queue.family()
    )
    .unwrap()
    .dispatch([1024, 1, 1], compute_pipeline.clone(), set.clone(), ())
    .unwrap()
    .build()
    .unwrap();

    let finished = command_buf.execute(queue.clone()).unwrap();

    finished.then_signal_fence_and_flush().unwrap()
      .wait(None).unwrap();

    let content: Vec<u32> = data_buf.read().unwrap().iter().map(|n|*n).collect();
    println!("{:?}", content);

    Application {
      events_loop, window, device, queue, config
    }
  }

  pub fn new() -> Application {
    Application::with_configuration(Configuration::default())
  }

  pub fn main_loop(&mut self) -> Result<(), String> {
    let mut done = false;

    while !done {
      self.events_loop.poll_events(|ev| {
        match ev {
          Event::WindowEvent { event: win_event, .. } => {
            done = !handle_window_event(win_event);
          },
          _ => ()
        }
      });
    }

    Ok(())
  }
}

fn handle_window_event (event: WindowEvent) -> bool {
  match event {
    WindowEvent::KeyboardInput { input, .. } => {
      handle_keyboard_input(input)
    },
    WindowEvent::CloseRequested => {
      false
    },
    _ => true
  }
}

fn handle_keyboard_input(input: KeyboardInput) -> bool {
  if let Some(key) = input.virtual_keycode {
    println!("Key pressed: {:?}", key);
    if key == VirtualKeyCode::Escape {
      return false;
    }
  }
  true
}

fn create_instance() -> (Arc<Instance>, Option<DebugCallback>) {
  let enabled_extensions = collect_vulkan_extensions();
  let enabled_layers = collect_vulkan_layers();

  let app_info = app_info_from_cargo_toml!();

  #[cfg(not(release))]
  {
    println!("Application info: {:?} \n", app_info);
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

  #[cfg(not(release))]
  {
    println!("\n\n================");
    println!("Selected device:\n");
    inspect_device_info(&device);
    println!("================");
  }

  device
}

fn create_device (
  phys_device: PhysicalDevice
) -> (Arc<Device>, Arc<Queue>) {
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
    Some(DebugCallback::new(
      instance,
      messages,
      vulkan_debug_message_received
    ).unwrap())
  }

  #[cfg(release)]
  {
    None
  }
}

fn vulkan_debug_message_received (msg: &Message) {
  println!("[{}]: {}", msg.layer_prefix, msg.description);
}

#[derive(Debug, Default)]
pub struct Configuration {
}
