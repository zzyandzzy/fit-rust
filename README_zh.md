# fit-rust

[English](./README_zh.md)

`fit-rust` 是一个用 Rust 语言编写的库，用于读取、写入和合并 Garmin 的 [Flexible and Interoperable Data Transfer (FIT)](https://developer.garmin.com/fit/protocol/) 文件。该库提供了一种高效且类型安全的方式来处理 FIT 文件，适用于运动和健康数据的分析和处理。

## 功能

- 读取 FIT 文件，解析为 Rust 可操作的结构。
- 写入数据到 FIT 文件。
- 合并多个 FIT 文件中的数据。

## 安装

将 `fit-rust` 添加到你的 Cargo 项目中：

```toml
[dependencies]
fit-rust = "0.1"
```

## 使用
以下是使用 fit-rust 库进行基本操作的示例。

**读取 FIT 文件**

```rust
use fit_rust::protocol::message_type::MessageType;
use fit_rust::protocol::FitMessage;
use fit_rust::Fit;
use std::fs;

fn main() {
    let file = fs::read("tests/test.fit").unwrap();
    let fit: Fit = Fit::read(file).unwrap();
    for data in &fit.data {
        match data {
            FitMessage::Definition(msg) => {
                println!("Definition: {:?}", msg.data);
            }
            FitMessage::Data(msg) if msg.data.message_type == MessageType::FileId => {
                println!("FileId data type: {:?}", msg.data);
            }
            FitMessage::Data(_) => {
                // Other data type is here
            }
        }
    }
}
```

**写入 FIT 文件**
```rust
use fit_rust::Fit;
use std::fs;

fn main() {
    let file = fs::read("tests/test.fit").unwrap();
    let fit: Fit = Fit::read(file).unwrap();
    fit.write("tests/write-test.fit").unwrap();
}
```

更多例子看: [xingzhe](https://github.com/zzyandzzy/igps_tools/tree/main/xingzhe)

**合并 FIT 文件**
```rust
use fit_rust::Fit;

fn main() {
    Fit::merge(
        vec![
            "tests/ride-0-2023-09-29-12-49-21.fit",
            "tests/ride-0-2023-09-29-09-41-54.fit",
        ],
        "tests/merge.fit",
    )
        .unwrap();
}
```

## 贡献

如果你想为 fit-rust 贡献代码，欢迎提交 Pull Request 或创建 Issue 讨论新功能或发现的问题。

## 许可证

fit-rust 采用 MIT 许可证

```text
MIT License

Copyright (c) 2020 intent

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
## 致谢

- [fit-rs](https://github.com/richardbrodie/fit-rs)
- [bin-rw](https://github.com/jam1garner/binrw)