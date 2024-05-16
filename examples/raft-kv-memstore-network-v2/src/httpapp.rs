use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tokio::sync::oneshot;

use crate::api;
use crate::httprouter::HttpRouter;
use crate::typ;
use crate::NodeId;
use crate::StateMachineStore;

use crate::actixweb::raft;
use crate::actixweb::management;

use actix_web::middleware;
use actix_web::middleware::Logger;
use actix_web::HttpServer;
use actix_web::web::Data;

pub type Path = String;
pub type Payload = String;
pub type ResponseTx = oneshot::Sender<String>;
pub type RequestTx = mpsc::UnboundedSender<(Path, Payload, ResponseTx)>;

/// Representation of an application state.
#[derive(Clone)]
pub struct App {
    pub id: NodeId,
    pub raft: typ::Raft,
    pub addr: String,

    pub router: HttpRouter,

    pub state_machine: Arc<StateMachineStore>,
}

impl App {
    pub fn new(id: NodeId, raft: typ::Raft, addr: String, router: HttpRouter, state_machine: Arc<StateMachineStore>) -> Self {
        Self {
            id,
            raft,
            addr,
            router,
            state_machine,
        }
    }

    pub async fn run(mut self) -> Option<()> {
        let addr = self.addr.clone();
        
        // Start the actix-web server.
        let server = HttpServer::new(move || {
            actix_web::App::new()
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .wrap(middleware::Compress::default())
                .app_data(Data::new(self.clone()))
                // raft internal RPC
                .service(raft::append)
                .service(raft::snapshot)
                .service(raft::vote)
                // admin API
                .service(management::init)
                .service(management::add_learner)
                .service(management::change_membership)
                .service(management::metrics)
                // application API
                .service(crate::actixweb::api::write)
                .service(crate::actixweb::api::read)
                // .service(api::consistent_read)
        });

        let x = server.bind(addr);
        let x = x.ok().unwrap();

        x.run().await.ok()
    }
}
