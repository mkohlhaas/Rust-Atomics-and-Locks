// mutex
// https://mara.nl/atomics/basics.html#mutex-and-rwlock

use std::sync::{Mutex, MutexGuard};
use std::thread;

const MAX_COUNT: i32 = 1000;

fn main() {
  let n = Mutex::new(0);

  thread::scope(|s| {
    for _ in 0..MAX_COUNT {
      s.spawn(|| {
        let mut guard: MutexGuard<'_, i32> = n.lock().unwrap();
        for _ in 0..MAX_COUNT {
          *guard += 1; // MutexGuard implements DerefMut
        }
      }); // guard dropped, mutex unlocked
    }
  });

  println!("{:?}", n);

  // into_inner() consumes the mutex, returning the underlying data.
  // unwrap(…) in case there was a panic.
  assert_eq!(n.into_inner().unwrap(), 1_000_000);
}
