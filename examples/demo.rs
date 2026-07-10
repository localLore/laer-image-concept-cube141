//! CubeImage141 demo —— 展示基本 API 用法

use code_steps::{ignore, step};
use laer_image_concept::color::Color;
use laer_image_concept::image::Image;
use laer_image_concept_cube141::CubeImage141;

fn main() {
    step!["::new()" {
        let blue = Color::new(0, 100, 200, 255);
        let img = CubeImage141::new(blue);
        ignore![("std::println") {
            println!("{img}");
            println!("width: {}, height: {}", img.width(), img.height());
        }];
    }];

    step![".set_pixel()" {
        let mut img = CubeImage141::new(Color::new(0, 100, 200, 255));
        let red = Color::new(255, 0, 0, 255);
        img.set_pixel(6, 6, red).expect("set_pixel failed");
        ignore![("std::println") {
            println!("center pixel: {}", img.pixels[6][6]);
        }];
    }];

    step![".save() & .show()" {
        let mut img = CubeImage141::new(Color::new(0, 100, 200, 255));
        let white = Color::new(255, 255, 255, 255);
        for i in 0..13 {
            img.set_pixel(i, 6, white).unwrap();
            img.set_pixel(6, i, white).unwrap();
        }
        img.save("/tmp/cube_demo.png").expect("save failed");
        ignore![("std::println") {
            println!("saved to /tmp/cube_demo.png");
        }];
        img.show().expect("show failed");
        ignore![("std::println") {
            println!("opened in Preview.app");
        }];
    }];

    step![".mix() with laer Image" {
        let mut base = CubeImage141::new(Color::new(0, 100, 200, 255));
        // 用通用的 laer Image 创建覆盖层
        let red_overlay = Image::new(6, 6, Color::new(255, 0, 0, 128));
        base.mix(&red_overlay, 1, 1).expect("mix failed");
        base.save("/tmp/cube_mix_demo.png").expect("save failed");
        ignore![("std::println") {
            println!("mixed result saved to /tmp/cube_mix_demo.png");
        }];
    }];

    step![".to_image() & ::from_image()" {
        let cube = CubeImage141::new(Color::new(100, 200, 50, 255));
        let img: Image = cube.to_image();
        ignore![("std::println") {
            println!("converted to Image: {img}");
        }];
        let cube2 = CubeImage141::from_image(&img);
        ignore![("std::println") {
            println!("converted back: {cube2}");
        }];
    }];
}
