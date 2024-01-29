//! 字符串缓存池简单实现
//! 
//! 对比和传递时只需要确认瘦指针，而无需带着Vec乱跑
//! 
//! 但字符串越短，性能收益越小

use std::collections::HashSet;

static mut POOL:Option<HashSet<Vec<u8>>> = None;

pub fn init() {
  unsafe{POOL = Some(HashSet::with_capacity(64));}
}

/// 将字符串缓存为指针
pub fn intern(s:&[u8])-> Interned {
  let p = unsafe{POOL.as_mut().unwrap()};
  Interned { p:p.get_or_insert(s.to_vec()) as *const Vec<u8> }
}

/// 字符串缓存
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Interned {
  p: *const Vec<u8>
}
impl Interned {
  pub fn vec(&self)-> &Vec<u8> {
    unsafe{&*self.p}
  }
  pub fn str(&self)-> String {
    String::from_utf8_lossy(self.vec()).into_owned()
  }
  pub const fn ptr(&self)-> *const Vec<u8> {
    self.p as *const Vec<u8>
  }
}

impl std::fmt::Debug for Interned {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    f.write_fmt(format_args!("\"{}\"",self.str()))
  }
}
impl std::fmt::Display for Interned {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = unsafe {String::from_utf8_lossy(&**self.p)};
    f.write_str(&s)
  }
}
