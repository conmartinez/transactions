use std::{env, fs::File, io::BufReader};

use transactions::client::ClientStore;
use transactions::handle_transactions_from_reader;

/// Execute transactions and output the final state of all clients. 
/// 
/// expects a single command line arguement be a path to a csv file which contains
/// the transactions to execute. 
/// 
/// When all transactions are complete, the final state is printed to stdout.
/// If errors occur while handling transactions, theses errors are printed to 
/// stderr.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: transations <FILE PATH>");
        return;
    }

    let file_path = &args[1];
    let f = File::open(file_path).unwrap();
    let reader = BufReader::new(f);
    let mut client_store = ClientStore::new();

    handle_transactions_from_reader(reader, &mut client_store);

    let final_state = client_store.get_current_state(false).unwrap();
    println!("{}", final_state);
}
