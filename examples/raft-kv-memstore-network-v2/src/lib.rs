#![allow(clippy::uninlined_format_args)]
#![deny(unused_qualifications)]

use std::sync::Arc;

use openraft::Config;

use crate::app::App;
use crate::router::SimulatedRouter;
use crate::store::Request;
use crate::store::Response;
use crate::store::StateMachineData;

pub mod router;
pub mod httprouter;
pub mod httpapp;
pub mod httpnetwork;

pub mod actixweb;
pub mod api;
pub mod app;
pub mod network;
pub mod store;
pub mod client;

pub type NodeId = u64;

openraft::declare_raft_types!(
    /// Declare the type configuration for example K/V store.
    pub TypeConfig:
        D = Request,
        R = Response,
        // In this example, snapshot is just a copy of the state machine.
        // And it can be any type.
        SnapshotData = StateMachineData,
);

pub type LogStore = store::LogStore;
pub type StateMachineStore = store::StateMachineStore;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[derive(serde::Deserialize, serde::Serialize)]
#[error("infallible")]
pub enum RKVError {
    SimpleError(String)
}

pub mod typ {

    use crate::NodeId;
    use crate::TypeConfig;

    pub type Raft = openraft::Raft<TypeConfig>;

    pub type Vote = openraft::Vote<NodeId>;
    pub type SnapshotMeta = openraft::SnapshotMeta<TypeConfig>;
    pub type SnapshotData = <TypeConfig as openraft::RaftTypeConfig>::SnapshotData;
    pub type Snapshot = openraft::Snapshot<TypeConfig>;

    pub type Infallible = openraft::error::Infallible;
    pub type Fatal = openraft::error::Fatal<TypeConfig>;
    pub type RaftError<E = Infallible> = openraft::error::RaftError<TypeConfig, E>;
    pub type RPCError<E = Infallible> = openraft::error::RPCError<TypeConfig, RaftError<E>>;
    pub type StreamingError<E> = openraft::error::StreamingError<TypeConfig, E>;

    pub type RaftMetrics = openraft::RaftMetrics<TypeConfig>;

    pub type ClientWriteError = openraft::error::ClientWriteError<TypeConfig>;
    pub type CheckIsLeaderError = openraft::error::CheckIsLeaderError<TypeConfig>;
    pub type ForwardToLeader = openraft::error::ForwardToLeader<TypeConfig>;
    pub type InitializeError = openraft::error::InitializeError<TypeConfig>;

    pub type ClientWriteResponse = openraft::raft::ClientWriteResponse<TypeConfig>;
}

pub fn encode<T: serde::Serialize>(t: T) -> String {
    serde_json::to_string(&t).unwrap()
}

pub fn decode<T: serde::de::DeserializeOwned>(s: &str) -> T {
    serde_json::from_str(s).unwrap()
}

pub async fn new_raft(node_id: NodeId, router: SimulatedRouter, addr: String) -> (typ::Raft, App) {
    // Create a configuration for the raft instance.
    let config = Config {
        heartbeat_interval: 500,
        election_timeout_min: 1500,
        election_timeout_max: 3000,
        // Once snapshot is built, delete the logs at once.
        // So that all further replication will be based on the snapshot.
        max_in_snapshot_log_to_keep: 0,
        ..Default::default()
    };

    let config = Arc::new(config.validate().unwrap());

    // Create a instance of where the Raft logs will be stored.
    let log_store = LogStore::default();

    // Create a instance of where the state machine data will be stored.
    let state_machine_store = Arc::new(StateMachineStore::default());

    // Create a local raft instance.
    let raft = openraft::Raft::new(node_id, config, router.clone(), log_store, state_machine_store.clone())
        .await
        .unwrap();

    let app = App::new(node_id, raft.clone(), addr, router, state_machine_store);

    (raft, app)
}

pub async fn new_http_raft(node_id: NodeId, router: httprouter::HttpRouter, addr: String) -> (typ::Raft, httpapp::App) {
    // Create a configuration for the raft instance.
    let config = Config {
        heartbeat_interval: 500,
        election_timeout_min: 1500,
        election_timeout_max: 3000,
        // Once snapshot is built, delete the logs at once.
        // So that all further replication will be based on the snapshot.
        max_in_snapshot_log_to_keep: 0,
        ..Default::default()
    };

    let config = Arc::new(config.validate().unwrap());

    // Create a instance of where the Raft logs will be stored.
    let log_store = LogStore::default();

    // Create a instance of where the state machine data will be stored.
    let state_machine_store = Arc::new(StateMachineStore::default());

    // Create a local raft instance.
    let raft = openraft::Raft::new(node_id, config, router.clone(), log_store, state_machine_store.clone())
        .await
        .unwrap();

    let app = httpapp::App::new(node_id, raft.clone(), addr, router, state_machine_store);

    (raft, app)
}
