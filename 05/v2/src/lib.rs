use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};

pub struct Channel<T> {
  message: UnsafeCell<MaybeUninit<T>>,
  ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
  pub const fn new() -> Self {
    Self {
      message: UnsafeCell::new(MaybeUninit::uninit()),
      ready: AtomicBool::new(false),
    }
  }

  /// Safety: Only call this once!
  pub unsafe fn send(&self, message: T) {
    unsafe {
      (*self.message.get()).write(message);
      self.ready.store(true, Release);
    }
  }

  pub fn is_ready(&self) -> bool {
    self.ready.load(Acquire)
  }

  /// Safety: Only call this once,
  /// and only after is_ready() returns true!
  pub unsafe fn receive(&self) -> T {
    unsafe { (*self.message.get()).assume_init_read() }
  }
}

#[test]
fn main() {
  use std::thread;

  let channel = Channel::new();
  let t = thread::current();

  thread::scope(|s| {
    s.spawn(|| {
      unsafe { channel.send("hello world!") };
      t.unpark();
    });

    while !channel.is_ready() {
      thread::park();
    }

    let msg = unsafe { channel.receive() };

    assert_eq!(msg, "hello world!");
  });
}
