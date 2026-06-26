// https://mara.nl/atomics/building-spinlock.html#building-safe-spinlock

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct SpinLock<T> {
  locked: AtomicBool,
  value: UnsafeCell<T>,
}

pub struct Guard<'a, T> {
  lock: &'a SpinLock<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

impl<T> Deref for Guard<'_, T> {
  type Target = T;
  fn deref(&self) -> &T {
    // Safety: The very existence of this Guard guarantees we've exclusively locked the lock.
    unsafe { &*self.lock.value.get() }
  }
}

impl<T> DerefMut for Guard<'_, T> {
  fn deref_mut(&mut self) -> &mut T {
    // Safety: The very existence of this Guard guarantees we've exclusively locked the lock.
    unsafe { &mut *self.lock.value.get() }
  }
}

impl<T> Drop for Guard<'_, T> {
  fn drop(&mut self) {
    self.lock.locked.store(UNLOCKED, Release);
  }
}

impl<T> SpinLock<T> {
  pub const fn new(value: T) -> Self {
    Self {
      locked: AtomicBool::new(UNLOCKED),
      value: UnsafeCell::new(value),
    }
  }

  pub fn lock(&self) -> Guard<'_, T> {
    while self.locked.swap(LOCKED, Acquire) {
      std::hint::spin_loop();
    }
    Guard { lock: self }
  }
}

#[test]
fn main() {
  use std::thread;

  let x = SpinLock::new(Vec::new());

  thread::scope(|s| {
    s.spawn(|| x.lock().push(1));
    s.spawn(|| {
      let mut g = x.lock();
      g.push(2);
      g.push(3);
    });
  });

  let g = x.lock();
  assert!(g.as_slice() == [1, 2, 3] || g.as_slice() == [2, 3, 1]);
}
