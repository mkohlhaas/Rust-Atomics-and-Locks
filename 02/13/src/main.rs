// lazy-one-time-init
// Compare-and-Exchange Operations
// https://mara.nl/atomics/atomics.html#example-racy-init

use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;

use rand::RngExt;

fn generate_random_key() -> u64 {
  let mut rng = rand::rng();
  rng.random_range(1..100)
}

fn get_key() -> u64 {
  static KEY: AtomicU64 = AtomicU64::new(0);
  let key = KEY.load(Relaxed);
  if key == 0 {
    let new_key = generate_random_key();
    match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
      Ok(_) => new_key,
      Err(key) => key, // key has allready been set by another thread (not possible in our scenario;
                       // we have only one thread)
    }
  } else {
    key
  }
}

fn main() {
  for _ in 1..50 {
    println!("Key: {}", get_key());
  }
}
