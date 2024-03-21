//! 映射表

/// 返回符号优先级
pub const fn prec(x:&[u8])-> u8 {
  match x {
    b"-."|b"-:" => 16,
    b"is"=> 15,
    b"::"|b"." => 14,
    b"("|b"[" => 13, // 代指调用和索引
    // unary => 12
    b"*" | b"%" | b"/" => 11, 
    b"+" | b"-" => 10, 
    b"<<"|b">>" => 9,
    b"&" => 8,
    b"^" => 7,
    b"|" => 6,
    b"=="|b"!="|b"<"|b">"|b"<="|b">=" => 5,
    b"&&" => 4,
    b"||" => 3,
    b"="|b"+="|b"-="|b"*="|b"/="|b"%="|b"&="|b"|="|b"^="|b"<<="|b">>=" => 2,
    _=> 0
  }
}
pub const PREC_UNARY:u8 = 12;


/// 转义符表
pub const fn escape(c:u8)-> u8 {
  match c {
    b'n'=> b'\n',
    b'r'=> b'\r',
    b't'=> b'\t',
    b'\\'=> b'\\',
    b'0'=> 0,
    b'`'=> b'`',
    _=> 255
  }
}

