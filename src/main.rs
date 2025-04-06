use std::os::unix::process;
use std::thread::sleep;
use std::time::Duration;
use blockchain::BlockChain;
use crate::block::Block;
use sha256::digest;
mod block;
mod blockchain;
use std::process::exit;
fn main() {
    // let input = String::from("hi there");
    // let val = digest(input);
    // println!("{:?}", val);

  let mut blockchain = BlockChain::new();

  blockchain.genesis();
    // let mut block_amount = 0;
    let mut i = 1;
    while i < 10_000 { 
        let data = format!("block {}", i);
        let prev_block = &blockchain.blocks[i-1];
        let block1 = Block::new(i, data, prev_block);
        blockchain.add_block(block1);
        println!("{:#?}", blockchain);
        sleep(Duration::from_secs(2));
        i += 1;
    }
    
}

// 
// Block { <- last_block
//     index: 11,
//     timestamp: 1743811745,
//     data: "block 11",
//     previous_hash: "00deffff3d1d4583b76a050bfcdde28d4dca8b65bb22b8b3e3996b83ff024204",
//     hash: "007a8b2c178b18c2539b5bff621b2308bf46af73eb57be2d1642c7e4f65716ff",
//     nonce: 311,
// },
// Block { <- block
//     index: 12,
//     timestamp: 1743811747,
//     data: "block 12",
//     previous_hash: "007a8b2c178b18c2539b5bff621b2308bf46af73eb57be2d1642c7e4f65716ff",
//     hash: "0033cca5a13ba45191863a0163587182209e89a9586e04009f659b65066a7b95",
//     nonce: 992,
// },