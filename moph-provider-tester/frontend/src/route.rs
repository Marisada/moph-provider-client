use js_sys::JsString;
use wasm_bindgen::JsCast;
use web_sys::Url;

#[derive(Clone, PartialEq)]
pub enum Route {
    Root,
    Greet,
    NotFound {
        path: String,
    },
    Index,
    Provider {
        hash_nonce: String,
    },
    Info,
    External {
        path: String,
    },
}

impl Route {
    pub fn string(&self) -> String {
        match self {
            Self::Root => "#".to_owned(),
            Self::NotFound { path } => ["#/notfound/", path].concat(),
            Self::Index => "#/index".to_owned(),
            Self::Greet => "#/greet".to_owned(),
            Self::Provider { hash_nonce } => ["#/provider/", hash_nonce].concat(),
            Self::Info => "#/info".to_owned(),
            Self::External { path } => path.to_owned(),
        }
    }

    pub fn from_url(url: &str, host: &str) -> Self {
        if url.is_empty() {
            return Self::NotFound { path: String::new() };
        }
        if url.starts_with("#/") {
            return Self::from_hash(url);
        }

        match Url::new(url).map(|u| {
            let h = u.hash();
            // NOTE "http://xxx/#" has pathname() == "/" and hash() == ""
            (h.clone(), (u.host().as_str() == host && u.pathname().as_str() == "/") && (h.starts_with("#/") || h.is_empty()))
        }) {
            Ok((hash, is_accepted)) => {
                if is_accepted {
                    Self::from_hash(&hash)
                } else {
                    Self::External { path: url.to_owned() }
                }
            }
            Err(_) => Self::External { path: url.to_owned() },
        }
    }

    pub fn from_hash(hash: &str) -> Self {
        if hash.starts_with("#/notfound/") {
            return Self::NotFound {
                path: hash.strip_prefix("#/notfound/").unwrap_or(hash).to_owned(),
            };
        }
        let url_parts = hash.split('/').collect::<Vec<&str>>();
        match url_parts.len() {
            // #
            1 => Self::Root,
            // #/xxx
            2 => match url_parts[1] {
                "" => Self::Root,
                "index" => Self::Index,
                "info" => Self::Info,
                "greet" => Self::Greet,
                _ => Self::NotFound { path: hash.to_owned() },
            }
            // #/xxx/yyy
            3 => match url_parts[1] {
                "provider" => Self::Provider { hash_nonce: url_parts[2].to_owned() },
                _ => Self::NotFound { path: hash.to_owned() },
            }
            _ => Self::NotFound { path: hash.to_owned() },
        }
    }

    pub fn hard_redirect(&self) {
        let w = web_sys::window().unwrap();
        if let Self::External { path } = self {
            if let Err(e) = w.open_with_url_and_target(path, "_blank") {
                if let Some(message) = e.dyn_ref::<JsString>().map(|s| Into::<String>::into(s)) {
                    w.document().unwrap().get_element_by_id("errormessage").unwrap().set_text_content(Some(&message));
                    log::error!("{}", message);
                }
            }
        } else {
            let location = w.location();
            if let Err(e) = location.assign(&self.string()) {
                if let Some(message) = e.dyn_ref::<JsString>().map(|s| Into::<String>::into(s)) {
                    w.document().unwrap().get_element_by_id("errormessage").unwrap().set_text_content(Some(&message));
                    log::error!("{}", message);
                }
            }
        }
    }
}
