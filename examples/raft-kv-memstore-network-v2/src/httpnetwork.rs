use std::future::Future;

use openraft::error::RemoteError;
use openraft::error::ReplicationClosed;
use openraft::network::v2::RaftNetworkV2;
use openraft::network::RPCOption;
use openraft::raft::AppendEntriesRequest;
use openraft::raft::AppendEntriesResponse;
use openraft::raft::SnapshotResponse;
use openraft::raft::VoteRequest;
use openraft::raft::VoteResponse;
use openraft::BasicNode;
use openraft::OptionalSend;
use openraft::RaftNetworkFactory;
use openraft::Snapshot;
use openraft::SnapshotMeta;
use openraft::Vote;

use crate::httprouter::HttpRouter;
use crate::typ;
use crate::NodeId;
use crate::TypeConfig;

#[derive(Clone)]
pub struct Connection {
    router: HttpRouter,
    target: NodeId,
}

impl RaftNetworkFactory<TypeConfig> for HttpRouter {
    type Network = Connection;

    async fn new_client(&mut self, target: NodeId, node: &BasicNode) -> Self::Network {
        let mut r = HttpRouter::default();
        r.targets.lock().unwrap().insert(target, node.addr.clone());
        Connection {
            router: r,
            target,
        }
    }
}

type SnapshotReq = (Vote<NodeId>, typ::SnapshotMeta, Box<typ::SnapshotData>);

impl RaftNetworkV2<TypeConfig> for Connection {
    async fn append_entries(
        &mut self,
        req: AppendEntriesRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<AppendEntriesResponse<TypeConfig>, typ::RPCError> {
        let resp = self
            .router
            .send(self.target, "raft_append", req)
            .await;
        // print errro for debugging
        tracing::error!("RESPONSE===== {:?}", resp);
        // return the error
        resp.map_err(|e| openraft::error::RPCError::Network(openraft::error::NetworkError::new(&e)))
    }

    /// A real application should replace this method with customized implementation.
    async fn full_snapshot(
        &mut self,
        vote: Vote<NodeId>,
        snapshot: Snapshot<TypeConfig>,
        _cancel: impl Future<Output = ReplicationClosed> + OptionalSend,
        _option: RPCOption,
    ) -> Result<SnapshotResponse<TypeConfig>, typ::StreamingError<typ::Fatal>> {
        let resp = self
            .router
            .send::<SnapshotReq, SnapshotResponse<TypeConfig>>(self.target, "raft_snapshot", (vote, snapshot.meta, snapshot.snapshot))
            .await;
        let resp: Result<SnapshotResponse<TypeConfig>, typ::StreamingError<typ::Fatal>> = resp.map_err(|e| typ::StreamingError::Network(openraft::error::NetworkError::new(&e)));
        resp
    }

    async fn vote(
        &mut self,
        req: VoteRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<VoteResponse<TypeConfig>, typ::RPCError> {
        let resp = self
            .router
            .send(self.target, "raft_vote", req)
            .await;
        resp.map_err(|e| openraft::error::RPCError::Network(openraft::error::NetworkError::new(&e)))
    }
}
