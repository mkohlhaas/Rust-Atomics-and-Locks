// total-modification-order
// https://mara.nl/atomics/memory-ordering.html#relaxed

// Atomic operations using relaxed memory ordering do not provide any happens-before relationship.
// They do guarantee a TOTAL MODIFICATION ORDER.
// This means that all modifications of the same atomic variable happen in an order that is the same
// from the perspective of every single thread.

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

fn a() {
  X.fetch_add(5, Relaxed);
  X.fetch_add(10, Relaxed);
}

fn b() {
  let a = X.load(Relaxed);
  let b = X.load(Relaxed);
  let c = X.load(Relaxed);
  let d = X.load(Relaxed);
  println!("{a} {b} {c} {d}");
  // ⚠️ assertion is wrong!
  // assert!(a == b && b == c && c == d);
}

fn main() {
  thread::scope(|s| {
    s.spawn(a);
    s.spawn(b);
  });
}
