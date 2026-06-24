// stop-flag
// Atomic Load and Store Operations
// https://mara.nl/atomics/atomics.html#example-stop-flag

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn some_work() {
  print!(".");
  thread::sleep(Duration::from_millis(200));
}

fn main() {
  static STOP: AtomicBool = AtomicBool::new(false);

  // Spawn a thread to do the work.
  let background_thread = thread::spawn(|| {
    while !STOP.load(Relaxed) {
      some_work();
    }
    println!("Shutting down...")
  });

  // Use the main thread to listen for user input.
  for line in std::io::stdin().lines() {
    match line.unwrap().as_str() {
      "help" => println!("Commands: help, stop"),
      "stop" => break,
      cmd => println!("Unknown command: {cmd:?}. Ask for help!"),
    }
  }

  // Inform the background thread it needs to stop.
  STOP.store(true, Relaxed);

  // Wait until the background thread finishes.
  background_thread.join().unwrap();

  println!("Done!")
}
