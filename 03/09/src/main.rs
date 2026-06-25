// lazy-init-box
// https://mara.nl/atomics/memory-ordering.html#example-lazy-initialization-with-indirection

use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{Acquire, Release};

#[allow(dead_code)]
struct Data([u8; 100]);

fn generate_data() -> Data {
  Data([42; 100])
}

fn get_data() -> &'static Data {
  static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

  let mut p = PTR.load(Acquire);

  if p.is_null() {
    p = Box::into_raw(Box::new(generate_data()));
    if let Err(ptr) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
      // Safety: p comes from Box::into_raw right above, and wasn't shared with any other thread.
      // NOTE: We are accessing p so we need an Acquire.
      drop(unsafe { Box::from_raw(p) }); // deallocate our data; a different thread alreay initialized
      p = ptr; // ptr points to data from another thread
    }
  }

  // Safety: p is not null and points to a properly initialized value.
  unsafe { &*p }
}

fn main() {
  println!("{:p}", get_data());
  println!("{:p}", get_data()); // same address as before
}
