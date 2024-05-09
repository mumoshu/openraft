use actix_web::post;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::Responder;
use openraft::raft::AppendEntriesRequest;
use crate::actixweb::install_snapshot::InstallSnapshotRequest;
use openraft::raft::VoteRequest;

use crate::app::App;
use crate::TypeConfig;

// --- Raft communication

#[post("/raft-vote")]
pub async fn vote(app: Data<App>, req: Json<VoteRequest<TypeConfig>>) -> actix_web::Result<impl Responder> {
    let res = app.raft.vote(req.0).await;
    Ok(Json(res))
}

#[post("/raft-append")]
pub async fn append(app: Data<App>, req: Json<AppendEntriesRequest<TypeConfig>>) -> actix_web::Result<impl Responder> {
    let res = app.raft.append_entries(req.0).await;
    Ok(Json(res))
}

#[post("/raft-snapshot")]
pub async fn snapshot(
    app: Data<App>,
    req: Json<InstallSnapshotRequest<TypeConfig>>,
) -> actix_web::Result<impl Responder> {
    let res = app.raft.install_full_snapshot(req.0.vote, req.0.snapshot).await;
    Ok(Json(res))
}
