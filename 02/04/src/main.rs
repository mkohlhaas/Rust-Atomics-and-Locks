// lazy-init
// Atomic Load and Store Operations
// https://mara.nl/atomics/atomics.html#example-lazy-init

use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn calculate_x() -> u64 {
  println!("Calculating...");
  thread::sleep(Duration::from_secs(1));
  42
}

fn get_x() -> u64 {
  static X: AtomicU64 = AtomicU64::new(0);

  let mut x = X.load(Relaxed);
  if x == 0 {
    x = calculate_x();
    X.store(x, Relaxed);
  }
  x
}

fn main() {
  // There could be a race.
  // But doesn't matter for this example.
  // You could use sync::Once or sync::OnceLock.
  dbg!(get_x());
  dbg!(get_x());

  println!("Done!");
}
