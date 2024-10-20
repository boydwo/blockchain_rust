use block::Blockchain;

mod block;

fn main() {
    let mut b = Blockchain::new();
    let _ = b.add_block("Send 1 BTC to Ivan, data1".to_string());

    dbg!(&b);
}
