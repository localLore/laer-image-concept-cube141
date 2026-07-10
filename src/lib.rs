use laer_image_concept::color::Color;
use laer_image_concept::image::Image;

use std::error::Error;
use std::fmt::{Display, Formatter};

/// 一个固定 13×13 像素的方形图像
pub struct CubeImage141 {
    pub pixels: [[Color; 13]; 13],
}

impl Default for CubeImage141 {
    fn default() -> Self {
        CubeImage141 {
            pixels: [[Color::default(); 13]; 13],
        }
    }
}

impl Display for CubeImage141 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CubeImage141 {{ width: {}, height: {} }}",
            self.width(),
            self.height()
        )
    }
}

impl CubeImage141 {
    pub const WIDTH: usize = 13;
    pub const HEIGHT: usize = 13;

    // ── 构造 ──────────────────────────────────────────────

    /// 以指定背景色创建一张 13×13 的图像
    pub fn new(background_color: Color) -> Self {
        CubeImage141 {
            pixels: [[background_color; 13]; 13],
        }
    }

    /// 从 laer_image_concept::Image 转换（裁剪或填充至 13×13）
    pub fn from_image(img: &Image) -> Self {
        let mut pixels = [[Color::default(); 13]; 13];
        let w = img.width().min(Self::WIDTH);
        let h = img.height().min(Self::HEIGHT);
        for y in 0..h {
            for x in 0..w {
                pixels[y][x] = img.pixels[y][x];
            }
        }
        CubeImage141 { pixels }
    }

    // ── 查询 ──────────────────────────────────────────────

    /// 图像宽度，始终为 13
    pub fn width(&self) -> usize {
        Self::WIDTH
    }

    /// 图像高度，始终为 13
    pub fn height(&self) -> usize {
        Self::HEIGHT
    }

    // ── 像素操作 ──────────────────────────────────────────

    /// 设置指定坐标的像素颜色
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), &'static str> {
        if x >= Self::WIDTH {
            return Err("x out of bounds");
        }
        if y >= Self::HEIGHT {
            return Err("y out of bounds");
        }
        self.pixels[y][x] = color;
        Ok(())
    }

    /// 将一张通用的 laer Image 混合到当前图像上（alpha 叠加），offset 为左上角偏移。
    /// self 在上层（与 laer-image-concept 保持一致）。
    pub fn mix(&mut self, other: &Image, offset_x: usize, offset_y: usize) -> Result<(), &'static str> {
        if offset_x + other.width() > self.width() {
            return Err("other image exceeds horizontal bounds");
        }
        if offset_y + other.height() > self.height() {
            return Err("other image exceeds vertical bounds");
        }

        for y in 0..other.height() {
            for x in 0..other.width() {
                let tx = offset_x + x;
                let ty = offset_y + y;
                self.pixels[ty][tx] = self.pixels[ty][tx].mix(&other.pixels[y][x]);
            }
        }

        Ok(())
    }

    // ── I/O（委托给 laer_image_concept::Image）───────────

    /// 转换为 laer_image_concept::Image
    pub fn to_image(&self) -> Image {
        let mut img = Image::new(Self::WIDTH, Self::HEIGHT, Color::default());
        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                img.pixels[y][x] = self.pixels[y][x];
            }
        }
        img
    }

    /// 保存图像到指定路径（格式由扩展名决定，如 .png / .jpg）
    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        self.to_image().save(path)
    }

    /// 从文件加载图像，取前 13×13 像素
    pub fn open(path: &str) -> Result<Self, Box<dyn Error>> {
        let img = Image::open(path)?;
        Ok(Self::from_image(&img))
    }

    /// 在系统默认图片查看器中显示图像（macOS 上使用 Preview.app）
    pub fn show(&self) -> Result<(), Box<dyn Error>> {
        self.to_image().show()
    }
}
