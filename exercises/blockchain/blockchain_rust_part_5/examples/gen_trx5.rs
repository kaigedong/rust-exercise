use std::{env::current_dir, sync::Arc};

use blockchain_rust_part_5::{Blockchain, SledDb, Transaction, UTXOSet, Wallets};

fn main() {
    tracing_subscriber::fmt().init();

    let justin_addr = "1M684nX5dTNQYi2ELSCazjyz5dgegJ3mVD";

    let mut wallets = Wallets::new().unwrap();
    let bob_addr = wallets.create_wallet();
    let bruce_addr = wallets.create_wallet();

    let path = current_dir().unwrap().join("data");
    let storage = Arc::new(SledDb::new(path));

    let mut bc = Blockchain::new(storage.clone(), justin_addr);
    let utxos = UTXOSet::new(storage);

    let tx_1 = Transaction::new_utxo(justin_addr, &bob_addr, 4, &utxos, &bc);
    let tx_2 = Transaction::new_utxo(justin_addr, &bruce_addr, 2, &utxos, &bc);

    let txs = vec![tx_1, tx_2];

    bc.mine_block(&txs);
    utxos.reindex(&bc).unwrap();

    bc.blocks_info();
}
