use serde_derive::Serialize;

use crate::MophProviderIdClient;

#[derive(Serialize)]
pub(crate) struct MophIdAccessForm {
    grant_type: &'static str,
    code: String,
    redirect_uri: String,
    client_id: String,
    client_secret: String,
}

impl MophIdAccessForm {
    pub(crate) fn new(code: &str, client: &MophProviderIdClient) -> Self {
        Self {
            grant_type: "authorization_code",
            code: code.to_owned(),
            redirect_uri: client.redirect_uri.clone(),
            client_id: client.health_id_client.clone(),
            client_secret: client.health_id_key.clone(),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct ProviderIdAccessForm {
    client_id: String,
    secret_key: String,
    token_by: &'static str,
    token: String,

}

impl ProviderIdAccessForm {
    pub(crate) fn new(health_id_access_token: &str, client: &MophProviderIdClient) -> Self {
        Self {
            client_id: client.provider_id_client.clone(),
            secret_key: client.provider_id_key.clone(),
            token_by: "Health ID",
            token: health_id_access_token.to_owned(),
            
        }
    }
}
