// hello join
// https://mara.nl/atomics/basics.html#threads

use std::thread;

fn f() {
  println!("Hello from another thread!");

  let id = thread::current().id();
  println!("This is my thread id: {id:?}");
}

fn main() {
  {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    let id = thread::current().id();
    println!("Hello from the main thread: {id:?}");

    t1.join().unwrap();
    t2.join().unwrap();
  }

  {
    // returns Err on panics
    let t = thread::spawn(|| panic!("Oops!!!"));
    t.join().expect("I wanted to join!");
  }
}
