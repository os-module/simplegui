#![no_std]
#![allow(unused)]
extern crate alloc;

use core::any::Any;
use core::ops::{Deref, DerefMut};
use spin::{Mutex, MutexGuard};

pub mod basic;
pub mod complex;


pub trait GPUDevice: Send + Sync + Any {
    fn update_cursor(&self);
    fn get_frame_buffer(&self) -> &mut [u8];
    fn flush(&self);
    fn get_resolution(&self) -> (u32, u32);
}
