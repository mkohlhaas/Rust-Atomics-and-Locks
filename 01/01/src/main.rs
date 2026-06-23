// hello
// https://mara.nl/atomics/basics.html#threads

use std::thread;

fn f() {
  println!("Hello from another thread!");

  let id = thread::current().id();
  println!("This is my thread id: {id:?}");
}

fn main() {
  thread::spawn(f);
  thread::spawn(f);
  let id = thread::current().id();
  println!("Hello from the main thread: {id:?}");
}
