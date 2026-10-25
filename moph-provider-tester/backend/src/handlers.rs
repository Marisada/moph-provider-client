use axum::{extract::{Query, State}, response::Redirect};
use moph_provider_client::response::OauthRedirectParams;

pub async fn greet_handler() -> &'static str {
    "Nice to meet you!"
}

pub async fn get_oauth_uri_handler(State(state): State<crate::AppState>) -> String {
    state.client.generate_oauth_uri()
}

pub async fn get_oauth_uri_with_nonce_handler(State(state): State<crate::AppState>) -> String {
    let (uri, nonce) = state.client.generate_oauth_uri_with_state();
    state.nonce_set.lock().await.insert(nonce);
    uri
}

pub async fn provider_auth_fast(
    Query(params): Query<OauthRedirectParams>,
    State(state): State<crate::AppState>,
) -> Redirect {
    tracing::debug!("params = {:?}", &params);
    if let Some(code) = params.code.as_ref() {
        if let Some(param_state) = params.state.as_ref() {
            if let Some(nonce) = state.nonce_set.lock().await.take(param_state) {
                match state.client.get_hash_cid_uppercase_from_code(&code).await {
                    Ok(Some(hash_cid)) => {
                        Redirect::to(&["/#/provider/", &hash_cid, "-", &nonce].concat())
                    }
                    Ok(None) => {
                        tracing::warn!("Nothing returned");
                        Redirect::to("/#/index")
                    }
                    Err(e) => {
                        tracing::error!("{e}");
                        Redirect::to("/#/index")
                    }
                }
            } else {
                tracing::warn!("Nonce has been used / Not found");
                Redirect::to("/#/index")
            }
        } else {
            match state.client.get_hash_cid_uppercase_from_code(&code).await {
                Ok(Some(hash_cid)) => {
                    Redirect::to(&["/#/provider/", &hash_cid].concat())
                }
                Ok(None) => {
                    tracing::warn!("Nothing returned");
                    Redirect::to("/#/index")
                }
                Err(e) => {
                    tracing::error!("{e}");
                    Redirect::to("/#/index")
                }
            }
        }
    } else {
        tracing::warn!("No Code returned");
        Redirect::to("/#/index")
    }
}

pub async fn provider_auth_full(
    Query(params): Query<OauthRedirectParams>,
    State(state): State<crate::AppState>,
) -> Redirect {
    tracing::debug!("params = {:?}", &params);
    if let Some(param_state) = params.state.as_ref() {
        if let Some(nonce) = state.nonce_set.lock().await.take(param_state) {
            match state.client.get_all_response(params).await {
                Ok(all) => {
                    tracing::debug!("all = {:?}", &all);
                    if let Some(hash_cid) = all.get_hash_cid_uppercase() {
                        Redirect::to(&["/#/provider/", &hash_cid, "-", &nonce].concat())
                    } else {
                        tracing::warn!("hash_cid not found");
                        Redirect::to("/#/index")
                    }
                }
                Err(e) => {
                    tracing::error!("{e}");
                    Redirect::to("/#/index")
                }
            }
        } else {
            tracing::error!("Nonce has been used / Not found");
            Redirect::to("/#/index")
        }
    } else {
        match state.client.get_all_response(params).await {
            Ok(all) => {
                tracing::debug!("all = {:?}", &all);
                if let Some(hash_cid) = all.get_hash_cid_uppercase() {
                    Redirect::to(&["/#/provider/", &hash_cid].concat())
                } else {
                    tracing::warn!("hash_cid not found");
                    Redirect::to("/#/index")
                }
            }
            Err(e) => {
                tracing::error!("{e}");
                Redirect::to("/#/index")
            }
        }
    }
}
