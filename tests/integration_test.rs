//! 集成测试 —— CubeImage141 API

use laer_image_concept::color::Color;
use laer_image_concept::image::Image;
use laer_image_concept_cube141::CubeImage141;

#[test]
fn new_and_default() {
    let img = CubeImage141::new(Color::new(255, 0, 0, 255));
    assert_eq!(img.width(), 13);
    assert_eq!(img.height(), 13);
    for y in 0..13 {
        for x in 0..13 {
            let p = img.pixels[y][x];
            assert_eq!(p.r, 255);
            assert_eq!(p.g, 0);
            assert_eq!(p.b, 0);
            assert_eq!(p.a, 255);
        }
    }
}

#[test]
fn default_is_transparent() {
    let img = CubeImage141::default();
    for row in &img.pixels {
        for p in row {
            assert_eq!(p.r, 0);
            assert_eq!(p.g, 0);
            assert_eq!(p.b, 0);
            assert_eq!(p.a, 0);
        }
    }
}

#[test]
fn set_pixel_and_bounds() {
    let mut img = CubeImage141::default();
    let red = Color::new(255, 0, 0, 255);

    assert!(img.set_pixel(0, 0, red).is_ok());
    assert!(img.set_pixel(12, 12, red).is_ok());
    assert_eq!(img.pixels[0][0].r, 255);

    assert!(img.set_pixel(13, 0, red).is_err());
    assert!(img.set_pixel(0, 13, red).is_err());
}

#[test]
fn mix_with_laer_image() {
    // 半透明蓝色底图
    let translucent_blue = Color::new(0, 100, 200, 100);
    let mut base = CubeImage141::new(translucent_blue);

    // 用通用的 laer Image 作为覆盖层
    let opaque_red = Color::new(255, 0, 0, 255);
    let overlay = Image::new(5, 5, opaque_red);

    base.mix(&overlay, 2, 2).expect("mix failed");

    // 混合区域应有红色分量
    let mixed = base.pixels[4][4];
    assert!(mixed.r > 0, "should have red component after mixing");
}

#[test]
fn mix_out_of_bounds() {
    let mut base = CubeImage141::default();
    let overlay = Image::new(5, 5, Color::default());

    // 超大 offset
    assert!(base.mix(&overlay, 9, 0).is_err()); // 5+9=14 > 13
    assert!(base.mix(&overlay, 0, 9).is_err());
    // 合法 offset
    assert!(base.mix(&overlay, 8, 8).is_ok()); // 5+8=13 ≤ 13
}

#[test]
fn save_and_open_roundtrip() {
    let blue = Color::new(0, 100, 200, 255);
    let original = CubeImage141::new(blue);

    let path = "/tmp/cube_test_roundtrip.png";
    original.save(path).expect("save failed");

    let loaded = CubeImage141::open(path).expect("open failed");
    assert_eq!(loaded.width(), 13);
    assert_eq!(loaded.height(), 13);

    let p = loaded.pixels[0][0];
    assert_eq!(p.r, 0);
    assert_eq!(p.g, 100);
    assert_eq!(p.b, 200);
    assert_eq!(p.a, 255);
}

#[test]
fn to_image_and_from_image_roundtrip() {
    let red = Color::new(255, 0, 0, 255);
    let cube = CubeImage141::new(red);

    let img = cube.to_image();
    assert_eq!(img.width(), 13);
    assert_eq!(img.height(), 13);

    let cube2 = CubeImage141::from_image(&img);
    assert_eq!(cube2.pixels[0][0].r, 255);
}

#[test]
fn from_image_trims_large_image() {
    let large = Image::new(20, 20, Color::new(0, 255, 0, 255));
    let cube = CubeImage141::from_image(&large);
    assert_eq!(cube.width(), 13);
    assert_eq!(cube.height(), 13);
    assert_eq!(cube.pixels[0][0].g, 255);
}
