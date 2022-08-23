use crate::{
    client::{Client, History},
    error::TransactionError,
    Amount, ClientID, CsvLine, CsvLineType, TransactionID,
};

pub trait Transaction {
    /// Execute the transaction on the ClientStore.
    ///
    /// Generic execute call for all transactions.
    fn execute(&self, client: &mut Client) -> Result<(), TransactionError>;

    /// Get the Client ID this transaction is meant to run against
    ///
    /// Generic method for getting the transaction's client id.
    fn requested_client_id(&self) -> ClientID;

    /// Get the ID of this transaction
    ///
    /// Generic method for getting the transaction's id.
    fn transaction_id(&self) -> TransactionID;

    /// Get the Amount of this transaction
    ///
    /// Generic method for getting the transaction's amount.
    /// Not all transations have an amount so an option is returned.
    fn amount(&self) -> Option<Amount>;
}

impl From<CsvLine> for Box<dyn Transaction> {
    fn from(csv_line: CsvLine) -> Self {
        match csv_line.t_type {
            CsvLineType::Chargeback => {
                Box::new(Chargeback::new(csv_line.tx, csv_line.client)) as Box<dyn Transaction>
            },
            CsvLineType::Deposit => {
                Box::new(Deposit::new(csv_line.tx, csv_line.client, csv_line.amount))
                    as Box<dyn Transaction>
            },
            CsvLineType::Withdrawal => {
                Box::new(Withdrawal::new(csv_line.tx, csv_line.client, csv_line.amount)) 
                    as Box<dyn Transaction>
            },
            CsvLineType::Dispute => {
                Box::new(Dispute::new(csv_line.tx, csv_line.client)) as Box<dyn Transaction>
            },
            CsvLineType::Resolve => {
                Box::new(Resolve::new(csv_line.tx, csv_line.client)) as Box<dyn Transaction>
            },
        }
    }
}

struct Deposit {
    transaction_id: TransactionID,
    client_id: ClientID,
    ammount: f64,
}

impl Deposit {
    pub fn new(transaction_id: TransactionID, client_id: ClientID, ammount: f64) -> Self {
        Self {
            transaction_id,
            client_id,
            ammount,
        }
    }
}

impl Transaction for Deposit {
    /// Execute the transaction on the ClientStore.
    ///
    /// Add money to available balance of the acount
    fn execute(&self, client: &mut Client) -> Result<(), TransactionError> {
        if client.locked {
            return Err("Could not deposit funds. Account is locked.".into())
        }
        client.available += self.ammount;
        client
            .client_history
            .insert(self.transaction_id, History::new(self.ammount));
        Ok(())
    }

    fn requested_client_id(&self) -> ClientID {
        self.client_id
    }

    fn transaction_id(&self) -> TransactionID {
        self.transaction_id
    }

    fn amount(&self) -> Option<Amount> {
        Some(self.ammount)
    }
}

struct Withdrawal {
    transaction_id: TransactionID,
    client_id: ClientID,
    ammount: f64,
}

impl Withdrawal {
    pub fn new(transaction_id: TransactionID, client_id: ClientID, ammount: f64) -> Self {
        Self {
            transaction_id,
            client_id,
            ammount,
        }
    }
}

impl Transaction for Withdrawal {
    /// Execute the transaction on the ClientStore.
    ///
    /// Remove money to available balance of the acount
    fn execute(&self, client: &mut Client) -> Result<(), TransactionError> {
        if client.locked {
            return Err("Could not withdrawal funds. Account is locked.".into())
        }
        if client.available < self.ammount {
            Err("Insufficent funds!".into())
        } else {
            client.available -= self.ammount;
            client
                .client_history
                .insert(self.transaction_id, History::new(self.ammount));
            Ok(())
        }
    }

    fn requested_client_id(&self) -> ClientID {
        self.client_id
    }

    fn transaction_id(&self) -> TransactionID {
        self.transaction_id
    }

    fn amount(&self) -> Option<Amount> {
        Some(self.ammount)
    }
}
struct Dispute {
    transaction_id: TransactionID,
    client_id: ClientID,
}

impl Dispute {
    pub fn new(transaction_id: TransactionID, client_id: ClientID) -> Self {
        Self {
            transaction_id,
            client_id,
        }
    }
}

impl Transaction for Dispute {
    /// Execute the transaction on the ClientStore.
    ///
    /// Remove money to available balance of the acount
    fn execute(&self, client: &mut Client) -> Result<(), TransactionError> {
        if client.locked {
            return Err("Could not dispute funds. Account is locked.".into())
        }
        match client.client_history.get_mut(&self.transaction_id) {
            Some(history) => {
                if !history.dispute {
                    history.dispute = true;
                    client.available -= history.amount;
                    client.held += history.amount;
                    Ok(())
                } else {
                    Err(format!("Specified transaction {} for client {} is not already disputed.", self.transaction_id, self.client_id).into()) 
                }
            }
            None => {
                Err(format!("No transaction {} found for client {}", self.transaction_id, self.client_id).into())
            }
        }
    }

    fn requested_client_id(&self) -> ClientID {
        self.client_id
    }

    fn transaction_id(&self) -> TransactionID {
        self.transaction_id
    }

    fn amount(&self) -> Option<Amount> {
        None
    }
}

struct Resolve {
    transaction_id: TransactionID,
    client_id: ClientID,
}

impl Resolve {
    pub fn new(transaction_id: TransactionID, client_id: ClientID) -> Self {
        Self {
            transaction_id,
            client_id,
        }
    }
}

impl Transaction for Resolve {
    /// Execute the transaction on the ClientStore.
    ///
    /// Resolve the disputed transaction.
    /// Move amount in question from held to total
    fn execute(&self, client: &mut Client) -> Result<(), TransactionError> {
        if client.locked {
            return Err("Could not resolve funds. Account is locked.".into())
        }
        match client.client_history.get_mut(&self.transaction_id) {
            Some(history) => {
                if history.dispute {
                    history.dispute = false;
                    client.available += history.amount;
                    client.held -= history.amount;
                    Ok(())
                } else {
                    Err(format!("Specified transaction {} for client {} is not being disputed.", self.transaction_id, self.client_id).into()) 
                }
            }
            None => {
                Err(format!("No transaction {} found for client {}", self.transaction_id, self.client_id).into())
            }
        }
    }

    fn requested_client_id(&self) -> ClientID {
        self.client_id
    }

    fn transaction_id(&self) -> TransactionID {
        self.transaction_id
    }

    fn amount(&self) -> Option<Amount> {
        None
    }
}

struct Chargeback {
    transaction_id: TransactionID,
    client_id: ClientID,
}

impl Chargeback {
    pub fn new(transaction_id: TransactionID, client_id: ClientID) -> Self {
        Self {
            transaction_id,
            client_id,
        }
    }
}

impl Transaction for Chargeback {
    /// Execute the transaction on the ClientStore.
    ///
    /// Resolve the disputed transaction.
    /// Remove the amount in question from held and lock account
    fn execute(&self, client: &mut Client) -> Result<(), TransactionError> {
        if client.locked {
            return Err("Could not chargeback funds. Account is locked.".into())
        }
        match client.client_history.get_mut(&self.transaction_id) {
            Some(history) => {
                if history.dispute {
                    history.dispute = false;
                    client.held -= history.amount;
                    client.locked = true;
                    Ok(())
                } else {
                    Err(format!("Specified transaction {} for client {} is not being disputed.", self.transaction_id, self.client_id).into()) 
                }
            }
            None => {
                Err(format!("No transaction {} found for client {}", self.transaction_id, self.client_id).into())
            }
        }
    }

    fn requested_client_id(&self) -> ClientID {
        self.client_id
    }

    fn transaction_id(&self) -> TransactionID {
        self.transaction_id
    }

    fn amount(&self) -> Option<Amount> {
        None
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
        assert_eq!(client.client_history.get(&1).unwrap().amount, ammount)
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
        assert_eq!(client.client_history.get(&1).unwrap().amount, ammount)
    }

    #[test]
    fn withdrawal_45_7611_from_a_client_with_insufficent_funds() {
        let ammount = 35.7611;
        let mut client = Client::new(157);
        client.available = 30.0000;
        let transaction = Withdrawal::new(1, 157, ammount);

        // verify it errors. Don't care what the error is now becuase of simple error handling in place.
        transaction.execute(&mut client).unwrap_err();
        // verify available is still the same
        assert_eq!(client.available, 30.0000);
        // verify the withdrawal is not added since it is invalid
        assert_eq!(client.client_history.get(&1), None);
    }

    #[test]
    fn dispute_transaction() {
        let mut client = Client::new(157);
        client.available = 10.0;
        let deposit = Deposit::new(1, 157, 5.0);
        let dispute = Dispute::new(1, 157);

        deposit.execute(&mut client).unwrap();
        dispute.execute(&mut client).unwrap();
        assert_eq!(client.available, 10.0000);
        assert_eq!(client.held, 5.0000);
        assert_eq!(client.total(), 15.0000);
        assert_eq!(
            client.client_history.get(&1),
            Some(&History {
                amount: 5.0,
                dispute: true
            })
        );
    }

    #[test]
    fn resolve_dispute() {
        let mut client = Client::new(157);
        client.available = 10.0;
        let deposit = Deposit::new(1, 157, 5.0);
        let dispute = Dispute::new(1, 157);
        let resolve = Resolve::new(1, 157);

        deposit.execute(&mut client).unwrap();
        dispute.execute(&mut client).unwrap();
        resolve.execute(&mut client).unwrap();
        assert_eq!(client.available, 15.0000);
        assert_eq!(client.held, 0.0000);
        assert_eq!(client.total(), 15.0000);
        assert_eq!(
            client.client_history.get(&1),
            Some(&History {
                amount: 5.0,
                dispute: false
            })
        );
    }

    #[test]
    fn chargeback_dispute() {
        let mut client = Client::new(157);
        client.available = 10.0;
        let deposit = Deposit::new(1, 157, 5.0);
        let dispute = Dispute::new(1, 157);
        let chargeback = Chargeback::new(1, 157);

        deposit.execute(&mut client).unwrap();
        dispute.execute(&mut client).unwrap();
        chargeback.execute(&mut client).unwrap();
        assert_eq!(client.available, 10.0000);
        assert_eq!(client.held, 0.0000);
        assert_eq!(client.total(), 10.0000);
        assert!(client.locked);
        assert_eq!(
            client.client_history.get(&1),
            Some(&History {
                amount: 5.0,
                dispute: false
            })
        );
    }

    #[test]
    fn deposit_to_locked_account_errors() {
        let ammount = 345.4823;
        let mut client = Client::new(157);
        client.locked = true;
        let transaction = Deposit::new(1, 157, ammount);

        // Loose error handling in place. Just verify an error is returned
        transaction.execute(&mut client).unwrap_err();
    }

    #[test]
    fn withdrawal_from_locked_account_errors() {
        let ammount = 345.4823;
        let mut client = Client::new(157);
        client.locked = true;
        let transaction = Withdrawal::new(1, 157, ammount);

        // Loose error handling in place. Just verify an error is returned
        transaction.execute(&mut client).unwrap_err();
    }

    #[test]
    fn dispute_on_locked_account_errors() {
        let mut client = Client::new(157);
        client.locked = true;
        let transaction = Dispute::new(1, 157);

        // Loose error handling in place. Just verify an error is returned
        transaction.execute(&mut client).unwrap_err();
    }

    #[test]
    fn resolve_on_locked_account_errors() {
        let mut client = Client::new(157);
        client.locked = true;
        let transaction = Resolve::new(1, 157);

        // Loose error handling in place. Just verify an error is returned
        transaction.execute(&mut client).unwrap_err();
    }

    #[test]
    fn chargeback_on_locked_account_errors() {
        let mut client = Client::new(157);
        client.locked = true;
        let transaction = Chargeback::new(1, 157);

        // Loose error handling in place. Just verify an error is returned
        transaction.execute(&mut client).unwrap_err();
    }

    #[test]
    fn dispute_on_account_invalid_tx_errors() {
        let mut client = Client::new(157);
        let transaction = Dispute::new(1, 157);

        // Loose error handling in place. Just verify an error is returned
        transaction.execute(&mut client).unwrap_err();
    }

    #[test]
    fn resolve_on_account_invalid_tx_errors() {
        let mut client = Client::new(157);
        let transaction = Resolve::new(1, 157);

        // Loose error handling in place. Just verify an error is returned
        transaction.execute(&mut client).unwrap_err();
    }

    #[test]
    fn chargeback_on_account_invalid_tx_errors() {
        let mut client = Client::new(157);
        let transaction = Chargeback::new(1, 157);

        // Loose error handling in place. Just verify an error is returned
        transaction.execute(&mut client).unwrap_err();
    }

    #[test]
    fn dispute_on_account_tx_already_disputed_errors() {
        let mut client = Client::new(157);
        let deposit = Deposit::new(1, 157, 1.0);
        let dispute1 = Dispute::new(1, 157);
        let dispute2 = Dispute::new(1, 157);
        deposit.execute(&mut client).unwrap();
        dispute1.execute(&mut client).unwrap();
        // Loose error handling in place. Just verify an error is returned
        dispute2.execute(&mut client).unwrap_err();
    }

    #[test]
    fn resolve_on_account_undisputed_tx_errors() {
        let mut client = Client::new(157);
        let deposit = Deposit::new(1, 157, 1.0);
        let resolve = Resolve::new(1, 157);
        deposit.execute(&mut client).unwrap();
        // Loose error handling in place. Just verify an error is returned
        resolve.execute(&mut client).unwrap_err();
    }

    #[test]
    fn chargeback_on_account_undisputed_tx_errors() {
        let mut client = Client::new(157);
        let deposit = Deposit::new(1, 157, 1.0);
        let chargeback = Chargeback::new(1, 157);
        deposit.execute(&mut client).unwrap();
        // Loose error handling in place. Just verify an error is returned
        chargeback.execute(&mut client).unwrap_err();
    }

}
