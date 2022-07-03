// mod swap;
// use swap::swap;
mod word_swap;
use word_swap::WordSwap;


fn main() {
    // let swappy = swap("Hello,World.How are you World?","World","me");
    let mut swap = WordSwap::new("Hello,World.How are you World?","World","me");
    let swappy:String = swap.swap();
    println!("{}",swappy);
}
