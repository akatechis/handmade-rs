#![feature(custom_attribute)]

#[macro_use]
extern crate vulkano_shader_derive;
#[macro_use]
extern crate vulkano;
extern crate winit;
extern crate vulkano_win;

pub mod shaders;
pub mod application;
