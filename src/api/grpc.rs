use std::error::Error;

use tonic::transport::Channel;

use crate::{
    api::grpc::api::aurora_launcher_service_client::AuroraLauncherServiceClient, config::CONFIG,
};

pub mod api {
    tonic::include_proto!("aurora_launcher.rpc");
}

struct ApiManager {
    client: AuroraLauncherServiceClient<Channel>,
}

impl ApiManager {
    async fn default() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: AuroraLauncherServiceClient::connect(CONFIG.api.grpc.clone()).await?,
        })
    }
}
