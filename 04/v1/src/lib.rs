// https://mara.nl/atomics/building-spinlock.html#a-minimal-implementation

// A spin lock is a mutex that when attempting to lock an already locked mutex will result in
// busy-looping/spinning.

#![allow(unused_imports)]

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct SpinLock {
  locked: AtomicBool,
}

impl SpinLock {
  pub const fn new() -> Self {
    Self {
      locked: AtomicBool::new(UNLOCKED),
    }
  }

  pub fn lock(&self) {
    // the next Acquire in another thread will see our Release in unlock(…)
    while self.locked.swap(LOCKED, Acquire) {
      std::hint::spin_loop(); // tells the processor that we’re spinning while waiting for something to change
    }
  }

  // Alternatively with a compare-and-exchange operation.
  // pub fn lock(&self) {
  //   while self
  //     .locked
  //     .compare_exchange_weak(UNLOCKED, LOCKED, Acquire, Relaxed)
  //     .is_err()
  //   {
  //     std::hint::spin_loop();
  //   }
  // }

  pub fn unlock(&self) {
    self.locked.store(UNLOCKED, Release);
  }
}

impl Default for SpinLock {
  fn default() -> Self {
    Self::new()
  }
}

#[test]
fn main() {
  use std::thread;

  let sl = SpinLock::new();
  sl.lock();
  sl.unlock();
}
