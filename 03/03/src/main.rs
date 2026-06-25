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

  // This is a wrong assert:
  // assert!(a == b && b == c && c == d);

  assert!(a == 0 || a == 5 || a == 15);
  assert!(b == 0 || b == 5 || b == 15);
  assert!(c == 0 || c == 5 || c == 15);
  assert!(d == 0 || d == 5 || d == 15);
}

fn main() {
  thread::scope(|s| {
    s.spawn(a);
    s.spawn(b);
  });
}
