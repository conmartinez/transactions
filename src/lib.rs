use serde::{self, Deserialize, Serialize};

mod client;
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
}
