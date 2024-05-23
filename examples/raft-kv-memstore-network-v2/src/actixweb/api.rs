use actix_web::error::ErrorInternalServerError;
use actix_web::post;
use actix_web::web;
use actix_web::web::Data;
use actix_web::Responder;
use openraft::error::CheckIsLeaderError;
use openraft::error::Infallible;
use openraft::error::RaftError;
use web::Json;

use crate::httpapp::App;
use crate::store::Request;
use crate::TypeConfig;

/**
 * Application API
 *
 * This is where you place your application, you can use the example below to create your
 * API. The current implementation:
 *
 *  - `POST - /write` saves a value in a key and sync the nodes.
 *  - `POST - /read` attempt to find a value from a given key.
 */
#[post("/write")]
pub async fn write(app: Data<App>, req: Json<Request>) -> actix_web::Result<impl Responder> {
    let response = app.raft.client_write(req.0).await;
    Ok(Json(response))
}

#[post("/local_read")]
pub async fn local_read(app: Data<App>, req: Json<Request>) -> actix_web::Result<impl Responder> {
    let state_machine = app.state_machine.state_machine.lock().unwrap();
    match req.0 {
        Request::Get { key, version } => {
            let value = state_machine.data.get(&key).cloned();
            let res: Result<String, Infallible> = Ok(value.unwrap_or_default());
            Ok(Json(res))
        }
        _ => {
            let res: Result<String, Infallible> = Ok("Invalid request".to_string());
            Ok(Json(res))
        }
    }
}

#[post("/read")]
pub async fn read(app: Data<App>, req: Json<String>) -> actix_web::Result<impl Responder> {
    let state_machine = app.state_machine.state_machine.lock().unwrap();
    let key = req.0;
    let value = state_machine.data.get(&key).cloned();

    let res: Result<String, Infallible> = Ok(value.unwrap_or_default());
    Ok(Json(res))
}

#[post("/consistent_read")]
pub async fn consistent_read(app: Data<App>, req: Json<Request>) -> actix_web::Result<impl Responder> {
    let ret = app.raft.ensure_linearizable().await;

    match ret {
        Ok(_) => {
            let state_machine = app.state_machine.state_machine.lock().unwrap();
            match req.0 {
                Request::Get { key, version } => {
                    let value = state_machine.data.get(&key).cloned();
                    let res: Result<Option<String>, crate::store::Error> = Ok(value);
                    let r = res.map(|v| Json(crate::Response{value:v}))
                        .map_err(|e| ErrorInternalServerError(e));
                    r
                }
                _ => {
                    Err(ErrorInternalServerError("Invalid request"))
                    // let res: Result<String, crate::store::Error> = Ok("Invalid request".to_string());
                    // let res = crate::Response{value:res};
                    // Ok(Json(res))
                }
            }
        }
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
