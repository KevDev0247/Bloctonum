use blockchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut block = Block::new(0, 0, vec![0; 32], 0, "Bloconum Block".to_owned(), difficulty);

    block.mine();
    println!("Mined bloconum block {:?}", &block);

    let mut previous_hash = block.hash.clone();
    let mut blockchain = Blockchain {
        blocks: vec![block],
    };
    for i in 1..=10 {
        let mut block = Block::new(i, 0, previous_hash, 0, "New Bloconum Block".to_owned(), difficulty);

        block.mine();
        println!("Mined bloconum block {:?}", &block);

        previous_hash = block.hash.clone();

        blockchain.blocks.push(block);
    }
}
