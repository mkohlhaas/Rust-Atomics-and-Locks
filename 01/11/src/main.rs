// thread-parking
// https://mara.nl/atomics/basics.html#waiting

use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
  let queue = Mutex::new(VecDeque::new());

  thread::scope(|s| {
    // Consuming thread
    let t = s.spawn(|| {
      loop {
        let item = queue.lock().unwrap().pop_front(); // takes from the front
        if let Some(item) = item {
          dbg!(item);
        } else {
          // VecDeque is empty
          thread::park(); // blocks unless or until the current thread's token is made available
        }
      }
    });

    // Producing thread
    for i in 0.. {
      queue.lock().unwrap().push_back(i); // inserts at the end
      if i % 5 == 0 {
        t.thread().unpark(); // atomically makes the handle's token available if it is not already
      }
      thread::sleep(Duration::from_millis(200));
    }
  });
}
