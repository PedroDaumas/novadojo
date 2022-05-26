#[cfg(test)]
mod test;

fn main() {
  println!("The sum of 2 + 2 is {}", sum(2,2));
}

fn sum(a: i32, b:i32) -> i32 {
  return a + b;
}