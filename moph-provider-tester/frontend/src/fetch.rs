use js_sys::JsString;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, RequestInit, Response};

pub async fn get_text_from_url(url: &str) -> Result<Option<String>, String> {
    match fetch_text_api(url).await {
        Ok((response, true)) => {
            let response = response.dyn_ref::<JsString>().map(|s| s.into());
            Ok(response)
        }
        Ok((app_error, false)) => {
            let error: String = serde_wasm_bindgen::from_value(app_error).map_err(|e| e.to_string())?;
            Err(error)
        }
        Err(e) => Err(js_sys::Error::from(e).to_js_string().as_string().unwrap_or(String::from("Error"))),
    }
}

/// only GET method, return ((value, is_ok), error), not check token
async fn fetch_text_api(url: &str) -> Result<(JsValue, bool), JsValue> {
    let headers = Headers::new()?;
    headers.set("Accept", "text/plain;charset=UTF-8")?;
    let w = web_sys::window().unwrap();
    let init = RequestInit::new();
    init.set_method("GET");
    init.set_headers(&headers);
    let future = w.fetch_with_str_and_init(url, &init);

    let response = JsFuture::from(future).await?.unchecked_into::<Response>();

    if response.ok() {
        let value = JsFuture::from(response.text()?).await?;
        Ok((value, true))
    } else {
        let value = JsFuture::from(response.json()?).await?;
        Ok((value, false))
    }
}
