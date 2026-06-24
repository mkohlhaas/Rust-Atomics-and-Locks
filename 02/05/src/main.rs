// fetch-add
// Fetch-and-Modify Operations
// https://mara.nl/atomics/atomics.html#fetch-and-modify-operations

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
  let a = AtomicI32::new(42);
  let b = a.fetch_add(10, Relaxed);
  a.fetch_add(10, Relaxed);
  let c = a.load(Relaxed);

  println!("Previous value: {}", b);
  println!("Current  value: {}", c);

  assert_eq!(b, 42);
  assert_eq!(c, 62);
}
