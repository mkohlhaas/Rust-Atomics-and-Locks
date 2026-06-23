// refcell
// https://mara.nl/atomics/basics.html#refcell

#![allow(unused_imports)]

use std::cell::{Cell, RefCell};

use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

#[derive(Debug)]
struct X {
  handle: i32,
  _not_sync: PhantomData<Cell<()>>,
}

struct Y {
  p: *mut i32,
}

unsafe impl Send for Y {}
unsafe impl Sync for Y {}

fn check_traits<T>(x: &T) -> &T
where
  T: Send,
  // T: Sync,
  // T: Send + Sync,
{
  x
}

fn f(v: &RefCell<Vec<i32>>) {
  v.borrow_mut().push(1); // we can modify the `Vec` directly.
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
    check_traits(&x);
  }

  {
    let y = Y { p: 42 as *mut i32 };

    dbg!(y.p);

    check_traits(&y);
  }

  // {
  //   let a = Rc::new(123);
  //
  //   // ⚠️ `Rc<i32>` cannot be sent between threads (safely)
  //   thread::spawn(|| {
  //     // Error!
  //     dbg!(&a);
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
    if list.lock().unwrap().pop() == Some(1) {
      println!("I'm alread unlocked");
    }

    // will be unlocked AFTER processing the item
    if let Some(item) = list.lock().unwrap().pop() {
      dbg!(&item);
    } // unlock happens here

    // this is better as unlock happens immediately
    let item = list.lock().unwrap().pop();
    if let Some(item) = item {
      dbg!(&item);
    }
  }
}
