use alloc::sync::Arc;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{OriginDimensions, Point, RgbColor, Size},
};
use spin::Mutex;
use crate::{GPU_DEVICE, GPUDevice};

#[derive(Clone)]
pub struct Graphics {
    pub size: Size,
    pub point: Point,
    pub gpu: Arc<Mutex<dyn GPUDevice>>,
}

impl Graphics {
    pub fn new(size: Size, point: Point) -> Self {
        Self { size, point, gpu:GPU_DEVICE.get().unwrap().clone() }
    }
}

impl  OriginDimensions for Graphics {
    fn size(&self) -> Size {
        self.size
    }
}

impl DrawTarget for Graphics {
    type Color = Rgb888;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        let mut gpu = self.gpu.lock();
        pixels.into_iter().for_each(|px| {
            let color = px.1.b() as u32 | (px.1.g() as u32) << 8 | (px.1.r() as u32) << 16;
            gpu.draw_point(self.point.x + px.0.x, self.point.y + px.0.y, color)
        });
        gpu.flush();
        Ok(())
    }
}
