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

  fn test_method() {
    // use super::file::Writer; // 用super导入单元模块
    // use crate::file::CanRead; // 用crate导入单元模块, 特性功能要显式引用
    let w = Writer { x: 50, y: 50 };
    assert_eq!(w.get_x(), 50);
    assert_eq!(w.get_y(), 50);
    println!("测试特性附加");
    w.can_read();
  }
}
