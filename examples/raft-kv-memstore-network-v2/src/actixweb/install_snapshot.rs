use std::fmt;

use openraft::RaftTypeConfig;
use openraft::SnapshotMeta;
use openraft::Vote;
use openraft::Snapshot;

/// An RPC sent by the Raft leader to send chunks of a snapshot to a follower (§7).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct InstallSnapshotRequest<C: RaftTypeConfig> {
    pub vote: Vote<C::NodeId>,
    pub uploaded_snapshot_id: i128,
}

impl<C: RaftTypeConfig> fmt::Display for InstallSnapshotRequest<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InstallSnapshotRequest {{ vote:{} }}",
            self.vote,
        )
    }
}

/// The response to an `InstallSnapshotRequest`.
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(derive_more::Display)]
#[display(fmt = "{{vote:{}}}", vote)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(bound = ""))]
pub struct InstallSnapshotResponse<C: RaftTypeConfig> {
    pub vote: Vote<C::NodeId>,
}

/// The response to `Raft::install_full_snapshot` API.
#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(derive_more::Display)]
#[display(fmt = "SnapshotResponse{{vote:{}}}", vote)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(bound = ""))]
pub struct SnapshotResponse<C: RaftTypeConfig> {
    pub vote: Vote<C::NodeId>,
}

impl<C: RaftTypeConfig> SnapshotResponse<C> {
    pub fn new(vote: Vote<C::NodeId>) -> Self {
        Self { vote }
    }
}

impl<C> From<SnapshotResponse<C>> for InstallSnapshotResponse<C>
where C: RaftTypeConfig
{
    fn from(snap_resp: SnapshotResponse<C>) -> Self {
        Self { vote: snap_resp.vote }
    }
}
