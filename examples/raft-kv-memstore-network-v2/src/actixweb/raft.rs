use actix_web::post;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::Responder;
use openraft::raft::AppendEntriesRequest;
use crate::actixweb::install_snapshot::InstallSnapshotRequest;
use crate::store::StateMachineData;
use openraft::raft::VoteRequest;
use crate::typ;

use crate::httpapp::App;
use crate::TypeConfig;

// --- Raft communication

#[post("/raft_vote")]
pub async fn vote(app: Data<App>, req: Json<VoteRequest<TypeConfig>>) -> actix_web::Result<impl Responder> {
    let res = app.raft.vote(req.0).await;
    Ok(Json(res))
}

#[post("/raft_append")]
pub async fn append(app: Data<App>, req: Json<AppendEntriesRequest<TypeConfig>>) -> actix_web::Result<impl Responder> {
    let res = app.raft.append_entries(req.0).await;
    Ok(Json(res))
}

#[post("/raft_snapshot")]
pub async fn snapshot(
    app: Data<App>,
    req: Json<(typ::Vote, typ::SnapshotMeta, typ::SnapshotData)>,
) -> actix_web::Result<impl Responder> {
    // let snapshot = get_uploaded_snapshot(req.0.uploaded_snapshot_id).await;
    let (v, snapshot_meta, snapshot_data) = req.0;
    let snapshot = typ::Snapshot {
        meta: snapshot_meta,
        snapshot: Box::new(snapshot_data),
    };
    let res = app.raft.install_full_snapshot(v, snapshot).await;
    Ok(Json(res))
}

async fn get_uploaded_snapshot(_uploaded_snapshot_id: i128) -> openraft::Snapshot<TypeConfig> {
    // This is a placeholder for a real implementation.
    // In a real application, the snapshot should be fetched from a storage.
    openraft::Snapshot {
        meta: Default::default(),
        snapshot: Box::new(StateMachineData::default()),
    }
}