mod hello;
mod hoge;
mod tako;

use hello::*;
use hoge::*;
use tako::*;

fn main() {
  hello();

  let mut tako = Tako { legs: 8 };
  println!("{}", tako.legs);
  tako.twice_legs();
  println!("{}", tako.legs);

  hoge();
}
