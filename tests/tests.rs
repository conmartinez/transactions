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

#[test]
fn handle_transations_deposits_withdrawals_and_dispute_one_client() {
    let csv = include_str!("../data/deposit_withdrawal_and_dispute_one_client.csv");
    let expected = "client,available,held,total,locked\n1,4.5,1.5,6.0,false\n";
    let mut client_store = ClientStore::new();
    transactions::handle_transactions_from_reader(csv.as_bytes(), &mut client_store);
    let state = client_store.get_current_state(true).unwrap();
    assert_eq!(state, expected);
}

#[test]
fn handle_transations_deposits_withdrawals_and_dispute_multi_client() {
    let csv = include_str!("../data/deposit_withdrawal_and_dispute_multi_client.csv");
    let expected = "client,available,held,total,locked\n1,4.5,1.5,6.0,false\n2,7.5,2.5,10.0,false\n3,10.5,3.5,14.0,false\n";
    let mut client_store = ClientStore::new();
    transactions::handle_transactions_from_reader(csv.as_bytes(), &mut client_store);
    let state = client_store.get_current_state(true).unwrap();
    assert_eq!(state, expected);
}

#[test]
fn handle_transations_deposits_withdrawals_dispute_and_resolve_one_client() {
    let csv = include_str!("../data/deposit_withdrawal_dispute_and_resolve_one_client.csv");
    let expected = "client,available,held,total,locked\n1,6.0,0.0,6.0,false\n";
    let mut client_store = ClientStore::new();
    transactions::handle_transactions_from_reader(csv.as_bytes(), &mut client_store);
    let state = client_store.get_current_state(true).unwrap();
    assert_eq!(state, expected);
}

#[test]
fn handle_transations_deposits_withdrawals_dispute_and_resolve_multi_client() {
    let csv = include_str!("../data/deposit_withdrawal_dispute_and_resolve_multi_client.csv");
    let expected = "client,available,held,total,locked\n1,6.0,0.0,6.0,false\n2,10.0,0.0,10.0,false\n3,14.0,0.0,14.0,false\n";
    let mut client_store = ClientStore::new();
    transactions::handle_transactions_from_reader(csv.as_bytes(), &mut client_store);
    let state = client_store.get_current_state(true).unwrap();
    assert_eq!(state, expected);
}

#[test]
fn handle_transations_deposits_withdrawals_dispute_and_chargeback_one_client() {
    let csv = include_str!("../data/deposit_withdrawal_dispute_and_chargeback_one_client.csv");
    let expected = "client,available,held,total,locked\n1,1.5,0.0,1.5,true\n";
    let mut client_store = ClientStore::new();
    transactions::handle_transactions_from_reader(csv.as_bytes(), &mut client_store);
    let state = client_store.get_current_state(true).unwrap();
    assert_eq!(state, expected);
}

#[test]
fn handle_transations_deposits_withdrawals_dispute_and_chargeback_multi_client() {
    let csv = include_str!("../data/deposit_withdrawal_dispute_and_chargeback_multi_client.csv");
    let expected = "client,available,held,total,locked\n1,3.0,0.0,3.0,true\n2,7.5,0.0,7.5,true\n3,10.5,0.0,10.5,true\n";
    let mut client_store = ClientStore::new();
    transactions::handle_transactions_from_reader(csv.as_bytes(), &mut client_store);
    let state = client_store.get_current_state(true).unwrap();
    assert_eq!(state, expected);
}