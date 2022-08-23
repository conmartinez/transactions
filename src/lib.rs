use std::io::Read;

use client::ClientStore;
use csv::{ReaderBuilder, Trim};
use serde::{self, Deserialize, Deserializer, Serialize};
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
    #[serde(rename = "chargeback")]
    Chargeback,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "withdrawal")]
    Withdrawal,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct CsvLine {
    t_type: CsvLineType,
    client: ClientID,
    tx: TransactionID,
    #[serde(deserialize_with = "default_empty_amount_to_zero")]
    amount: Amount,
}

fn default_empty_amount_to_zero<'de, D>(deserializer: D) -> Result<Amount, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(0.0))
}

pub fn handle_transactions_from_reader<R>(reader: R, store: &mut ClientStore)
where
    R: Read,
{
    let mut csv_reader = ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::All)
        .from_reader(reader);
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
    fn de_dispute() {
        let data = "t_type,client,tx,amount\ndispute,1,1,\n";
        let expected = CsvLine {
            t_type: CsvLineType::Dispute,
            client: 1,
            tx: 1,
            amount: 0.0,
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
    fn de_resolve() {
        let data = "t_type,client,tx,amount\nresolve,1,1,\n";
        let expected = CsvLine {
            t_type: CsvLineType::Resolve,
            client: 1,
            tx: 1,
            amount: 0.0,
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
    fn de_chargeback() {
        let data = "t_type,client,tx,amount\nchargeback,1,1,\n";
        let expected = CsvLine {
            t_type: CsvLineType::Chargeback,
            client: 1,
            tx: 1,
            amount: 0.0,
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
    fn de_all() {
        let data = "t_type,client,tx,amount\nwithdrawal,1,1,15\ndeposit,1,1,15\ndispute,1,1,\nresolve,1,1,\nchargeback,1,1,\n";
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
        let expected_dispute = CsvLine {
            t_type: CsvLineType::Dispute,
            client: 1,
            tx: 1,
            amount: 0.0,
        };
        let expected_resolve = CsvLine {
            t_type: CsvLineType::Resolve,
            client: 1,
            tx: 1,
            amount: 0.0,
        };
        let expected_chargeback = CsvLine {
            t_type: CsvLineType::Chargeback,
            client: 1,
            tx: 1,
            amount: 0.0,
        };
        let mut reader = ReaderBuilder::new().from_reader(data.as_bytes());
        let mut results = vec![];
        for result in reader.deserialize::<CsvLine>() {
            results.push(result.unwrap())
        }

        assert_eq!(results.len(), 5);
        let result_withdrawal = results.get(0).unwrap();
        assert_eq!(result_withdrawal, &expected_withdrawal);
        let result_deposit = results.get(1).unwrap();
        assert_eq!(result_deposit, &expected_deposit);
        let result_dispute = results.get(2).unwrap();
        assert_eq!(result_dispute, &expected_dispute);
        let result_resolve = results.get(3).unwrap();
        assert_eq!(result_resolve, &expected_resolve);
        let result_chargeback = results.get(4).unwrap();
        assert_eq!(result_chargeback, &expected_chargeback);
    }
}
