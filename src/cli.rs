// use blockchain_rust::block::Blockchain;
pub type Result<T> = std::result::Result<T, failure::Error>;
use crate::Blockchain;
use clap::arg;
use clap::Command;
use std::fs;
use std::mem;
use std::process::exit;

pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {
            bc: Blockchain::new()?,
        })
    }
    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("Marcos Tulio")
            .about("blockchain in rust: a simple blockchain for learning")
            .subcommand(Command::new("printchain").about("print all the chain blocks"))
            .subcommand(Command::new("clear").about("clear the chain blocks"))
            .subcommand(
                Command::new("addblock")
                    .about("add a block in the blockchain")
                    .arg(arg!(<DATA>"'The blockchain data'")),
            )
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("addblock") {
            if let Some(c) = matches.get_one::<String>("DATA") {
                self.addblock(String::from(c))?;
            } else {
                println!("Not printing testing list...");
                exit(1)
            };
        }

        if let Some(_) = matches.subcommand_matches("printchain") {
            self.cmd_print_chain()?;
        }

        if let Some(_) = matches.subcommand_matches("clear") {
            self.clear_blockchain()?
        }

        Ok(())
    }

    fn cmd_print_chain(&self) -> Result<()> {
        for b in self.bc.db.iter() {
            println!("{:#?}", b);
        }
        Ok(())
    }

    fn addblock(&mut self, data: String) -> Result<()> {
        println!("Adding block with data: {:?}", data);
        self.bc.add_block(data)?;
        Ok(())
    }

    fn clear_blockchain(&mut self) -> Result<()> {
        let db_default = sled::open("/dev/null")?; // Abre um banco de dados vazio como um substituto
        let old_db = mem::replace(&mut self.bc.db, db_default);

        // Liberar o banco de dados original
        drop(old_db);

        // Excluir o arquivo do banco de dados
        let db_path = "data/blocks";
        fs::remove_dir_all(db_path)?;

        println!("Blockchain cleared.");
        std::process::exit(0);
    }
}
