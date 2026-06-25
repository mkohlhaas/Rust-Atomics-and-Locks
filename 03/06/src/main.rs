// release-aquire
// https://mara.nl/atomics/memory-ordering.html#release-and-acquire-ordering

// Release applies to store (enforced by the compiler).
// Acquire applies to load  (enforced by the compiler).

// NOTE:
// Release store: Can't reorder PREVIOUS   memory operations to be AFTER  the store.
// Acquire load:  Can't reorder SUBSEQUENT memory operations to be BEFORE the load.

// NOTE:
// Acquire-Release Semantics make only sense:
//  - as a pair
//  - if some other data is shared

#![allow(unused_imports)]

use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::thread;
use std::time::Duration;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
  thread::spawn(|| {
    DATA.store(123, Relaxed);
    READY.store(true, Release); // NOTE: Everything from before this store …
  });

  while !READY.load(Acquire) {
    // NOTE: … is visible after this loads `true`.

    // thread::sleep(Duration::from_millis(100));
    println!("waiting...");
  }

  let data = DATA.load(Relaxed);
  assert_eq!(data, 123);

  println!("{}", data);
}
