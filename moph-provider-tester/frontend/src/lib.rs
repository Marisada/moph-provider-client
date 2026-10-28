mod fetch;
mod loader;
mod route;

use dominator::{clone, events, html, link, Dom, routing};
use futures_signals::signal::{Mutable, SignalExt};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Url;

#[wasm_bindgen(start)]
pub fn main_js() {
    wasm_logger::init(wasm_logger::Config::default());

    console_error_panic_hook::set_once();
    log::info!("wasm logging enabled");

    dominator::append_dom(&dominator::get_id("app"), render());
}

fn render() -> Dom {
    html!("div", {
        .child_signal(routing::url().signal_cloned().map(|url| {
            route::Route::from_url(&url, &Url::new(&url).map(|u| u.host()).unwrap_or_default())
        }).dedupe_cloned().map_future(move |route| render_inner(route.clone())))
    })
}

async fn render_inner(route: route::Route) -> Dom {
    log::debug!("Render {}", route.string());
    match route {
            route::Route::Root => {
                route::Route::Index.hard_redirect();
                Dom::empty()
            }
            route::Route::NotFound { path } => html!("div", {.text(&path).text(" NOT FOUND")}),
            route::Route::Index => index(),
            route::Route::Provider { hash_nonce } => provider(hash_nonce),
            route::Route::Info => html!("div", {.text("SUCCESS !!")}),
            route::Route::Greet => greet(),
            route::Route::External { path } => html!("div", {.text(&path).text(" NOT FOUND")}),
    }
}

fn index() -> Dom {
    // Generate new state every render
    let oauth_uri = Mutable::new(String::new());
    let oauth_uri_nonce = Mutable::new(String::new());
    let loader = loader::AsyncLoader::new();
    loader.load(clone!(oauth_uri, oauth_uri_nonce => async move {
        match fetch::get_text_from_url("api/oauth").await {
            Ok(Some(uri)) => oauth_uri.set(uri),
            Ok(None) => oauth_uri.set(String::from("/")),
            Err(e) => log::error!("{e}"),
        }
        match fetch::get_text_from_url("api/oauth-nonce").await {
            Ok(Some(uri)) => oauth_uri_nonce.set(uri),
            Ok(None) => oauth_uri_nonce.set(String::from("/")),
            Err(e) => log::error!("{e}"),
        }
    }));

    html!("div", {
        .style("padding","10px")
        .text("Hello World !!!")
        .children([
            html!("br"),
            html!("br"),
            link!("#/greet", {
                .text("Open greeting page")
            }),
            html!("br"),
            html!("br"),
            html!("button", {
                .attr("type", "button")
                .children([
                    html!("img", {
                        .attr("src","logo-provider-id.webp")
                        .style("height","40px")
                    }),
                    html!("span", {
                        .style("vertical-align","middle")
                        .text("เข้าใช้งานด้วย Provider ID")
                    }),
                ])
                .event(clone!(oauth_uri => move |_:events::Click| {
                    if let Some(w) = web_sys::window() {
                        w.location().set_href(&oauth_uri.lock_ref()).unwrap();
                    }
                }))
            }),
            html!("div", {
                .style("font-size","small")
                .text_signal(oauth_uri.signal_cloned())
            }),
            html!("br"),
            html!("button", {
                .attr("type", "button")
                .children([
                    html!("img", {
                        .attr("src","logo-provider-id.webp")
                        .style("height","40px")
                    }),
                    html!("span", {
                        .style("vertical-align","middle")
                        .text("เข้าใช้งานด้วย Provider ID + State")
                    }),
                ])
                .event(clone!(oauth_uri_nonce => move |_:events::Click| {
                    if let Some(w) = web_sys::window() {
                        w.location().set_href(&oauth_uri_nonce.lock_ref()).unwrap();
                    }
                }))
            }),
            html!("div", {
                .style("font-size","small")
                .text_signal(oauth_uri_nonce.signal_cloned())
            }),
            html!("br"),
            html!("button", {
                .attr("type", "button")
                .text("REFRESH URI ปุ่มเข้าใช้งานด้วย Provider ID + State")
                .event(clone!(oauth_uri_nonce => move |_:events::Click| {
                    loader.load(clone!(oauth_uri_nonce => async move {
                        match fetch::get_text_from_url("api/oauth-nonce").await {
                            Ok(Some(uri)) => oauth_uri_nonce.set(uri),
                            Ok(None) => oauth_uri_nonce.set(String::from("/")),
                            Err(e) => log::error!("{e}"),
                        }
                    }));
                }))
            }),
        ])
    })
}

fn greet() -> Dom {
    let greeting = Mutable::new(String::new());
    let loader = loader::AsyncLoader::new();
    loader.load(clone!(greeting => async move {
        match fetch::get_text_from_url("api/greet").await {
            Ok(Some(id)) => greeting.set(id),
            Ok(None) => greeting.set(String::from("Nothing")),
            Err(e) => greeting.set(e),
        }
    }));
    html!("div", {
        .text("RESULT: ")
        .text_signal(greeting.signal_cloned())
        .children([
            html!("br"),
            link!("#/index", {
                .text("Return to index")
            })
        ])
    })
}

fn provider(hash_nonce : String) -> Dom {
    let (hash_cid, nonce) = {
        let split = hash_nonce.split('-').collect::<Vec<&str>>();
        if split.len() == 2 {
            (split[0], split[1])
        } else {
            ("", "")
        }
    };
    html!("div", {
        .text("hash of cid: ")
        .text(hash_cid)
        .child(html!("br"))
        .text("nonce: ")
        .text(nonce)
        .children([
            html!("br"),
            html!("a", {
                .attr("href","#/index")
                .text("Return to index")
            })
        ])
    })   
}
