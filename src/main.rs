// okay rust is beyonhd stupid when it comes to package management I have accepted it and will not question it again
use types::domino::Domino;
mod types;

fn main() {
    let d = Domino { left: 1, right: 2 };
    let d2 = Domino::new(2, 4);
    println!("left: {}, right: {}", d2.left, d2.right);
}
