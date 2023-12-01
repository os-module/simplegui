#![no_std]
#![allow(unused)]
extern crate alloc;

use alloc::sync::Arc;
use core::any::Any;

use spin::{Mutex, Once};

pub mod basic;
pub mod complex;


pub trait GPUDevice: Send + Sync + Any {
    fn flush(&self);
    fn draw_point(&mut self, x: i32, y: i32, color: u32);
}


pub static GPU_DEVICE:Once<Arc<Mutex<dyn GPUDevice>>> = Once::new();

pub fn init_gpu(gpu:Arc<Mutex<dyn GPUDevice>>) {
    GPU_DEVICE.call_once(||gpu);
}