//! # Examples
//!
//! 本目录存放可运行的示例程序，每个 `.rs` 文件对应一个 `cargo run --example <name>` 可执行的示例。
//!
//! ```bash
//! cargo run --example demo
//! ```

use code_steps::step;

fn main() {
    step!["example" {
        println!("This is an example.");
    }]
}
