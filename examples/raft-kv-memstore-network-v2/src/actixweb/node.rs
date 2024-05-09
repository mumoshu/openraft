#![allow(clippy::uninlined_format_args)]
#![deny(unused_qualifications)]

use actix_web::middleware;
use actix_web::middleware::Logger;
use actix_web::HttpServer;

use crate::app::App;
use crate::typ;
use crate::actixweb::raft;
use crate::actixweb::management;
use crate::actixweb::api;

pub type NodeId = u64;

pub async fn start_raft_node(app: App, http_addr: String) -> std::io::Result<()> {
    // Start the actix-web server.
    let server = HttpServer::new(move || {
        actix_web::App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::Compress::default())
            .app_data(app.clone())
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
            .service(api::write)
            .service(api::read)
            .service(api::consistent_read)
    });

    let x = server.bind(http_addr)?;

    x.run().await
}
