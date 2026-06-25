// spawn-closure
// https://mara.nl/atomics/basics.html#threads

use std::thread;

fn main() {
  let numbers = vec![1, 2, 3];

  // move is necessary bc spawn has a 'static (thread might live as long as the program)
  thread::spawn(move || {
    for n in &numbers {
      println!("{n}");
    }
  })
  .join()
  .unwrap();
}
