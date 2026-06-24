// lock
// https://mara.nl/atomics/memory-ordering.html#example-locking

// It's like a mutex:
// 1. Acquire
// 2. Access
// 3. Release

#![allow(static_mut_refs)]

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
  // 1. Acquire
  match LOCKED.compare_exchange(false, true, Acquire, Relaxed) {
    Ok(_) => {
      // 2. Access
      // Safety: We hold the exclusive lock, so nothing else is accessing DATA.
      unsafe { DATA.push('!') };
      // 3. Release
      LOCKED.store(false, Release);
    }
    Err(_) => {
      println!("Already locked.")
    }
  }
}

fn main() {
  thread::scope(|s| {
    for _ in 0..100 {
      s.spawn(f);
    }
  });

  // DATA now contains at least one exclamation mark (and maybe more).
  assert!(unsafe { DATA.len() } > 0);
  assert!(unsafe { DATA.chars().all(|c| c == '!') });

  dbg!(unsafe { &DATA });
}
