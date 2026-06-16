use crate::utils::sixel::encode_dynamic_image;
use anyhow::Result;
use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct AnimatedGif {
    frames: Vec<String>,
}

impl AnimatedGif {
    pub fn new(data: &[u8], max_dimension: u32) -> Result<Self> {
        let cursor = Cursor::new(data);
        let decoder = GifDecoder::new(cursor)?;
        let frames = decoder.into_frames();

        let mut encoded_frames = Vec::new();
        for frame in frames {
            let frame = frame?;
            let img = image::DynamicImage::ImageRgba8(frame.into_buffer());
            // Use the resize function from sixel.rs, but it's private.
            // I'll make resize_to_fit public too.
            let resized = crate::utils::sixel::resize_to_fit(img, max_dimension);
            if let Some(sixel) = encode_dynamic_image(&resized)? {
                encoded_frames.push(sixel);
            }
        }
        if encoded_frames.is_empty() {
            anyhow::bail!("No frames decoded or terminal lacks SIXEL support");
        }

        Ok(Self {
            frames: encoded_frames,
        })
    }

    #[must_use]
    pub fn get_frame(&self, time_ms: u128, frame_duration_ms: u128) -> &str {
        if self.frames.is_empty() {
            return "";
        }
        let idx = (time_ms / frame_duration_ms) as usize % self.frames.len();
        &self.frames[idx]
    }
}
