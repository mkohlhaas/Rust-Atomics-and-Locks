// sleep-before-unlock
// https://mara.nl/atomics/basics.html#rusts-mutex

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

const MAX_COUNT: i32 = 10;

fn main() {
  let n = Mutex::new(0);

  thread::scope(|s| {
    for _ in 0..MAX_COUNT {
      s.spawn(|| {
        let mut guard = n.lock().unwrap();
        for _ in 0..MAX_COUNT {
          *guard += 1;
        }
        thread::sleep(Duration::from_millis(200)); // new!
      });
    }
  });

  println!("{:?}", n);

  assert_eq!(n.into_inner().unwrap(), 100);
}
