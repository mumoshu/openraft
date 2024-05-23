use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;

use openraft::error::NetworkError;
use openraft::error::RemoteError;

use crate::{encode, decode};
use crate::typ::RaftError;
use crate::NodeId;

use awc::{http::header, Client, Connector};

/// A HTTP-based router
#[derive(Clone)]
#[derive(Default)]
pub struct HttpRouter {
    pub targets: Arc<Mutex<BTreeMap<NodeId, String>>>,
}

impl HttpRouter {
    fn get_addr(&self, id: NodeId) -> Option<String> {
        let targets = self.targets.lock().unwrap();
        let addr = targets.get(&id);
        let addr = addr.cloned();
        addr
    }

    /// Send request `Req` to target node `to`, and wait for response `Result<Resp, RaftError<E>>`.
    pub async fn send<Req, Resp>(&self, to: NodeId, path: &str, req: Req) -> Result<Resp, reqwest::Error>
    where
        Req: serde::Serialize,
        Resp: serde::de::DeserializeOwned,
    {
        let addr = self.get_addr(to);
        let addr = addr.unwrap();

        let url = format!("http://{}/{}", addr, path);

        let client = reqwest::Client::new();

        let resp = client.post(url).json(&req).send().await?;

        resp.json().await

        // let res =
        //     resp.json().await;
            
        // let res = res.unwrap();

        // Ok(res)
    }
}
