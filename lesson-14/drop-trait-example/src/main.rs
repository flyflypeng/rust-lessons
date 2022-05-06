use std::{fmt, slice};

#[derive(Clone)]
struct RawBuffer {
    ptr: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for RawBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let slice = vec.into_boxed_slice();
        Self {
            len: slice.len(),
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}

impl Drop for RawBuffer {
    #[inline]
    fn drop(&mut self) {
        let data = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len)) };
        drop(data)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl fmt::Debug for RawBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}: {:?}", self.ptr, self.len)
    }
}

fn use_buffer(buf: RawBuffer) {
    println!("buf to die: {:?}", buf);
}

fn main() {
    let data: Vec<u8> = vec![1, 2, 3, 4];
    let buf: RawBuffer = data.into();

    use_buffer(buf);

    // println!("buf: {:?}", buf);
}
