#[cfg(test)]
mod test;

fn main() {
  let array1 = vec!["azul", "verde"];
  let array2 = vec!["vermelho", "verde"];

  for (pos,elem) in array1.iter().enumerate() {
    for (pos2,elem2) in array2.iter().enumerate() {
      if elem == elem2 {

      }
      // println!("{}", pos);
      // println!("{}", elem);
    }
  }
  // evaluate(test, test2)
}

fn sum(a: i32, b:i32) -> i32 {
  return a + b;
}

// fn evaluate(array1: vec!, array2: vec!) {
//   let well_placed = 0;
//   let miss_placed = 0;

  
//   for (pos,elem) in array1.iter().enumerate() {
//       println!("{}", pos);
//       println!("{}", elem);
//     // for (pos2,elem2) in array2.iter().enumerate() {
//     //   println!("{}", pos);
//     //   println!("{}", elem);
//     // }
//   }
// }
