use std::collections::HashMap;

use crate::transaction::Transaction;
use crate::ClientID;

/// Representation of
pub struct Client {
    pub id: ClientID,
    /// Ammount of currently available funds
    pub available: f64,
    /// Ammount of currently held funds
    pub held: f64,
    /// Client is locked status
    pub locked: bool,
}

impl Client {
    pub fn new(id: ClientID) -> Self {
        Client {
            id,
            available: 0.0,
            held: 0.0,
            locked: false,
        }
    }

    /// Get the total ammount of funds
    ///
    /// This is `available funds` + `held funds`
    fn total(&self) -> f64 {
        self.available + self.held
    }
}

pub struct ClientStore {
    /// Collection of all Clients.
    /// Assumption: All Clients will have a unique ID
    clients: HashMap<ClientID, Client>,
}

impl ClientStore {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    /// Execute the transaction.
    pub fn execute<T>(&mut self, transaction: T)
    where
        T: Transaction,
    {
        match self.clients.get_mut(&transaction.requested_client_id()) {
            Some(client) => {
                transaction.execute(client);
            }
            None => {
                let mut new_client = Client::new(transaction.requested_client_id());
                transaction.execute(&mut new_client);
                self.clients.insert(transaction.requested_client_id(), new_client);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::TransactionError;

    #[test]
    fn new_client() {
        let client = Client::new(157);
        assert_eq!(client.id, 157, "New Client ID is not as expected!");
        assert_eq!(
            client.available, 0.0,
            "New Client available balance is not as expected!"
        );
        assert_eq!(
            client.locked, false,
            "New Client is locked! Should be unlocked"
        );
    }

    #[test]
    fn client_total_greater_available_than_held() {
        let mut client = Client::new(157);
        client.available = 54.7345;
        client.held = 3.5678;
        assert_eq!(client.total(), 54.7345 + 3.5678)
    }

    #[test]
    fn client_total_greater_held_than_available() {
        let mut client = Client::new(157);
        client.available = 3.5678;
        client.held = 54.7345;
        assert_eq!(client.total(), 54.7345 + 3.5678)
    }

    #[test]
    fn new_client_store() {
        let client_store = ClientStore::new();
        assert!(client_store.clients.is_empty())
    }

    #[test]
    fn client_store_plus_1_transaction() {
        struct TestTransaction {}
        impl Transaction for TestTransaction {
            fn execute(&self, client: &mut Client) -> Result<(), TransactionError> {
                // Add one to client 
                client.available += 1.0;
                client.held += 1.0;
                Ok(())
            }

            fn requested_client_id(&self) -> ClientID {
                1
            }

            fn transaction_id(&self) -> crate::TransactionID {
                1
            }
        }
        let mut client_store = ClientStore::new();
        client_store.execute(TestTransaction {});
        assert_eq!(client_store.clients.get(&1).unwrap().available, 1.0);
        assert_eq!(client_store.clients.get(&1).unwrap().held, 1.0);
        assert_eq!(client_store.clients.get(&1).unwrap().locked, false);
    }

    #[test]
    fn client_store_null_transaction() {
        struct TestTransaction {}
        impl Transaction for TestTransaction {
            fn execute(&self, client: &mut Client) -> Result<(), TransactionError> {
                Ok(())
            }

            fn requested_client_id(&self) -> ClientID {
                1
            }

            fn transaction_id(&self) -> crate::TransactionID {
                1
            }
        }
        let mut client_store = ClientStore::new();
        client_store.execute(TestTransaction {});
        assert_eq!(client_store.clients.get(&1).unwrap().available, 0.0);
        assert_eq!(client_store.clients.get(&1).unwrap().held, 0.0);
        assert_eq!(client_store.clients.get(&1).unwrap().locked, false);
    }

    #[test]
    fn client_store_add_available_transaction_multiple() {
        struct TestTransaction {
            id: ClientID,
        }
        impl Transaction for TestTransaction {
            fn execute(&self, client: &mut Client) -> Result<(), TransactionError> {
                // Add 4.5689 to avaialble
                client.available += 4.5689;
                Ok(())
            }

            fn requested_client_id(&self) -> ClientID {
                self.id
            }

            fn transaction_id(&self) -> crate::TransactionID {
                1
            }
        }
        let mut client_store = ClientStore::new();
        client_store.execute(TestTransaction { id: 1 });
        client_store.execute(TestTransaction { id: 1 });
        client_store.execute(TestTransaction { id: 1 });
        client_store.execute(TestTransaction { id: 1 });
        assert_eq!(client_store.clients.get(&1).unwrap().available, 4.5689 + 4.5689 + 4.5689 + 4.5689);
        assert_eq!(client_store.clients.get(&1).unwrap().held, 0.0);
        assert_eq!(client_store.clients.get(&1).unwrap().locked, false);
    }

    #[test]
    fn client_store_add_available_transaction_multiple_with_different_clients() {
        struct TestTransaction {
            id: ClientID,
        }
        impl Transaction for TestTransaction {
            fn execute(&self, client: &mut Client) -> Result<(), TransactionError> {
                // Add 4.5689 to avaialble
                client.available += 4.5689;
                Ok(())
            }

            fn requested_client_id(&self) -> ClientID {
                self.id
            }

            fn transaction_id(&self) -> crate::TransactionID {
                1
            }
        }
        let mut client_store = ClientStore::new();
        client_store.execute(TestTransaction { id: 1 });
        client_store.execute(TestTransaction { id: 2 });
        client_store.execute(TestTransaction { id: 1 });
        client_store.execute(TestTransaction { id: 2 });
        client_store.execute(TestTransaction { id: 2 });
        client_store.execute(TestTransaction { id: 1 });
        client_store.execute(TestTransaction { id: 1 });
        client_store.execute(TestTransaction { id: 2 });
        assert_eq!(client_store.clients.get(&1).unwrap().available, 4.5689 + 4.5689 + 4.5689 + 4.5689);
        assert_eq!(client_store.clients.get(&1).unwrap().held, 0.0);
        assert_eq!(client_store.clients.get(&1).unwrap().locked, false);
        assert_eq!(client_store.clients.get(&1).unwrap().available, 4.5689 + 4.5689 + 4.5689 + 4.5689);
        assert_eq!(client_store.clients.get(&1).unwrap().held, 0.0);
        assert_eq!(client_store.clients.get(&1).unwrap().locked, false);
    }
}
