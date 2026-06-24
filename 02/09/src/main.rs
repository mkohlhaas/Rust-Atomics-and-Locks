// id-allocation-panic
// Fetch-and-Modify Operations
// https://mara.nl/atomics/atomics.html#example-id-allocation

// u32 would take too long ot overflow (several hours)
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering::Relaxed;

const MAX_ID: u16 = 100;

// ⚠️ This version is problematic.
fn allocate_new_id() -> u16 {
  static NEXT_ID: AtomicU16 = AtomicU16::new(0);
  let id = NEXT_ID.fetch_add(1, Relaxed);
  assert!(id < MAX_ID, "too many IDs!");
  println!("Returning ID: {id}");
  id
}

fn main() {
  dbg!(allocate_new_id()); // This will produce a zero.

  for _ in 1..MAX_ID {
    allocate_new_id(); // 1 through 999.
  }

  println!("Overflowing the counter…");

  // don't do anything on a panic
  std::panic::set_hook(Box::new(|_| {}));

  for _ in MAX_ID..=u16::MAX {
    let _ = std::panic::catch_unwind(|| allocate_new_id());
  }

  println!("…overflowed!");

  dbg!(allocate_new_id()); // ⚠️ this will produce zero again
}
