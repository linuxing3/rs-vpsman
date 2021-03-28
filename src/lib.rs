pub mod todos; 

pub mod file;

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
}
