use serde::Deserialize;
use std::collections::HashMap;

/// Matches Aseprite's JSON export format (array mode)
#[derive(Debug, Deserialize)]
pub struct AsepriteAtlas {
    pub frames: Vec<AsepriteFrame>,
    pub meta: AsepriteMeta,
}

#[derive(Debug, Deserialize)]
pub struct AsepriteFrame {
    pub filename: String,
    pub frame: AsepriteRect,
    pub duration: u32, // milliseconds
}

#[derive(Debug, Deserialize)]
pub struct AsepriteRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Deserialize)]
pub struct AsepriteMeta {
    pub image: String,
    #[serde(rename = "frameTags")]
    pub frame_tags: Vec<AsepriteTag>,
    pub size: AsepriteSize,
}

#[derive(Debug, Deserialize)]
pub struct AsepriteTag {
    pub name: String,
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Deserialize)]
pub struct AsepriteSize {
    pub w: u32,
    pub h: u32,
}

/// A loaded sprite sheet with raw RGBA bytes
pub struct SpriteSheet {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub atlas: AsepriteAtlas,
    /// animation name -> (start_frame, end_frame)
    tag_map: HashMap<String, (usize, usize)>,
}

impl SpriteSheet {
    pub fn load(png_path: &str, json_path: &str) -> anyhow::Result<Self> {
        let img = image::open(png_path)?.to_rgba8();
        let (width, height) = img.dimensions();
        let pixels = img.into_raw();

        let json = std::fs::read_to_string(json_path)?;
        let atlas: AsepriteAtlas = serde_json::from_str(&json)?;

        let mut tag_map = HashMap::new();
        for tag in &atlas.meta.frame_tags {
            tag_map.insert(tag.name.clone(), (tag.from, tag.to));
        }

        Ok(Self {
            pixels,
            width,
            height,
            atlas,
            tag_map,
        })
    }

    pub fn get_tag(&self, name: &str) -> Option<(usize, usize)> {
        self.tag_map.get(name).copied()
    }

    /// Blit a single frame into a destination RGBA buffer of `dest_w x dest_h`
    pub fn blit_frame(&self, frame_idx: usize, dest: &mut [u8], dest_w: u32, dest_h: u32) {
        let frame = &self.atlas.frames[frame_idx];
        let src_x = frame.frame.x;
        let src_y = frame.frame.y;
        let fw = frame.frame.w;
        let fh = frame.frame.h;

        // Center the sprite in the destination buffer
        let off_x = (dest_w.saturating_sub(fw) / 2) as i32;
        let off_y = (dest_h.saturating_sub(fh) / 2) as i32;

        for row in 0..fh {
            for col in 0..fw {
                let sx = (src_x + col) as usize;
                let sy = (src_y + row) as usize;
                let src_i = (sy * self.width as usize + sx) * 4;

                let dx = off_x + col as i32;
                let dy = off_y + row as i32;
                if dx < 0 || dy < 0 || dx >= dest_w as i32 || dy >= dest_h as i32 {
                    continue;
                }
                let dst_i = (dy as usize * dest_w as usize + dx as usize) * 4;

                dest[dst_i..dst_i + 4].copy_from_slice(&self.pixels[src_i..src_i + 4]);
            }
        }
    }
}

/// Drives frame advancement for a single animation clip
pub struct Animator {
    pub current_frame: usize,
    clip_start: usize,
    clip_end: usize,
    elapsed_ms: u32,
}

impl Animator {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            current_frame: start,
            clip_start: start,
            clip_end: end,
            elapsed_ms: 0,
        }
    }

    pub fn set_clip(&mut self, start: usize, end: usize) {
        if self.clip_start != start || self.clip_end != end {
            self.clip_start = start;
            self.clip_end = end;
            self.current_frame = start;
            self.elapsed_ms = 0;
        }
    }

    pub fn update(&mut self, dt_ms: u32, frames: &[AsepriteFrame]) {
        self.elapsed_ms += dt_ms;
        let frame_duration = frames[self.current_frame].duration;
        if self.elapsed_ms >= frame_duration {
            self.elapsed_ms -= frame_duration;
            self.current_frame += 1;
            if self.current_frame > self.clip_end {
                self.current_frame = self.clip_start;
            }
        }
    }
}
