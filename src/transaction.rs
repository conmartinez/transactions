use crate::{client::Client, ClientID, TransactionID};

pub type TransactionError = String;

pub trait Transaction {
    /// Execute the transaction on the ClientStore.
    ///
    /// Generic execute call for all transactions.
    fn execute(&self, client: &mut Client) -> Result<(), TransactionError> {
        unimplemented!()
    }

    /// Get the Client ID this transaction is meant to run against
    ///
    /// Generic execute call for all transactions.
    fn requested_client_id(&self) -> ClientID;

    /// Get the ID of this transaction
    ///
    /// Generic execute call for all transactions.
    fn transaction_id(&self) -> TransactionID;
}

struct Deposit {
    transaction_id: TransactionID,
    client_id: ClientID,
    ammount: f64,
}

impl Deposit {
    pub fn new(transaction_id: TransactionID, client_id: ClientID, ammount: f64) -> Self {
        Self { transaction_id, client_id, ammount }
    }
}

impl Transaction for Deposit {
    /// Execute the transaction on the ClientStore.
    ///
    /// Add money to available balance of the acount
    fn execute(&self, client: &mut Client) -> Result<(), TransactionError> {
        client.available += self.ammount;
        Ok(())
    }

    fn requested_client_id(&self) -> ClientID {
        self.client_id
    }

    fn transaction_id(&self) -> TransactionID {
        self.transaction_id
    }
}

struct Withdrawal {
    transaction_id: TransactionID,
    client_id: ClientID,
    ammount: f64,
}

impl Withdrawal {
    pub fn new(transaction_id: TransactionID, client_id: ClientID, ammount: f64) -> Self {
        Self { transaction_id, client_id, ammount }
    }
}

impl Transaction for Withdrawal {
    /// Execute the transaction on the ClientStore.
    ///
    /// Remove money to available balance of the acount
    fn execute(&self, client: &mut Client) -> Result<(), TransactionError> {
        if client.available < self.ammount {
            Err("Insufficent funds!".to_string())
        } else {
            client.available -= self.ammount;
            Ok(())
        }
    }

    fn requested_client_id(&self) -> ClientID {
        self.client_id
    }

    fn transaction_id(&self) -> TransactionID {
        self.transaction_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_345_4823_to_empty_client() {
        let ammount = 345.4823;
        let mut client = Client::new(157);
        let transaction = Deposit::new(1, 157, ammount);

        transaction.execute(&mut client).unwrap();
        // verify available is expected
        assert_eq!(client.available, ammount);
        // verify other values are not touched
        assert_eq!(client.held, 0.0);
        assert_eq!(client.locked, false);
    }

    #[test]
    fn withdrawal_45_7611_from_a_client_with_sufficent_funds() {
        let ammount = 35.7611;
        let mut client = Client::new(157);
        client.available = 300.00;
        let transaction = Withdrawal::new(1, 157, ammount);

        transaction.execute(&mut client).unwrap();

        assert_eq!(client.available, 300.00 - ammount);
        assert_eq!(client.held, 0.0);
        assert_eq!(client.locked, false);
    }

    #[test]
    fn withdrawal_45_7611_from_a_client_with_insufficent_funds() {
        let ammount = 35.7611;
        let mut client = Client::new(157);
        client.available = 30.0000;
        let transaction = Withdrawal::new(1, 157, ammount);

        // verify it errors. Don't care what the error is now becuase of simple error handling in place.
        assert!(transaction.execute(&mut client).is_err());
        // verify available is still the same
        assert_eq!(client.available, 30.0000)
    }
}