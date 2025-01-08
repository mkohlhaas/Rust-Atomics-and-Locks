use std::thread;

fn main() {
  let numbers = vec![1, 2, 3];

  thread::scope(|s| {
    s.spawn(|| {
      println!("length: {}", numbers.len());
    });
    s.spawn(|| {
      for n in &numbers {
        println!("{n}");
      }
    });
  });

  // let mut numbers = vec![1, 2, 3];
  // thread::scope(|s| {
  //   s.spawn(|| {
  //     numbers.push(1);
  //   });
  //   s.spawn(|| {
  //     numbers.push(2); // Error!
  //   });
  // });
}
