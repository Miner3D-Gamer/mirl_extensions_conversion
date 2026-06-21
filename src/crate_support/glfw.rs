// #[cfg(not(target_arch = "wasm32"))]
// #[cfg(feature = "glfw")]
// #[cfg(feature = "std")]
// use glfw::PixelImage;

use mirl_buffer::Buffer;
use mirl_graphics::buffer_traits::SwitchInternalFormat;

// use crate::*;

// /// Convert a Buffer into a `glfw::PixelImage`
// #[inline(always)]
// #[must_use]
// #[allow(clippy::cast_precision_loss)]
// #[allow(clippy::cast_possible_truncation)]
// #[allow(clippy::cast_sign_loss)]
// fn buffer_to_pixel_image(
//     buffer: mirl_buffer::Buffer,
// ) -> Option<glfw::PixelImage> {
// }
// /// Convert a `glfw::PixelImage` into a Buffer
// #[inline(always)]
// #[must_use]
// #[allow(clippy::missing_panics_doc, clippy::unwrap_used)]
// pub fn pixel_image_to_buffer(pixel_image: glfw::PixelImage) -> Buffer {
// }

impl crate::TryFromPatch<Buffer> for glfw::PixelImage {
    fn try_from_value(buffer: Buffer) -> Option<Self> {
        let mut buffer = buffer;
        buffer.switch_red_and_blue();
        Some(Self {
            width: u32::try_from(buffer.width).ok()?,
            height: u32::try_from(buffer.height).ok()?,
            pixels: buffer.data,
        })
    }
}

impl crate::FromPatch<glfw::PixelImage> for Buffer {
    fn from_value(pixel_image: glfw::PixelImage) -> Self {
        let mut buffer = unsafe {
            Self::new(
                (pixel_image.width as usize, pixel_image.height as usize),
                pixel_image.pixels,
            )
            .unwrap_unchecked()
        };
        buffer.switch_red_and_blue();
        buffer
    }
}
