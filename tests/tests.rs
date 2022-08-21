use transactions::{self, client::ClientStore};

#[test]
fn handle_transations_deposits_one_client() {
    let csv = include_str!("../data/deposit_one_client.csv");
    let expected = "client,available,held,total,locked\n1,6.0,0.0,6.0,false\n";
    let mut client_store = ClientStore::new();
    transactions::handle_transactions_from_reader(csv.as_bytes(), &mut client_store);
    let state = client_store.get_current_state(true).unwrap();
    assert_eq!(state, expected);
}

#[test]
fn handle_transations_deposits_multi_client() {
    let csv = include_str!("../data/deposit_multi_client.csv");
    let expected = "client,available,held,total,locked\n1,6.0,0.0,6.0,false\n2,10.0,0.0,10.0,false\n3,14.0,0.0,14.0,false\n";
    let mut client_store = ClientStore::new();
    transactions::handle_transactions_from_reader(csv.as_bytes(), &mut client_store);
    let state = client_store.get_current_state(true).unwrap();
    assert_eq!(state, expected);
}

#[test]
fn handle_transations_deposits_and_withdrawals_one_client() {
    let csv = include_str!("../data/deposit_and_withdrawal_one_client.csv");
    let expected = "client,available,held,total,locked\n1,6.0,0.0,6.0,false\n";
    let mut client_store = ClientStore::new();
    transactions::handle_transactions_from_reader(csv.as_bytes(), &mut client_store);
    let state = client_store.get_current_state(true).unwrap();
    assert_eq!(state, expected);
}

#[test]
fn handle_transations_deposits_and_withdrawals_multi_client() {
    let csv = include_str!("../data/deposit_and_withdrawal_multi_client.csv");
    let expected = "client,available,held,total,locked\n1,6.0,0.0,6.0,false\n2,10.0,0.0,10.0,false\n3,14.0,0.0,14.0,false\n";
    let mut client_store = ClientStore::new();
    transactions::handle_transactions_from_reader(csv.as_bytes(), &mut client_store);
    let state = client_store.get_current_state(true).unwrap();
    assert_eq!(state, expected);
}

#[test]
fn handle_transations_deposits_and_withdrawals_one_client_insufficent_funds() {
    let csv = include_str!("../data/deposit_and_withdrawal_one_client_insufficent_funds.csv");
    let expected = "client,available,held,total,locked\n1,6.0,0.0,6.0,false\n";
    let mut client_store = ClientStore::new();
    transactions::handle_transactions_from_reader(csv.as_bytes(), &mut client_store);
    let state = client_store.get_current_state(true).unwrap();
    assert_eq!(state, expected);
}

#[test]
fn handle_transations_deposits_and_withdrawals_multi_client_insufficent_funds() {
    let csv = include_str!("../data/deposit_and_withdrawal_multi_client_insufficent_funds.csv");
    let expected = "client,available,held,total,locked\n1,6.0,0.0,6.0,false\n2,10.0,0.0,10.0,false\n3,14.0,0.0,14.0,false\n";
    let mut client_store = ClientStore::new();
    transactions::handle_transactions_from_reader(csv.as_bytes(), &mut client_store);
    let state = client_store.get_current_state(true).unwrap();
    assert_eq!(state, expected);
}