# Vpsman

一个简单的示例项目,演示如何搭建一个命令行程序,可以调用用户界面程序.

## 技术栈配置

```toml
[package]
name = "vpsman"
version = "0.1.0"
authors = ["linuxing3 <linuxing3@qq.com>"]
edition = "2018"

[dependencies]
clap = { version = "3.0.0-beta.2", features = ["yaml"] }
iced = { version = "0.2", features = ["async-std", "debug"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
async-std = "1.0"
directories-next = "2.0"

[target.'cfg(target_arch = "wasm32")'.dependencies]
web-sys = { version = "0.3", features = ["Window", "Storage"] }
wasm-timer = "0.2"

[package.metadata.deb]
assets = [
    ["target/release/todos", "usr/bin/vpsman-todos", "755"],
    ["vpsman-todos.desktop", "usr/share/applications/", "644"],
]
```

## 包管理方式

## `main.rs` 文件中

导入规则: `[单元包]::[模块]::[对象]`

```rs
// 导入vpsman单元包的todos模块
use vpsman::todos::*;
```

## `lib.rs` 文件中

导出规则: `[mod]`

mod名和实际文件名最好一一对应

```rs
// 导入 todos 模块,并导出
pub mod todos; 
```

## `todos.rs`文件中

使用`pub mod todos`导出命名空间`todos` => `vps::todos`

```rs
// 导出 todos 模块
pub mod todos {
  use iced::{
    button, scrollable, text_input, Align, Application, Button, Checkbox, Column, Command,
    Container, Element, Font, HorizontalAlignment, Length, Row, Scrollable, Text,
    TextInput,
  };
  use serde::{Deserialize, Serialize};
  #[derive(Debug)]
  pub enum Todos {
    Loading,
    Loaded(State),
  }
  #[derive(Debug, Default)]
  pub struct State {
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    filter: Filter,
    tasks: Vec<Task>,
    controls: Controls,
    dirty: bool,
    saving: bool,
  }
}
```