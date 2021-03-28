#[allow(dead_code)]
#[allow(path_statements)]
pub mod gui {
  use iced::{button, Align, Button, Column, Element, Sandbox, Text};

  #[derive(Default)]
  pub struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
  }

  #[derive(Debug, Clone, Copy)]
  pub enum Message {
    IncrementPressed,
    DecrementPressed,
  }

  impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
      Self::default()
    }

    fn title(&self) -> String {
      String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
      match message {
        Message::IncrementPressed => {
          self.value += 1;
        }
        Message::DecrementPressed => {
          self.value -= 1;
        }
      }
    }

    fn view(&mut self) -> Element<Message> {
      Column::new()
        .padding(20)
        .align_items(Align::Center)
        .push(
          Button::new(&mut self.increment_button, Text::new("Increment"))
            .on_press(Message::IncrementPressed),
        )
        .push(Text::new(self.value.to_string()).size(50))
        .push(
          Button::new(&mut self.decrement_button, Text::new("Decrement"))
            .on_press(Message::DecrementPressed),
        )
        .into()
    }
  }
}

#[allow(dead_code)]
#[allow(path_statements)]
pub mod file {
  use std::fs::File;
  use std::io::*;
  pub fn write_file(path: String, text: &[u8]) -> Result<String> {
    let mut f = File::create(path)?;
    f.write_all(text)?;
    Ok(String::from("Ok"))
  }

  pub fn read_file() -> Result<String> {
    let mut file = File::open("test.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(String::from("Ok"))
  }

  fn change_string(s: &String) -> &String {
    s
  }

  fn add(x: i32, y: i32) -> i32 {
    x + y
  }

  fn test_string(s: &String) {
    println!("{}", s);
  }

  fn test_array() {
    let a: [i32; 3] = [1, 2, 3];
    println!("{}", a[0]);

    for i in a.iter() {
      println!("{}", i);
    }

    //for i in 0u32 .. 3 {
    //	println!("{}", a[i]);
    //}
  }

  fn test_vector() {
    let v: Vec<i32> = vec![1, 2, 3];
    match v[0] {
      1 => println!("{}", v[0]),
      _ => println!("Wrong"),
    }
  }

  pub struct Point {
    x: i32,
    y: i32,
  }

  fn test_struct() {
    let p = Point { x: 10, y: 10 };
    println!("测试结构体的属性");
    println!("x is {}", p.x);
    println!("y is {}", p.y);
  }

  pub struct Writer {
    pub x: i32,
    pub y: i32,
  }

  // 方法实现
  impl Writer {
    pub fn get_x(&self) -> i32 {
      self.x
    }
    pub fn get_y(&self) -> i32 {
      self.y
    }
  }

  // 特性附加
  pub trait CanRead<T> {
    fn can_read(self);
  }

  impl CanRead<Writer> for Writer {
    fn can_read(self) {
      println!("reading x is {}", self.x);
      println!("reading y is {}", self.y);
    }
  }
}

#[cfg(test)]
pub mod tests {
  #[test]
  pub fn test_format() {
    assert_eq!(format!("Hello {:<5}!", "x"), "Hello x    !");
    assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!");
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    assert_eq!(format!("Hello {:+}!", 5), "Hello +5!");
    assert_eq!(format!("Hello {:#02}!", 5), "Hello 05!");
    assert_eq!(format!("Hello {:#05}!", 5), "Hello 00005!");
    assert_eq!(format!("{:#06x}!", 27), "0x001b!");
  }
  #[allow(dead_code)]
  #[allow(path_statements)]
  #[test]
  fn test_reference() {
    let mut n: i32 = 100;
    println!("原始n为{}", n);
    assert_eq!(n, 100);

    println!("r引用导致借用了n");
    let r: &mut i32 = &mut n; // n borrowed!!!
    assert_eq!(*r, 100);
    // assert_eq!(n, 100);

    println!("修改引用的实际值为101");
    *r = 101;
    println!("{}", "测试*r=101");
    assert_eq!(*r, 101);
    r;
    // println!("*r {}", *r);
    println!("n is back {}", n);
    assert_eq!(n, 101);
  }
  #[allow(dead_code)]
  #[allow(path_statements)]
  #[test]
  fn test_borrow() {
    let mut mine: Box<i32> = Box::new(3); // 3
    assert_eq!(*mine, 3); // 3
    *mine = 5; // 5
    assert_eq!(*mine, 5); // 5
    let mut mine_now = mine; // mine is null
    println!("{}", "mine is borrowed:");
    println!("{}", "mine now is:"); // mine 5
                                    // assert_ne!(*mine, 5); // 5
    assert_eq!(*mine_now, 5); // 5

    *mine_now += 5;
    println!("{}", "muted mine now is:"); // mine 5
    println!("{}", *mine_now);
  }

  #[test]
  fn test_method() {
    use super::file::Writer; // 用super导入单元模块
    use crate::file::CanRead; // 用crate导入单元模块, 特性功能要显式引用
    let w = Writer { x: 50, y: 50 };
    assert_eq!(w.get_x(), 50);
    assert_eq!(w.get_y(), 50);
    println!("测试特性附加");
    w.can_read();
  }
}
