use sdl2::{
    pixels::{Color, PixelFormatEnum},
    render::{Texture, TextureAccess, TextureCreator, TextureValueError},
    video::WindowContext
};

pub struct Display<'a> {
    texture: Texture<'a>,
    color: Color
}

fn set_color_at(bytes: &mut [u8], color: &Color, index: usize) {
    if index < bytes.len() {
        bytes[index] = color.r;
        bytes[index + 1] = color.g;
        bytes[index + 2] = color.b;
    }
}

impl<'a> Display<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        width: usize,
        height: usize,
        color: Color
    ) -> Result<Self, TextureValueError> {
        let mut texture = texture_creator.create_texture(
            Some(PixelFormatEnum::RGB24),
            TextureAccess::Streaming,
            width as u32, height as u32
        )?;

        let _ = texture.with_lock(None, |bytes, _| {
            for byte in bytes {
                *byte = 0;
            }
        });

        Ok(Self { texture, color })
    }

    pub fn get_texture(&self) -> &'a Texture {
        &self.texture
    }

    pub fn update_texture(&mut self, bytes: &[u8]) {
        let _ = self.texture.with_lock(None, |pixel_bytes, pitch| {
            let height = pixel_bytes.len() / pitch;

            let bits_with_index = bytes.iter()
                .flat_map(|byte|
                    (0..8).map(
                        |n| ((1 << n) & byte) != 0
                    ).collect::<Vec<bool>>()
                )
                .enumerate()
                .map(|(index, bit)| {
                    let x = index / height; 
                    let y = height - 1 - index % height;

                    (y * pitch + x * 3, bit)
                });

            for (index, bit) in bits_with_index {
                if bit {
                    set_color_at(pixel_bytes, &self.color, index);
                } else {
                    set_color_at(pixel_bytes, &Color::BLACK, index);
                }
            }
        });
    }
}
