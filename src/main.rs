#[cfg(test)]
mod test;

fn main() {
  let s: Vec<String> = vec!["blue".to_string()];
  let g: Vec<String> = vec!["blue".to_string()];
  evaluate(s, g);
}

fn evaluate(secret: Vec<String>, guess: Vec<String>) -> Vec<u8> {
  //iterar
  let mut count_match: u8;
  for x in secret {
    if (secret.contains(&x) ) {
      count_match += 1;
    }
  }
  // let mut response: bool = secret.contains(&guess);
  println!("Secret: {:?}", secret);
  println!("Guess: {:?}", guess);
  println!("Response: {:?}", response);

  return vec![0, 1];
}