# fit-rust

[中文](./README_zh.md)

`fit-rust` is a Rust library designed for reading, writing, and merging [Garmin Flexible and Interoperable Data Transfer (FIT)](https://developer.garmin.com/fit/protocol/) protocol files. It offers an efficient and type-safe approach to handle FIT files, suitable for sports and health data analysis and manipulation.

## Features

- Read FIT files, parsing them into Rust-friendly structures.
- Write data to FIT files.
- Merge data from multiple FIT files.

## Installation

Add `fit-rust` to your Cargo project by including it in your `Cargo.toml`:

```toml
[dependencies]
fit-rust = "0.1"
```

## Usage

Here are some basic examples of how to use the fit-rust library.

**Reading FIT Files**

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

**Writing FIT Files**
```rust
use fit_rust::Fit;
use std::fs;

fn main() {
    let file = fs::read("tests/test.fit").unwrap();
    let fit: Fit = Fit::read(file).unwrap();
    fit.write("tests/write-test.fit").unwrap();
}
```
See more examples: [xingzhe](https://github.com/zzyandzzy/igps_tools/tree/main/xingzhe)

**Merging FIT Files**
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

## Contributing

Contributions to fit-rust are welcome. Feel free to submit Pull Requests or create Issues to discuss new features or report bugs.

## License

fit-rust is released under the MIT License.

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
## Acknowledgments

- [fit-rs](https://github.com/richardbrodie/fit-rs)
- [bin-rw](https://github.com/jam1garner/binrw)
