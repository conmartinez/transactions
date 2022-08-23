# Transactions

Read transactions from a csv and handle transactions including deposits, withdrawals, disputes, callbacks, and resolutions.

## Usage: 
cargo run -- <TRANSACTIONS>.csv > <OUTPUT>.csv

The final state of clients after the transactions run will be output to stdout. 
If errors occur during processing, they will be print to stderr. If processing can still occur
after and error, it will and transactions that cause an error will be ignored. 

## Design:
It is import to seperate items that may change in the future into distinct elements 
and contain an ripple effects a change to one of these may have. For this reason,
structures were created specifically for Data Ingestion, Executing Transactions, and
Client Account Storage.

### Data Ingestion:
Seperate data structures are present, whose sole purpose is for data infestion. This is to keep
the data ingestion (interface) contained. Any future changes to the input interface will most affect 
these data structures with minimal effects else where.

### Executing Transactions:
Transaction operate on accounts to allow for new transactions to be added with ease. 
Transactions are seperated for the Client accounts (internal storage) so that different types of Transaction can
be added, removed, or modified as needed with limited changes to the internal representation. However this
means, Transactions must know a lot about the underlying internal storage structures (Client's). This could 
posiblly be resolved with traits. However the seperation of concerns is more valuable than future headaches caused
the coupling here.

### Client Account Storage:
Clients are the internal representation of the funds and status of a client's account. Seperating these 
data structures out, means they only don't have to be concern input data format or ongoing transactions. 

## Future Improvements:

### Error Handling:
Current error handle casts all errors to string. In the future it would be nice to tell these errors apart. 
Adding an ErrorKind enum to the error used throughout this crate would allow the caller to determine the cause/
type/kind of error. Example: ClientNotFound or DeserializationError. This would also allow callers to finer controll
over recoverable/unrecoverable errors.

### Output Structures: 
The ouput structures right now are the same as the Client Account Storage structures. This creates a coupling of
the internal client representation and the programs outputs. Since we don't want internal changes to affect the program's 
output or output(interface) changes to dictact internal representation. Seperate output interface structs should be 
implemented. 

### Async and multithreading
A lot of transaction can occur and processed at a time. Implementing a Read-Write locking mechanism based
on Client-ID would allow processing of multiple clients at the same time possible since the current
implementation does not allow transfers, transfer funds from one account to another to occur. If transfers
were allowed some saftey mechanism would need to be in place to avoid deadlock. 