use super::*;

#[test]
// fn it_works() {
//     let result = sum(2, 2);
//     assert_eq!(result, 4);
// }

fn it_works() {
    let result = evaluate(['azul', 'verde'], ['vermelho', 'verde']);
    assert_eq!(result, [1, 0]);
}
