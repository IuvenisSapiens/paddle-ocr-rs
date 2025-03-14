﻿use opencv::core::Mat;
use opencv::prelude::MatTraitConst;

#[derive(Debug)]
pub struct ScaleParam {
    pub src_width: i32,
    pub src_height: i32,
    pub dst_width: i32,
    pub dst_height: i32,
    pub scale_width: f32,
    pub scale_height: f32,
}

impl ScaleParam {
    pub fn new(
        src_width: i32,
        src_height: i32,
        dst_width: i32,
        dst_height: i32,
        scale_width: f32,
        scale_height: f32,
    ) -> Self {
        Self {
            src_width,
            src_height,
            dst_width,
            dst_height,
            scale_width,
            scale_height,
        }
    }

    pub fn get_scale_param(src: &Mat, target_size: i32) -> Self {
        let src_width = src.cols();
        let src_height = src.rows();
        let mut dst_width;
        let mut dst_height;

        let ratio = if src_width > src_height {
            target_size as f32 / src_width as f32
        } else {
            target_size as f32 / src_height as f32
        };

        dst_width = (src_width as f32 * ratio) as i32;
        dst_height = (src_height as f32 * ratio) as i32;

        if dst_width % 32 != 0 {
            dst_width = (dst_width / 32) * 32;
            dst_width = dst_width.max(32);
        }
        if dst_height % 32 != 0 {
            dst_height = (dst_height / 32) * 32;
            dst_height = dst_height.max(32);
        }

        let scale_width = dst_width as f32 / src_width as f32;
        let scale_height = dst_height as f32 / src_height as f32;

        Self::new(
            src_width,
            src_height,
            dst_width,
            dst_height,
            scale_width,
            scale_height,
        )
    }
}

impl std::fmt::Display for ScaleParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "src_width:{},src_height:{},dst_width:{},dst_height:{},scale_width:{},scale_height:{}",
            self.src_width,
            self.src_height,
            self.dst_width,
            self.dst_height,
            self.scale_width,
            self.scale_height
        )
    }
}
