// id-allocation-subtract-before-panic
// Fetch-and-Modify Operations
// https://mara.nl/atomics/atomics.html#example-id-allocation

use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering::Relaxed;

const MAX_ID: u16 = 100;

fn allocate_new_id() -> u16 {
  static NEXT_ID: AtomicU16 = AtomicU16::new(0);
  let id = NEXT_ID.fetch_add(1, Relaxed);
  println!("Current ID: {id}");
  if id >= MAX_ID {
    NEXT_ID.fetch_sub(1, Relaxed);
    panic!("too many IDs!");
  }
  id
}

fn main() {
  dbg!(allocate_new_id());
  dbg!(allocate_new_id());
  dbg!(allocate_new_id());

  println!("Overflowing the counter… (this might take a minute)");

  // don't do anything on a panic
  std::panic::set_hook(Box::new(|_| {}));

  for _ in 3..=u16::MAX {
    let id = std::panic::catch_unwind(|| allocate_new_id()).unwrap_or_default();
    if id != 0 {
      println!("ID: {id}");
    }
  }

  println!("Done!");
}
