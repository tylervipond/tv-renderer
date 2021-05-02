use std::num::NonZeroU32;
use wgpu::{
    AddressMode, Extent3d, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d,
    SamplerDescriptor, TextureDescriptor, TextureDimension, TextureFormat, TextureUsage,
    TextureViewDescriptor,
};

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub size: wgpu::Extent3d,
    pub pixels: Vec<u8>,
}

impl Texture {
    pub fn new(device: &wgpu::Device, size: (u32, u32)) -> Self {
        let size = Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("something"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
        });
        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });
        Self {
            texture,
            view,
            sampler,
            size,
            pixels: vec![0; size.width as usize * 4 * size.height as usize],
        }
    }

    pub fn queue(&self, queue: &wgpu::Queue) {
        queue.write_texture(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            &self.pixels,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * self.size.width),
                rows_per_image: NonZeroU32::new(self.size.height),
            },
            self.size,
        );
    }

    // this needs to get clamped.
    pub fn blit(&mut self, position: (u32, u32), data: &[u8], size: (u32, u32)) {
        let dest_row_width = self.size.width * 4;
        let src_row_width = size.0 * 4;
        let start_y = position.1 * dest_row_width;
        let start_x = position.0 * 4;
        for y in 0..size.1 {
            if position.1 + y >= self.size.height {
                break;
            }
            let dest_y = start_y + y * dest_row_width;
            let src_y = y * src_row_width;
            for x in 0..src_row_width {
                if position.0 + x > self.size.width {
                    break;
                }
                let dest_x = start_x + x;
                self.pixels[dest_y as usize + dest_x as usize] = data[src_y as usize + x as usize];
            }
        }
    }

    pub fn clear(&mut self) {
        for (i, byte) in self.pixels.iter_mut().enumerate() {
            *byte = if i % 4 == 3 { 255 } else { 0 };
        }
    }
}
