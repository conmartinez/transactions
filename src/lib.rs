use std::io::Read;

use client::ClientStore;
use csv::{ReaderBuilder, Trim};
use serde::{self, Deserialize, Serialize};
use transaction::Transaction;

pub mod client;
mod error;
mod transaction;

type ClientID = u16;
type TransactionID = u32;
type Amount = f64;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename = "type")]
enum CsvLineType {
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdrawal")]
    Withdrawal,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct CsvLine {
    t_type: CsvLineType,
    client: ClientID,
    tx: TransactionID,
    amount: Amount,
}

pub fn handle_transactions_from_reader<R>(reader: R, store: &mut ClientStore)
where
    R: Read,
{
    let mut csv_reader = ReaderBuilder::new().trim(Trim::All).from_reader(reader);
    for result in csv_reader.deserialize() {
        let current: CsvLine = result.unwrap();
        let transaction: Box<dyn Transaction> = current.into();
        let _ = store
            .execute(transaction.as_ref())
            .map_err(|err| eprintln!("Couldn't handle transaction: {}", err));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CsvLineType;
    use csv::ReaderBuilder;

    #[test]
    fn de_deposit() {
        let data = "t_type,client,tx,amount\ndeposit,1,1,15\n";
        let expected = CsvLine {
            t_type: CsvLineType::Deposit,
            client: 1,
            tx: 1,
            amount: 15.0,
        };
        let mut reader = ReaderBuilder::new().from_reader(data.as_bytes());
        let mut results = vec![];
        for result in reader.deserialize::<CsvLine>() {
            results.push(result.unwrap())
        }

        assert_eq!(results.len(), 1);
        let result = results.get(0).unwrap();
        assert_eq!(result, &expected);
    }

    #[test]
    fn de_withdrawal() {
        let data = "t_type,client,tx,amount\nwithdrawal,1,1,15\n";
        let expected = CsvLine {
            t_type: CsvLineType::Withdrawal,
            client: 1,
            tx: 1,
            amount: 15.0,
        };
        let mut reader = ReaderBuilder::new().from_reader(data.as_bytes());
        let mut results = vec![];
        for result in reader.deserialize::<CsvLine>() {
            results.push(result.unwrap())
        }

        assert_eq!(results.len(), 1);
        let result = results.get(0).unwrap();
        assert_eq!(result, &expected);
    }

    #[test]
    fn de_withdrawal_and_deposit() {
        let data = "t_type,client,tx,amount\nwithdrawal,1,1,15\ndeposit,1,1,15\n";
        let expected_withdrawal = CsvLine {
            t_type: CsvLineType::Withdrawal,
            client: 1,
            tx: 1,
            amount: 15.0,
        };
        let expected_deposit = CsvLine {
            t_type: CsvLineType::Deposit,
            client: 1,
            tx: 1,
            amount: 15.0,
        };
        let mut reader = ReaderBuilder::new().from_reader(data.as_bytes());
        let mut results = vec![];
        for result in reader.deserialize::<CsvLine>() {
            results.push(result.unwrap())
        }

        assert_eq!(results.len(), 2);
        let result_withdrawal = results.get(0).unwrap();
        assert_eq!(result_withdrawal, &expected_withdrawal);
        let result_deposit = results.get(1).unwrap();
        assert_eq!(result_deposit, &expected_deposit);
    }
}
