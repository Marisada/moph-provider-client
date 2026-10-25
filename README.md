# MOPH Provider ID Client

Rust client for authentication via MOPH Provider ID

This library follow [คู่มือการเชื่อมต่อระบบ Provider ID ด้วย OAuth Health ID](https://docs.google.com/document/d/1oRNVwrCe-StLwCCpKY-LsQzNzZ5qxd1z3m65uAJ3T-E) to get user profile from Provider ID of MOPH Thailand

## Requirements
1. A self-hosted URL used for redirection from the authorization page
2. Health ID client-id
3. Health ID secret-key
4. Provider ID client-id
5. Provider ID secret-key

## Usage

1. Generate a server state (`nonce` is optional)
```rust
use moph_provider_client::MophProviderIdClient;
use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;

let client = MophProviderIdClient::builder()
    .redirect_uri("http://localhost:8088/provider")
    .health_id_client("012345-67890")
    .health_id_key("abcedf")
    .provider_id_client("012345-67890")
    .provider_id_key("abcdef")
    .build()
    .unwrap()
let nonce_set = Arc::new(Mutex::new(HashSet::new()));
let state = AppState {client, nonce_set};
```

2. Serve OAUTH URI to client (example using axum) or Client using static URI without nonce
```rust
use axum::extract::State;
use crate::AppState;

// GET /api/oauth-uri
pub async fn get_oauth_uri_with_nonce_handler(State(state): State<crate::AppState>) -> String {
    // // you can generate URI without nonce
    // let uri = state.client.generate_oauth_uri();
    // // or with nonce
    let (uri, nonce) = state.client.generate_oauth_uri_with_state();
    state.nonce_set.lock().await.insert(nonce);
    uri
}
```

3. User open URI and authorized with HealthID. Redirect to "http://localhost:8088/provider?code=XXX&state=YYY"

4. Server using authorization code (+ stage) to get user's `hash_cid`

    - Use `MophProviderIdClient::get_hash_cid_uppercase_from_code()` for `hash_cid` only
    ```rust
    use axum::{extract::{Query, State}, response::Redirect};
    use moph_provider_client::request::OauthRedirectParams;

    // GET /provider
    pub async fn provider_auth(
        Query(params): Query<OauthRedirectParams>,
        State(state): State<crate::AppState>,
    ) -> Redirect {
        if let Some(code) = params.code.as_ref() {
            if let Some(param_state) = params.state.as_ref() {
                // nonce can use only once
                if let Some(nonce) = state.nonce_set.lock().await.take(param_state) {
                    match state.client.get_hash_cid_uppercase_from_code(&code).await {
                        Ok(Some(hash_cid)) => {
                            // find user and log-in user
                            Redirect::to("/main")
                        }
                        Ok(None) => {
                            println!("Not found");
                            Redirect::to("/index")
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                            Redirect::to("/index")
                        }
                    }
                } else {
                    println!("Nonce has been used");
                    Redirect::to("/index")
                }
            } else {
                println!("No state");
                Redirect::to("/index")
            }
        } else {
            println!("No code returned");
            Redirect::to("/index")
        }
    }
    ```

    - Use `MophProviderIdClient::get_all_response()` for a full response
    ```rust
    use axum::{extract::{Query, State}, response::Redirect};
    use moph_provider_client::request::OauthRedirectParams;

    // GET /provider
    pub async fn provider_auth_full(
        Query(params): Query<OauthRedirectParams>,
        State(state): State<crate::AppState>,
    ) -> Redirect {
        if let Some(param_state) = params.state.as_ref() {
            // nonce can use only once
            if let Some(nonce) = state.nonce_set.lock().await.take(param_state) {
                match state.client.get_all_response(params).await {
                    Ok(all_data) => {
                        let has_cid_option = all.get_hash_cid_uppercase();
                        // read `model::AllResponse` for more information
                        Redirect::to("/main")
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        Redirect::to("/index")
                    }
                }
            } else {
                println!("Nonce has been used");
                Redirect::to("/index")
            }
        } else {
            println!("No state");
            Redirect::to("/index")
        }
    }
    ```

## Example app and server
Read more at [moph_provider_tester](moph_provider_tester/README.md)
