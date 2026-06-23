// scoped-threads
// https://mara.nl/atomics/basics.html#scoped-threads

use std::thread;

fn main() {
  // Thread Builder
  {
    let builder = thread::Builder::new()
      .name("my_thread".into())
      .stack_size(32 * 1024);

    let handler = builder.spawn(|| 42).unwrap();

    let res = handler.join().unwrap();
    println!("{res}");
  }

  println!();

  // Scoped Threads
  {
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
  }

  // ⚠️ not allowed: readers and writers at the same time
  // {
  //   let mut numbers = vec![1, 2, 3];
  //
  //   thread::scope(|s| {
  //     s.spawn(|| {
  //       numbers.push(1);
  //     });
  //     s.spawn(|| {
  //       numbers.push(2); // Error!
  //     });
  //   });
  // }

  println!();

  {
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    // let x: &mut [i32; 3] = Box::leak(Box::new([1, 2, 3])); // ⚠️

    let j1 = thread::spawn(move || dbg!(x));
    let j2 = thread::spawn(move || dbg!(x));

    j1.join().unwrap();
    j2.join().unwrap();
  }
}
