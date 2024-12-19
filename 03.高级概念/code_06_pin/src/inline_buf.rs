use std::{
    marker::PhantomPinned,
    pin::{pin, Pin},
    ptr::NonNull,
};

pub struct InlineBuf {
    data: [u8; 64],
    slice: NonNull<[u8]>, // 指向上面的 data 的指针
    _pinned: PhantomPinned,
}

impl InlineBuf {
    pub fn new() -> Self {
        Self {
            data: [0; 64],
            slice: NonNull::from(&[]),
            _pinned: PhantomPinned,
        }
    }

    pub fn set_contents(self: Pin<&mut Self>, buf: &[u8]) -> bool {
        let buf_len = buf.len();
        if buf_len > self.data.len() {
            return false;
        }

        unsafe {
            let this = self.get_unchecked_mut();
            this.data[0..buf_len].copy_from_slice(buf);
            this.slice = NonNull::from(&this.data[0..buf_len]); // slice指向data
        }

        true
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { &*self.slice.as_ref() }
    }
}

impl Default for InlineBuf {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn it_works() {
    // #[allow(unused_assignments)]
    // let mut buf = Box::pin(InlineBuf::new());
    // let buf_mut = buf.as_mut();
    // buf_mut.set_contents(b"hello");

    // assert_eq!(buf.as_bytes(), b"hello");

    #[allow(unused_assignments)]
    let buf = InlineBuf::new();
    let mut pinned_buf = pin!(buf);
    pinned_buf.as_mut().set_contents(b"hello");

    assert_eq!(pinned_buf.as_bytes(), b"hello");
}

// 总结
// 1. Pin住的是指针，而不是值
// 2.



/*
use std::ptr::NonNull;

pub struct InlineBuf {
    data: [u8; 64],
    slice: NonNull<[u8]>, // 指向上面的 data 的指针
                          // _pinned: PhantomPinned
}

impl InlineBuf {
    pub fn new() -> Self {
        Self {
            data: [0; 64],
            slice: NonNull::from(&[]),
        }
    }

    pub fn set_contents(&mut self, buf: &[u8]) -> bool {
        let buf_len = buf.len();
        if buf_len > self.data.len() {
            return false;
        }

        self.data[0..buf_len].copy_from_slice(buf);
        self.slice = NonNull::from(&self.data[0..buf_len]); // slice指向data

        true
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { &*self.slice.as_ref() }
    }
}

impl Default for InlineBuf {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn it_works(){
    #[allow(unused_assignments)]
    let mut buf = InlineBuf::new();

    {
        let mut tmp_buf = InlineBuf::new();
        tmp_buf.set_contents(b"hello");
        buf = tmp_buf;

        tmp_buf = InlineBuf::new();
        tmp_buf.set_contents(b"world");

        // buf和tmp_buf的slice指向同一个地方
    }

    assert_eq!(buf.as_bytes(), b"world"); // 被后一次覆盖了
}
*/
