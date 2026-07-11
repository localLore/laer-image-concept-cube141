# CubeImage141

在 $13 \times 13$ 的位图中，仅仅有 $141$ 个有效像素。

这个模型用于 laer 的空间图形分支。

---

直接定义位图，外层为行，固定 $13 \times 13$ 大小。

```rust
struct CubeImage141 {
    pixels: [[Color; 13]; 13]
}
```

## 构造

以指定背景色创建一张 $13 \times 13$ 的图像。

```rust
fn new(background_color: Color) -> Self {
    /* ... */
}
```

从通用的 `laer_image_concept::Image` 转换（自动裁剪或填充至 $13 \times 13$）。

```rust
fn from_image(img: &Image) -> Self {
    /* ... */
}
```

转换为 `laer_image_concept::Image`，用于与通用位图互操作。

```rust
fn to_image(&self) -> Image {
    /* ... */
}
```

## 制图

可以 `.set_pixel()` 以指定某个点的像素。

```rust
fn set_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), &'static str> {
    /* ... */
}
```

可以 `.mix()` 以叠加另一张通用的 `laer_image_concept::Image` 位图。

```rust
fn mix(&mut self, other: &Image, offset_x: usize, offset_y: usize) -> Result<(), &'static str> {
    /* ... */
}
```

## 文件存储

保存图像到指定路径。

```rust
fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
    /* ... */
}
```

从文件加载图像（取前 $13 \times 13$ 像素）。

```rust
fn open(path: &str) -> Result<Self, Box<dyn Error>> {
    /* ... */
}
```

在系统默认图片查看器中显示图像。

```rust
fn show(&self) -> Result<(), Box<dyn Error>> {
    /* ... */
}
```
