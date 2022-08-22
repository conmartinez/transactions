use std::collections::HashMap;

use csv::Writer;
use itertools::Itertools as _;
use serde::{ser::SerializeStruct as _, Serialize, Serializer};

use crate::error::TransactionError;
use crate::transaction::Transaction;
use crate::{Amount, ClientID, TransactionID};

#[derive(Debug, PartialEq)]
pub struct History {
    pub amount: Amount,
    pub dispute: bool,
}

impl History {
    pub fn new(amount: Amount) -> Self {
        Self {
            amount,
            dispute: false,
        }
    }
}

/// Representation of a client's account
pub struct Client {
    pub id: ClientID,
    /// Ammount of currently available funds
    pub available: Amount,
    /// Ammount of currently held funds
    pub held: Amount,
    /// Client is locked status
    pub locked: bool,
    /// Collection of all transactions
    pub client_history: HashMap<TransactionID, History>,
}

impl Client {
    pub fn new(id: ClientID) -> Self {
        Client {
            id,
            available: 0.0,
            held: 0.0,
            locked: false,
            client_history: HashMap::new(),
        }
    }

    /// Get the total ammount of funds
    ///
    /// This is `available funds` + `held funds`
    pub fn total(&self) -> Amount {
        self.available + self.held
    }
}

impl Serialize for Client {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Client", 5)?;
        state.serialize_field("client", &self.id)?;
        state.serialize_field("available", &self.available)?;
        state.serialize_field("held", &self.held)?;
        state.serialize_field("total", &self.total())?;
        state.serialize_field("locked", &self.locked)?;
        state.end()
    }
}

pub struct ClientStore {
    /// Collection of all Clients.
    /// Assumption: All Clients will have a unique ID
    pub clients: HashMap<ClientID, Client>,
}

impl ClientStore {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    /// Execute the transaction.
    pub fn execute<T>(&mut self, transaction: &T) -> Result<(), TransactionError>
    where
        T: Transaction + ?Sized,
    {
        match self.clients.get_mut(&transaction.requested_client_id()) {
            Some(client) => transaction.execute(client),
            None => {
                let mut new_client = Client::new(transaction.requested_client_id());
                transaction.execute(&mut new_client)?;
                let _ = self
                    .clients
                    .insert(transaction.requested_client_id(), new_client);
                Ok(())
            }
        }
    }

    pub fn get_current_state(&self, sort: bool) -> Result<String, TransactionError> {
        let mut state = Vec::new();
        {
            let mut writer = Writer::from_writer(&mut state);
            if sort {
                for (_id, client) in self.clients.iter().sorted_by_key(|kv| kv.0) {
                    writer.serialize(client)?;
                }
            } else {
                for client in self.clients.values() {
                    writer.serialize(client)?;
                }
            };

            writer.flush()?;
        }
        Ok(String::from_utf8(state)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    struct TestTransaction {
        id: ClientID,
        amount: Amount,
    }

    impl Transaction for TestTransaction {
        fn execute(&self, client: &mut Client) -> Result<(), TransactionError> {
            client.available += self.amount;
            Ok(())
        }

        fn requested_client_id(&self) -> ClientID {
            self.id
        }

        fn transaction_id(&self) -> crate::TransactionID {
            1
        }

        fn amount(&self) -> Option<Amount> {
            Some(self.amount)
        }
    }

    #[test]
    fn client_store_plus_1_transaction() {
        let mut client_store = ClientStore::new();
        client_store
            .execute(&TestTransaction { id: 1, amount: 1.0 })
            .unwrap();
        assert_eq!(client_store.clients.get(&1).unwrap().available, 1.0);
        assert_eq!(client_store.clients.get(&1).unwrap().held, 0.0);
        assert_eq!(client_store.clients.get(&1).unwrap().locked, false);
    }

    #[test]
    fn client_store_add_available_transaction_multiple() {
        let mut client_store = ClientStore::new();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        assert_eq!(
            client_store.clients.get(&1).unwrap().available,
            4.5689 + 4.5689 + 4.5689 + 4.5689
        );
        assert_eq!(client_store.clients.get(&1).unwrap().held, 0.0);
        assert_eq!(client_store.clients.get(&1).unwrap().locked, false);
    }

    #[test]
    fn client_store_add_available_transaction_multiple_with_different_clients() {
        let mut client_store = ClientStore::new();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 2,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 2,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 2,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 2,
                amount: 4.5689,
            })
            .unwrap();
        assert_eq!(
            client_store.clients.get(&1).unwrap().available,
            4.5689 + 4.5689 + 4.5689 + 4.5689
        );
        assert_eq!(client_store.clients.get(&1).unwrap().held, 0.0);
        assert_eq!(client_store.clients.get(&1).unwrap().locked, false);
        assert_eq!(
            client_store.clients.get(&1).unwrap().available,
            4.5689 + 4.5689 + 4.5689 + 4.5689
        );
        assert_eq!(client_store.clients.get(&1).unwrap().held, 0.0);
        assert_eq!(client_store.clients.get(&1).unwrap().locked, false);
    }

    #[test]
    fn final_state_1_plus_1_transaction() {
        // Use different transaction for testing
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

            fn amount(&self) -> Option<Amount> {
                Some(1.0)
            }
        }
        let mut client_store = ClientStore::new();
        client_store.execute(&TestTransaction {}).unwrap();
        assert_eq!(
            &client_store.get_current_state(true).unwrap(),
            "client,available,held,total,locked\n1,1.0,1.0,2.0,false\n"
        );
    }

    #[test]
    fn final_state_multiple_transactions() {
        let mut client_store = ClientStore::new();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        assert_eq!(
            client_store.clients.get(&1).unwrap().available,
            4.5689 + 4.5689 + 4.5689 + 4.5689
        );
        assert_eq!(
            &client_store.get_current_state(true).unwrap(),
            "client,available,held,total,locked\n1,18.2756,0.0,18.2756,false\n"
        );
    }

    #[test]
    fn final_state_multiple_transactions_multiple_clients() {
        let mut client_store = ClientStore::new();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 2,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 2,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 2,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 1,
                amount: 4.5689,
            })
            .unwrap();
        client_store
            .execute(&TestTransaction {
                id: 2,
                amount: 4.5689,
            })
            .unwrap();
        assert_eq!(&client_store.get_current_state(true).unwrap(), "client,available,held,total,locked\n1,18.2756,0.0,18.2756,false\n2,18.2756,0.0,18.2756,false\n");
    }
}
