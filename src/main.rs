extern crate playground;

trait Fruit {
    fn get_size(&self) -> u64;
}

struct Apple {
  size: u64
}

impl Fruit for Apple {
  fn get_size(&self) -> u64 {
    self.size
  }
}

fn fizz_buzz(num: i32) -> i32 {
    let mut count: i32 = 0;
    for x in 1..num {
        if x % 3 + x % 5 == 0 {
            println!("15の倍数： {}", x);
            count += 1;
            continue;
        }
        if x % 3 == 0 {
            println!("3の倍数： {}", x);
        }
        if x % 5 == 0 {
            println!("5の倍数： {}", x);
        }
    }
    return count;
}

fn main() {
    let count = fizz_buzz(10);
    let count2 = fizz_buzz(10);
    println!("15の倍数の個数： {}", count);
    println!("15の倍数の個数： {}", count2);

    let apple = Apple { size: 10 };
    let size = apple.get_size();
    println!("{}", size);

    playground::tomo::tomo();
}
