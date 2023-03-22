#![no_std]
#![no_main]

extern crate alloc;
extern crate opensbi_rt;

use core::{
    ptr::NonNull,
};
use lazy_static::lazy_static;
use log::{info};
use simplegui::UPIntrFreeCell;
use core::any::Any;
use alloc::sync::Arc;
use tinybmp::Bmp;
use virtio_drivers::device::gpu::VirtIOGpu;
use virtio_drivers::transport::mmio::{MmioTransport, VirtIOHeader};
use virtio_drivers::transport::Transport;
use alloc::vec::Vec;
use crate::hal::HalImpl;
use embedded_graphics_core::pixelcolor::Rgb888;
use embedded_graphics_core::prelude::{Point, Size};
use simplegui::complex::desktop::Desktop;
use simplegui::complex::terminal::GodTerminal;
use simplegui::complex::snake::Snake;

mod hal;

static BMP_DATA: &[u8] = include_bytes!("../assert/mouse.bmp");
const VIRTIO7: usize = 0x10007000;

const VIRTGPU_XRES: usize = 1280;
const VIRTGPU_YRES: usize = 800;

pub trait GPUDevice: Send + Sync + Any {
    fn update_cursor(&self);
    fn get_frame_buffer(&self) -> &mut [u8];
    fn flush(&self);
}
lazy_static!(
    pub static ref GPU_DEVICE: Arc<dyn GPUDevice> = {
        let header = NonNull::new(VIRTIO7 as *mut VirtIOHeader).unwrap();
        let transport = match unsafe { MmioTransport::new(header) } {
            Err(e) => {
                panic!("Error creating VirtIO MMIO transport: {}", e)
            },
            Ok(transport) => {
                info!(
                "Detected virtio MMIO device with vendor id {:#X}, device type {:?}, version {:?}",
                transport.vendor_id(),
                transport.device_type(),
                transport.version(),
            );
                transport
            }
        };
        let mut gpu = VirtIOGpu::<HalImpl,_>::new(transport).expect("failed to create gpu driver");
        let (x,y) = gpu.resolution().unwrap();
        info!("resolution: {}x{}", x, y);
        let fbuffer = gpu.setup_framebuffer().unwrap();
        let len = fbuffer.len();
        let ptr = fbuffer.as_mut_ptr();
        let fb = unsafe{core::slice::from_raw_parts_mut(ptr, len)};
        let bmp = Bmp::<Rgb888>::from_slice(BMP_DATA).unwrap();
        let raw = bmp.as_raw();
        let mut b = Vec::new();
        for i in raw.image_data().chunks(3) {
            let mut v = i.to_vec();
            b.append(&mut v);
            if i == [255, 255, 255] {
                b.push(0x0) //白色直接透明
            } else {
                b.push(0xff)
            }
        }
        gpu.setup_cursor(b.as_slice(), 0, 0, 50, 50).unwrap();
        let gpu = VirtIOGPU {
            gpu: UPIntrFreeCell::new(gpu),
            fb,
        };
        Arc::new(gpu)
    };
);

pub struct VirtIOGPU<T: Transport> {
    gpu: UPIntrFreeCell<VirtIOGpu<'static, HalImpl, T>>,
    fb: &'static [u8],
}

unsafe impl<T: Transport> Send for VirtIOGPU<T> {}
unsafe impl<T: Transport> Sync for VirtIOGPU<T> {}

impl<T: Transport + 'static> GPUDevice for VirtIOGPU<T> {
    fn update_cursor(&self) {
        let mut gpu = self.gpu.exclusive_access();
        gpu.move_cursor(0, 0).unwrap();
    }
    fn get_frame_buffer(&self) -> &mut [u8] {
        unsafe {
            let ptr = self.fb.as_ptr() as *const _ as *mut u8;
            core::slice::from_raw_parts_mut(ptr, self.fb.len())
        }
    }
    fn flush(&self) {
        self.gpu.exclusive_access().flush().unwrap();
    }
}



#[no_mangle]
fn draw_point(x: i32, y: i32, color: u32) {
    let fb = GPU_DEVICE.get_frame_buffer();
    let offset = (y * VIRTGPU_XRES as i32 + x) as usize * 4;
    fb[offset] = (color >> 16) as u8;
    fb[offset + 1] = (color >> 8) as u8;
    fb[offset + 2] = color as u8;
    fb[offset + 3] = 0xff;
}

#[no_mangle]
fn gpu_flush() {
    // info!("flush");
    GPU_DEVICE.flush();
}



#[no_mangle]
extern "C" fn main(_hartid: usize, _device_tree_paddr: usize) {
    log::set_max_level(log::LevelFilter::Info);
    // let icon = IconController::new(VIRTGPU_XRES as u32,VIRTGPU_YRES as u32,vec!["f1".to_string(),"f2".to_string()],None);
    // icon.paint();

    let desk = Desktop::new(VIRTGPU_XRES as u32,VIRTGPU_YRES as u32);
    desk.paint();
    let terminal = GodTerminal::new(Size::new(300,300),Point::new(100,100));
    terminal.add_str("hello world");


    let mut snake = Snake::new();
    snake.run();
    loop {
    }
}