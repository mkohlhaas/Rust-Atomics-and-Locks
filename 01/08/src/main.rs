// mutex
// https://mara.nl/atomics/basics.html#mutex-and-rwlock

use std::sync::Mutex;
use std::thread;

const MAX_COUNT: i32 = 1000;

fn main() {
  let n = Mutex::new(0);

  thread::scope(|s| {
    for _ in 0..MAX_COUNT {
      s.spawn(|| {
        let mut guard = n.lock().unwrap();
        for _ in 0..MAX_COUNT {
          *guard += 1;
        }
      });
    }
  });

  println!("{:?}", n);

  assert_eq!(n.into_inner().unwrap(), 1_000_000);
}
