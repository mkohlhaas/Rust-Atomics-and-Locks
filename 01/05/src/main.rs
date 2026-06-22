// rc

use std::rc::Rc;

fn main() {
  let a = Rc::new([1, 2, 3]);
  let b = a.clone();

  println!("{:p}", a);
  println!("{:p}", b);

  assert_eq!(a.as_ptr(), b.as_ptr()); // same allocation!
}
