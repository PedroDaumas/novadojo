#[cfg(test)]
mod test;

fn main() {
  println!("result:", evaluate(['azul', 'verde'], ['vermelho', 'verde']));
}

fn sum(a: i32, b:i32) -> i32 {
  return a + b;
}


// comparar as duas arrays e retornar se tiver algo igual.
// verificar o index e ver se estão na mesma posição
// mesma posição ? -> well placed ++
// não está na mesma posição ? -> misplaced ++

fn evaluate(array_1, array_2) {
  well_placed = 0
  misplaced = 0
}
