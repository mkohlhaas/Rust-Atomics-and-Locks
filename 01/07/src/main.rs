// refcell
// https://mara.nl/atomics/basics.html#refcell

// Atomics is the concurrent version of Cell.
//         Only certain primitive data types depending on platform.
// RwLock  is the concurrent version of RefCell.
// Mutex only allows exclusive borrows. (Simpler than RwLock.)

#![allow(unused_imports)]

use std::cell::{Cell, RefCell};

use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

#[derive(Debug)]
struct X {
  handle: i32,                      // is Send and Sync
  _not_sync: PhantomData<Cell<()>>, // we opt out from Send and Sync with help from PhantomData
}

struct Y {
  p: *mut i32, // raw pointers are not sync or send
}

// Send and Sync are normally auto-implemented.
// If you need to implement them by hand, this is the way to do it:
// `unsafe` indicates that the compiler cannot check the safety (he has to trust us.
unsafe impl Send for Y {}
unsafe impl Sync for Y {}

// Send - T can be sent   to   another thread.
// Sync - T can be shared with another thread, i.e. &T can be sent.

// checking (auto-implemented) traits Send and Sync
fn check_ref_traits<T>(x: &T) -> &T
where
  T: Send,
  // T: Sync,
  // T: Send + Sync,
{
  x
}

// checking (auto-implemented) traits Send and Sync
fn check_traits<T>(x: T) -> T
where
  T: Send,
  // T: Sync,
  // T: Send + Sync,
{
  x
}

fn f(v: &RefCell<Vec<i32>>) {
  v.borrow_mut().push(1); // we can modify the `Vec` directly (as a one-liner; not like Cell)
}

fn main() {
  {
    let v = RefCell::new(vec![1, 2, 3]);
    f(&v);

    println!("{:?}", v);

    assert_eq!(v.into_inner(), vec![1, 2, 3, 1]);
  }

  // https://mara.nl/atomics/basics.html#thread-safety
  {
    let x = X {
      handle: 42,
      _not_sync: Default::default(),
    };

    dbg!(x.handle);

    // Cell is Send but not Sync (and so is X)
    check_ref_traits(&x);
  }

  {
    let y = Y { p: 42 as *mut i32 };

    dbg!(y.p);

    check_ref_traits(&y);
  }

  // {
  //   // ⚠️ If a type is not Send, you can't move it onto another thread.
  //
  //   let a = Rc::new(123);
  //
  //   thread::spawn(|| {
  //     dbg!(&a); // ⚠️ `Rc<i32>` cannot be sent between threads (safely)
  //   });
  // }

  // Lifetime of the MutexGuard
  // https://mara.nl/atomics/basics.html#lifetime-of-mutexguard
  {
    let list: Mutex<Vec<i32>> = Mutex::new(vec![1, 2, 3]);

    // lock and unlock in one statement
    list.lock().unwrap().push(1);
    dbg!(&list);

    // unlock happens immediately
    // no lifetime extension (MutexGuard is unlocked immediately)
    if list.lock().unwrap().pop() == Some(1) {
      println!("I'm alread unlocked");
    }

    // will be unlocked AFTER processing the item
    // aka lifetime extension, extends the lifetime of the MutexGuard
    if let Some(item) = list.lock().unwrap().pop() {
      dbg!(&item);
    } // unlock happens here

    // this is better as unlock happens immediately
    // no lifetime extension
    let item = list.lock().unwrap().pop();
    if let Some(item) = item {
      dbg!(&item);
    }
  }
}
