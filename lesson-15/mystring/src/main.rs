use std::{
    fmt,
    ops::{Deref, DerefMut},
    str,
};

const MINI_STRING_MAX_LEN: usize = 30;

struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data: data,
        }
    }

    fn push_str(&mut self, s: &str) {}
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&(self.data[..self.len as usize])).unwrap()
    }
}

impl DerefMut for MiniString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        str::from_utf8_mut(&mut (self.data[..self.len as usize])).unwrap()
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}

impl DerefMut for MyString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match *self {
            MyString::Inline(ref mut v) => v.deref_mut(),
            MyString::Standard(ref mut v) => v.deref_mut(),
        }
    }
}

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        match s.len() > MINI_STRING_MAX_LEN {
            true => MyString::Standard(s.to_owned()),
            _ => MyString::Inline(MiniString::new(s)),
        }
    }
}

impl From<String> for MyString {
    fn from(s: String) -> Self {
        match s.len() > MINI_STRING_MAX_LEN {
            true => MyString::Standard(s.to_owned()),
            _ => MyString::Inline(MiniString::new(s)),
        }
    }
}

impl MyString {
    fn push_str(&mut self, s: &str) {
        match *self {
            MyString::Inline(ref mut v) => {
                let len = v.len as usize;
                let len1 = s.len();
                if len + len1 > MINI_STRING_MAX_LEN {
                    let mut owned = v.deref().to_string();
                    owned.push_str(s);
                    *self = MyString::Standard(owned);
                } else {
                    let total = len + len1;
                    v.data[len..total].copy_from_slice(s.as_bytes());
                    v.len = total as u8;
                }
            }
            MyString::Standard(ref mut v) => v.push_str(s),
        };
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len: MyString {}, MiniString {}", len1, len2);

    let mut s1: MyString = "hello world".into();
    let mut s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();

    let new_string = "hello jpf".to_string();
    let s3: MyString = new_string.into();

    s1.push_str("aaaaa");
    println!("after push aaaaa into s1: {}", s1);

    s2.push_str("bbb");
    println!("after push bbb into s2: {}", s2);

    s1.push_str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    println!("after push 30'a into s1: {}", s1);

    match s1 {
        MyString::Inline(ref _v) => {
            println!("s1 type now is MyString::Inline")
        }
        MyString::Standard(ref _v) => {
            println!("s1 type now is MyString::Standard")
        }
    }

    println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);

    println!(
        "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );

    // assert!(s1.ends_with("world"));
    // assert!(s2.starts_with("这"));
}
