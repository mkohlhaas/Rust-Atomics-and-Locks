// fetch-add
// https://mara.nl/atomics/atomics.html#fetch-and-modify-operations

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
  let a = AtomicI32::new(100);
  let b = a.fetch_add(23, Relaxed);
  let c = a.load(Relaxed);

  println!("Previous value: {}", b);
  println!("Current  value: {}", c);

  assert_eq!(b, 100);
  assert_eq!(c, 123);
}
