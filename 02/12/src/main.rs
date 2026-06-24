// id-allocation-without-overflow
// Compare-and-Exchange Operations
// https://mara.nl/atomics/atomics.html#example-handle-overflow

use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering::Relaxed;

const MAX_ID: u16 = 100;

fn allocate_new_id() -> u16 {
  static NEXT_ID: AtomicU16 = AtomicU16::new(0);
  let mut id = NEXT_ID.load(Relaxed);

  println!("ID: {}", id);

  loop {
    assert!(id < MAX_ID, "too many IDs!");
    match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
      Ok(_) => return id,
      Err(v) => id = v,
    }
  }
}

fn main() {
  dbg!(allocate_new_id());
  dbg!(allocate_new_id());
  dbg!(allocate_new_id());

  println!("Trying to overflow the counter… (this might take a minute)");

  // don't do anything on a panic
  std::panic::set_hook(Box::new(|_| {}));

  for _ in 3..=u16::MAX {
    let _id = std::panic::catch_unwind(|| allocate_new_id());
  }

  println!("Done!");
}
