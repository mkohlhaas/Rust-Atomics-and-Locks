// condvar
// https://mara.nl/atomics/basics.html#condvar

use std::collections::VecDeque;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
  let queue = Mutex::new(VecDeque::new());
  let not_empty = Condvar::new();

  thread::scope(|s| {
    s.spawn(|| {
      loop {
        let mut q = queue.lock().unwrap();
        let item = loop {
          if let Some(item) = q.pop_front() {
            break item;
          } else {
            // wait takes a mutex and unlocks, waits, relocks
            q = not_empty.wait(q).unwrap(); // blocks the current thread until this condition variable receives a notification
          }
        };
        drop(q);
        dbg!(item);
      }
    });

    for i in 0.. {
      queue.lock().unwrap().push_back(i);
      if i % 5 == 0 {
        not_empty.notify_one(); // wakes up one blocked thread on this condvar
      }
      thread::sleep(Duration::from_millis(200));
    }
  });
}
