use std::{error::Error, sync::Arc};

use rsa::pkcs1::RsaPublicKey;
use tokio::sync::RwLock;
use tonic::{Request, Status, transport::Channel};

use crate::{
    api::grpc::api::{
        ProfileRequest, ProfileResponse, ServersResponse, UpdateRequest, UpdateResponse,
        VerifyResponse, aurora_launcher_service_client::AuroraLauncherServiceClient,
    },
    config::CONFIG,
};

pub mod api {
    tonic::include_proto!("aurora_launcher.rpc");
}

struct ApiManager {
    client: AuroraLauncherServiceClient<Channel>,
    token: Arc<RwLock<Option<String>>>,
}

impl ApiManager {
    async fn default() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: AuroraLauncherServiceClient::connect(CONFIG.api.grpc.clone()).await?,
            token: Arc::new(RwLock::new(None)),
        })
    }

    // pub async fn set_token(&self, token: Option<String>) {
    //     *self.token.write().await = token;
    // }

    pub async fn get_servers(&self) -> Result<ServersResponse, Status> {
        let mut client = self.client.clone();
        let request = Request::new(());
        Ok(client.get_servers(request).await?.into_inner())
    }

    pub async fn get_profile(&self, uuid: String) -> Result<ProfileResponse, Status> {
        let mut client = self.client.clone();
        let request = Request::new(ProfileRequest { uuid });
        Ok(client.get_profile(request).await?.into_inner())
    }

    pub async fn get_updates(&self, dir: String) -> Result<UpdateResponse, Status> {
        let mut client = self.client.clone();
        let request = Request::new(UpdateRequest { dir });
        Ok(client.get_updates(request).await?.into_inner())
    }

    async fn get_token(&self) -> Result<VerifyResponse, Status> {
        let mut client = self.client.clone();
        let response = client.get_token(Request::new(())).await?.into_inner();
        Ok(response)
    }
}
