use clap::Parser;
use raft_kv_memstore_network_v2::{new_raft, new_http_raft};
use raft_kv_memstore_network_v2::httprouter::HttpRouter;
use raft_kv_memstore_network_v2::router::SimulatedRouter;
use raft_kv_memstore_network_v2::actixweb::app::start_raft_node;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Clone, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Opt {
    #[clap(long)]
    pub id: u64,

    #[clap(long)]
    pub http_addr: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup the logger
    tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_ansi(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Parse the parameters passed by arguments.
    let options = Opt::parse();

    // let router = SimulatedRouter::default();
    // let (_raft, app) = new_raft(options.id, router, options.http_addr).await;
    // start_raft_node(app).await

    let router = HttpRouter::default();
    let (_raft, app) = new_http_raft(options.id, router, options.http_addr).await;
    match app.run().await {
        Some(()) => Ok(()),
        None => Err(std::io::Error::new(std::io::ErrorKind::Other, "Error running the app")),
    }
}
