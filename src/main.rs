use blockchainlib::*;

fn main() {
    let mut block = Block::new(0, 0, vec![0; 32], 0, "Bloconum Block".to_owned());

    println!("{:?}", &block);

    let hash = block.hash();
    println!("{:?}", &hash);

    block.hash = hash;
    println!("{:?}", &block);
}
