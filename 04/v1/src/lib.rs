// https://mara.nl/atomics/building-spinlock.html#a-minimal-implementation

#![allow(unused_imports)]

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

pub struct SpinLock {
  locked: AtomicBool,
}

impl SpinLock {
  pub const fn new() -> Self {
    Self {
      locked: AtomicBool::new(false),
    }
  }

  pub fn lock(&self) {
    // acquire lock
    while self.locked.swap(true, Acquire) {
      std::hint::spin_loop(); // tells the processor that we’re spinning while waiting for something to change
    }
  }

  // Alternatively with a compare-and-exchange operation.
  // pub fn lock(&self) {
  //   while self
  //     .locked
  //     .compare_exchange_weak(false, true, Acquire, Relaxed)
  //     .is_err()
  //   {
  //     std::hint::spin_loop();
  //   }
  // }

  pub fn unlock(&self) {
    self.locked.store(false, Release); // release lock 
  }
}

impl Default for SpinLock {
  fn default() -> Self {
    Self::new()
  }
}
