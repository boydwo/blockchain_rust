use blockchain::blockchain::Blockchain;

mod blockchain;

pub type Result<T> = std::result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let mut b = Blockchain::new()?;
    let _ = b.add_block("Send 1 BTC to Ivan, data1".to_string());
    let _ = b.add_block("Send 2 more BTC to Ivan, data2".to_string());
    let _ = b.add_block("Send 3 more BTC to Ivan, data2".to_string());

    for block in b.inter() {
        println!("{:?}", block);
    }

    Ok(())
}
