use self::cluster::cluster_client::ClusterClient;
use self::cluster::*;
use crate::client::{Proof, Settings, Task};

pub mod cluster {
    tonic::include_proto!("cluster");
}

pub struct GrpcClient {
    id: String,
    upstream: String,
}

impl GrpcClient {
    pub fn from_config(config: &Settings) -> Self {
        Self {
            id: config.prover_id.clone(),
            upstream: config.upstream.clone(),
        }
    }

    // TODO:
    pub async fn poll_task(&self) -> Result<Task, anyhow::Error> {
        Ok(Task { id: "1".to_string() })
    }

    // TODO: only pass task_id here
    pub async fn submit(&self, task_id: &str, proof: Proof) -> Result<(), anyhow::Error> {
        let mut client = ClusterClient::connect(self.upstream).await?;

        let request = tonic::Request::new(SubmitProofRequest {});

        log::info!("prover({:?}) submiting result for task({:?})", self.id, task_id);
        log::debug!("proof: {:?}", proof);

        // if error, log error here instead of outer. because of we want an async submission.
        match client.submit_proof(request).await {
            Ok(_) => {
                log::info!("prover({:?}) submit result for task({:?}) successfully", self.id, task_id);
                Ok(())
            }
            Err(e) => {
                log::error!("{}", e);
                return Err::new(e);
            }
        }
    }
}
