#[cfg(test)]
mod test;

fn main() {
  let s = Vec::new();
  let g = Vec::new();
  evaluate(s, g);
}

fn evaluate(secret: Vec<String>, guess: Vec<String>) {
  println!("Secret: {}", secret);
  println!("Guess: {}", guess);
}


fn sum(a: i32, b:i32) -> i32 {
  return a + b;
}