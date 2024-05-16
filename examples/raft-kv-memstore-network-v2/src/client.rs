use bytes::Bytes;

use crate::httprouter;

// Txn is a transaction, which contains the keys to be read at the read version,
// and the keys and values to be written at the commit version (which is determined at the time of commit)
pub struct Txn {
    pub keys_and_values: Vec<(Bytes, Bytes)>,
    pub read_conflict_ranges: Vec<(Bytes, Bytes)>,
    pub write_conflict_ranges: Vec<(Bytes, Bytes)>,
    pub read_version: u128,

    // We don't have the commit_version within this struct,
    // because the commit_version is not known when the transaction is created.
    // pub commit_version: u128,
}

// Client is a client to a RaftKV cluster.
// It connects to one of the RaftKV nodes via RaftKV HTTP API.
// Client provides both high-level and low-level APIS.
// The high-level APIs provides a key-value store with linearizable read and write operations.
// The low-level APIs provide a way to call the RaftKV HTTP APIs directly.
pub struct Client {
    addr: String,
}
