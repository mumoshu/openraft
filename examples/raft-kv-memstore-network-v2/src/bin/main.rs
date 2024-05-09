use clap::Parser;
use raft_kv_memstore_network_v2::new_raft;
use raft_kv_memstore_network_v2::router::Router;
use raft_kv_memstore_network_v2::actixweb::node::start_raft_node;
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

    let router = Router::default();

    let (_raft, app) = new_raft(options.id, router, options.http_addr).await;

    start_raft_node(app, options.http_addr).await
}
