// increment-with-compare-exchange
// Compare-and-Exchange Operations
// https://mara.nl/atomics/atomics.html#cas

use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

// our version of fetch_add(…)
fn increment(a: &AtomicU32) {
  let mut current_val = a.load(Relaxed);

  loop {
    let new_val = current_val + 1;
    match a.compare_exchange(current_val, new_val, Relaxed, Relaxed) {
      Ok(_) => {
        println!("OK");
        return;
      }
      // another thread has changed to Atomic (not in our case as we don't have several threads
      // running)
      Err(v) => {
        println!("Error");
        current_val = v;
      }
    }
  }
}

// NOTE: fetch_update(…), now called try_update(…) to prevent overflow
// fn allocate_new_id() -> u16 {
//   static NEXT_ID: AtomicU16 = AtomicU16::new(0);
//   NEXT_ID
//     .try_update(Relaxed, Relaxed, |n| n.checked_add(1))
//     .expect("too many IDs!")
// }

fn main() {
  let a = AtomicU32::new(0);

  increment(&a);
  increment(&a);

  assert_eq!(a.into_inner(), 2);

  println!("Done!")
}
