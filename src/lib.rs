pub mod error;
pub mod request;
pub mod response;
pub mod verify;

use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

static MOPH_ID_OAUTH_REDIRECT_URI: &str = "https://moph.id.th/oauth/redirect";
static MOPHID_TOKEN_API: &str = "https://moph.id.th/api/v1/token";
static MOPHID_PUBLIC_KEY_API: &str = "https://moph.id.th/api/v1/oauth/public-key";

static PROVIDERID_TOKEN_API: &str = "https://provider.id.th/api/v1/services/token";
static PROVIDERID_PROFILE_API: &str = "https://provider.id.th/api/v1/services/profile";
static PROVIDERID_PUBLIC_KEY_API: &str = "https://provider.id.th/api/v1/services/public-key";

#[derive(Default)]
pub struct MophProviderIdClientBuilder {
    redirect_uri: Option<String>,
    health_id_client: Option<String>,
    health_id_key: Option<String>,
    provider_id_client: Option<String>,
    provider_id_key: Option<String>,
}

impl MophProviderIdClientBuilder {
    pub fn redirect_uri(mut self, uri: &str) -> Self {
        self.redirect_uri = Some(uri.to_owned());
        self
    }

    pub fn health_id_client(mut self, client_id: &str) -> Self {
        self.health_id_client = Some(client_id.to_owned());
        self
    }

    pub fn health_id_key(mut self, secret_key: &str) -> Self {
        self.health_id_key = Some(secret_key.to_owned());
        self
    }

    pub fn provider_id_client(mut self, client_id: &str) -> Self {
        self.provider_id_client = Some(client_id.to_owned());
        self
    }

    pub fn provider_id_key(mut self, secret_key: &str) -> Self {
        self.provider_id_key = Some(secret_key.to_owned());
        self
    }

    pub async fn build(self) -> Result<MophProviderIdClient, error::Error> {
        let client = reqwest::Client::builder().build()?;
        let redirect_uri = self.redirect_uri.ok_or(error::Error::ClientBuilder(String::from("Redirect URI not found")))?;
        let health_id_client = self.health_id_client.ok_or(error::Error::ClientBuilder(String::from("Health ID client_id not found")))?;
        let health_id_key = self.health_id_key.ok_or(error::Error::ClientBuilder(String::from("Health ID secret_key not found")))?;
        let provider_id_client = self.provider_id_client.ok_or(error::Error::ClientBuilder(String::from("Provider ID client_id not found")))?;
        let provider_id_key = self.provider_id_key.ok_or(error::Error::ClientBuilder(String::from("Provider ID secret_key not found")))?;

        let health_id_public_str = get_health_id_public_key(&client, &health_id_client, &health_id_key).await?;
        let provider_id_public_str = get_provider_id_public_key(&client, &provider_id_client, &provider_id_key).await?;

        // dbg!(&health_id_public_str);
        // dbg!(&provider_id_public_str);

        let health_id_public = Arc::new(Mutex::new(health_id_public_str));
        let provider_id_public = Arc::new(Mutex::new(provider_id_public_str));

        Ok(MophProviderIdClient {
            client,
            redirect_uri,
            health_id_client,
            health_id_key,
            health_id_public,
            provider_id_client,
            provider_id_key,
            provider_id_public,
        })
    }
}

pub struct MophProviderIdClient {
    client: reqwest::Client,
    redirect_uri: String,
    health_id_client: String,
    health_id_key: String,
    health_id_public: Arc<Mutex<String>>,
    provider_id_client: String,
    provider_id_key: String,
    provider_id_public: Arc<Mutex<String>>,
}

impl MophProviderIdClient {
    pub fn builder() -> MophProviderIdClientBuilder {
        MophProviderIdClientBuilder::default()
    }

    /// get a full uri to open moph-id oauth page
    pub fn generate_oauth_uri(&self) -> String {
        [MOPH_ID_OAUTH_REDIRECT_URI, "?client_id=", &self.health_id_client, "&redirect_uri=", &self.redirect_uri, "&response_type=code"].concat()
    }

    /// get a full uri with state to open moph-id oauth page
    pub fn generate_oauth_uri_with_state(&self, state: &str) -> String {
        [MOPH_ID_OAUTH_REDIRECT_URI, "?client_id=", &self.health_id_client, "&redirect_uri=", &self.redirect_uri, "&response_type=code&state=", state].concat()
    }

    pub async fn get_all_response(&self, code: &str) -> Result<response::AllResponse, error::Error> {
        let mut all_response = response::AllResponse::default();

        // 1. get HealthID access token
        let moph_id_access_form = request::MophIdAccessForm::new(code, self);
        let moph_id_token_response = self.client.post(MOPHID_TOKEN_API).form(&moph_id_access_form).send().await?.json::<response::MophIdTokenesponse>().await?;
        self.verify_health_id_token(&moph_id_token_response.data.access_token).await?;

        // 2. get ProviderID access token
        let provider_id_access_form = request::ProviderIdAccessForm::new(&moph_id_token_response.data.access_token, self);
        let provider_id_token_response = self.client.post(PROVIDERID_TOKEN_API).json(&provider_id_access_form).send().await?.json::<response::ProviderIdTokenResponse>().await?;
        self.verify_provider_id_token(&provider_id_token_response.data.access_token).await?;

        // 3. get provider profile
        let provider_id_profile_response = self.client.get(PROVIDERID_PROFILE_API)
            .bearer_auth(&provider_id_token_response.data.access_token)
            .header("client-id", &self.provider_id_client)
            .header("secret-key", &self.provider_id_key)
            .send().await?
            .json::<response::ProviderIdProfileResponse>().await?;

        all_response.set_moph_id_token_response(moph_id_token_response);
        all_response.set_provider_id_token_response(provider_id_token_response);
        all_response.set_provider_id_profile_response(provider_id_profile_response);

        Ok(all_response)
    }

    pub async fn get_hash_cid_uppercase(&self, code: &str) -> Result<Option<String>, error::Error> {

        // 1. get HealthID access token
        let health_id_access_form = request::MophIdAccessForm::new(code, self);
        let health_id_token_response_value = self.client.post(MOPHID_TOKEN_API).form(&health_id_access_form).send().await?.json::<Value>().await?;

        if let Some(health_id_access_token) = health_id_token_response_value["data"]["access_token"].as_str() {
            self.verify_health_id_token(health_id_access_token).await?;

            // 2. get ProviderID access token
            let provider_id_access_form = request::ProviderIdAccessForm::new(health_id_access_token, self);
            let provider_id_token_response_value = self.client.post(PROVIDERID_TOKEN_API).json(&provider_id_access_form).send().await?.json::<Value>().await?;

            if let Some(provider_id_access_token) = provider_id_token_response_value["data"]["access_token"].as_str() {
                self.verify_provider_id_token(provider_id_access_token).await?;

                // 3. get provider profile
                let provider_id_profile_response_value = self.client.get(PROVIDERID_PROFILE_API)
                    .bearer_auth(provider_id_access_token)
                    .header("client-id", &self.provider_id_client)
                    .header("secret-key", &self.provider_id_key)
                    .send().await?
                    .json::<Value>().await?;
                let hash_cid_uppercase = provider_id_profile_response_value["data"]["hash_cid"].as_str().map(|s| s.to_ascii_uppercase());

                Ok(hash_cid_uppercase)
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    async fn verify_health_id_token(&self, token: &str) -> Result<(), error::Error> {
        if verify::verify_rs256_is_error(token, &self.health_id_public.lock().await)? {
            // Try load public key to val
            self.reload_health_id_public_key().await?;
            if verify::verify_rs256_is_error(token, &self.health_id_public.lock().await)? {
                return Err(error::Error::new_custom("Invalid HealthId access token"));
            }
        }

        Ok(())
    }

    async fn verify_provider_id_token(&self, token: &str) -> Result<(), error::Error> {
        if verify::verify_rs256_is_error(token, &self.provider_id_public.lock().await)? {
            self.reload_provider_id_public_key().await?;
            if verify::verify_rs256_is_error(token, &self.provider_id_public.lock().await)? {
                return Err(error::Error::new_custom("Invalid ProviderId access token"));
            }
        }

        Ok(())
    }

    async fn reload_health_id_public_key(&self) -> Result<(), error::Error> {
        let health_id_public = get_health_id_public_key(&self.client, &self.health_id_client, &self.health_id_key).await?;
        let mut lock = self.health_id_public.lock().await;
        // dbg!(&health_id_public);
        *lock = health_id_public;

        Ok(())
    }

    async fn reload_provider_id_public_key(&self) -> Result<(), error::Error> {
        let provider_id_public = get_provider_id_public_key(&self.client, &self.provider_id_client, &self.health_id_key).await?;
        let mut lock = self.provider_id_public.lock().await;
        // dbg!(&provider_id_public);
        *lock = provider_id_public;

        Ok(())
    }
}

async fn get_health_id_public_key(client: &reqwest::Client, client_id: &str, secret_key: &str) -> Result<String, reqwest::Error> {
    client.get(MOPHID_PUBLIC_KEY_API)
        .header("client-id", client_id)
        .header("secret-key", secret_key)
        .send().await?
        .text().await
}

async fn get_provider_id_public_key(client: &reqwest::Client, client_id: &str, secret_key: &str) -> Result<String, reqwest::Error> {
    let payload = serde_json::json!({
        "client_id": client_id,
        "secret_key": secret_key
    });
    client.post(PROVIDERID_PUBLIC_KEY_API)
        .json(&payload)
        .send().await?
        .text().await
}
