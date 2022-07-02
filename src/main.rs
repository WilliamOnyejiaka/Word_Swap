mod swap;
use swap::swap;


fn main() {
    let swappy = swap("Hello,World.How are you World?","World","me");
    println!("{}",swappy);
}
