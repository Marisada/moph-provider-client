use axum::{extract::{Query, State}, response::Redirect};
use moph_provider_client::response::OauthRedirectParams;
use ulid::Ulid;

pub async fn greet_handler() -> &'static str {
    "Nice to meet you!"
}

pub async fn get_oauth_uri_handler(State(app): State<crate::AppState>) -> String {
    app.client.generate_oauth_uri()
}

pub async fn get_oauth_uri_with_nonce_handler(State(app): State<crate::AppState>) -> String {
    let uid = Ulid::generate();
    let state = uid.to_string();
    let Ulid(id) = uid;
    let uri = app.client.generate_oauth_uri_with_state(&state);
    app.nonce_set.lock().await.insert(id);
    uri
}

pub async fn provider_auth_fast(
    Query(params): Query<OauthRedirectParams>,
    State(app): State<crate::AppState>,
) -> Redirect {
    tracing::debug!("params = {:?}", &params);
    if let Some(code) = params.code.as_ref() {
        if let Some(param_state) = params.state.as_ref() {
            if let Ok(Ulid(id)) = Ulid::from_string(param_state) {
                let nonce_opt = app.nonce_set.lock().await.take(&id);
                if nonce_opt.is_some() {
                    match app.client.get_hash_cid_uppercase(&code).await {
                        Ok(Some(hash_cid)) => {
                            Redirect::to(&["/#/provider/", &hash_cid, "-", param_state].concat())
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
                tracing::warn!("Malform nonce");
                Redirect::to("/#/index")
            }
        } else {
            match app.client.get_hash_cid_uppercase(&code).await {
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
    State(app): State<crate::AppState>,
) -> Redirect {
    tracing::debug!("params = {:?}", &params);
    if let Some(code) = params.code.as_ref() {
        if let Some(param_state) = params.state.as_ref() {
            if let Ok(Ulid(id)) = Ulid::from_string(param_state) {
                let nonce_opt = app.nonce_set.lock().await.take(&id);
                if nonce_opt.is_some() {
                    match app.client.get_all_response(code).await {
                        Ok(all) => {
                            tracing::debug!("all = {:?}", &all);
                            if let Some(hash_cid) = all.get_hash_cid_uppercase() {
                                Redirect::to(&["/#/provider/", &hash_cid, "-", param_state].concat())
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
                tracing::error!("Malform nonce");
                Redirect::to("/#/index")
            }
        } else {
            match app.client.get_all_response(code).await {
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
    } else {
        tracing::warn!("No Code returned");
        Redirect::to("/#/index")
    }
}
