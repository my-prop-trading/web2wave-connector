use crate::model::{
    CreateInvoiceRequest, CreateInvoiceResponse, UpdateSubscriptionRequest,
    UpdateSubscriptionResponse,
};
use flurl::{hyper::Method, FlUrl};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::time::Duration;

pub struct Web2WaveApi {
    base_url: String,
    api_key: String,
    timeout: Duration,
}

impl Web2WaveApi {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>, timeout: Duration) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
            timeout,
        }
    }

    pub async fn create_invoce(
        &self,
        req: &CreateInvoiceRequest,
    ) -> Result<CreateInvoiceResponse, String> {
        let endpoint = "/api/subscription/invoice";
        let method = Method::POST;
        self.send_flurl_deserialized(endpoint, &method, req).await
    }

    pub async fn update_subscription(
        &self,
        subscription_id: i32,
        req: &UpdateSubscriptionRequest,
    ) -> Result<UpdateSubscriptionResponse, String> {
        let endpoint = format!("api/subscription/{subscription_id}");
        let method = Method::PUT;
        self.send_flurl_deserialized(&endpoint, &method, req).await
    }

    async fn send_flurl_deserialized<R: Serialize + Debug, T: DeserializeOwned + Debug>(
        &self,
        endpoint: &str,
        method: &Method,
        request: &R,
    ) -> Result<T, String> {
        let response = self.send_flurl(endpoint, method, request).await?;
        let result: Result<T, _> = serde_json::from_str(&response);

        let Ok(body) = result else {
            let msg = format!(
                "Failed to deserialize: {:?}. Url: {:?} {:?}. Request: {:?}. Body: {}",
                result, method, endpoint, request, response
            );
            return Err(msg);
        };

        Ok(body)
    }

    async fn send_flurl<R: Serialize + Debug>(
        &self,
        endpoint: &str,
        method: &Method,
        request: &R,
    ) -> Result<String, String> {
        let request_json = serde_json::to_string(&request).map_err(|e| format!("{:?}", e))?;
        let request_bytes: Option<Vec<u8>> = Some(request_json.clone().into_bytes());
        let flurl = self.build_flurl(endpoint);

        let result = if method == Method::GET {
            flurl.get().await
        } else if method == Method::POST {
            flurl.post(request_bytes).await
        } else if method == Method::PUT {
            flurl.put(request_bytes).await
        } else if method == Method::PATCH {
            flurl.patch(request_bytes).await
        } else if method == Method::DELETE {
            flurl.delete().await
        } else {
            panic!("not implemented");
        };

        let Ok(resp) = result else {
            return Err(format!(
                "FlUrl failed to receive_body: Url: {}. Request: {:?}. {:?}",
                endpoint,
                request_json,
                result.unwrap_err()
            ));
        };

        handle_flurl_resp(resp, Some(&request_json), endpoint, method).await
    }

    fn build_flurl(&self, endpoint: &str) -> FlUrl {
        let url = format!("{}/{}", self.base_url, endpoint);
        let flurl = FlUrl::new(&url).set_timeout(self.timeout);
        let flurl = self.add_headers(flurl);

        flurl
    }

    fn add_headers(&self, flurl: FlUrl) -> FlUrl {
        let content_str = "application/json";

        flurl
            .with_header("Content-Type", content_str)
            .with_header("Accept", content_str)
            .with_header("api_key", &self.api_key)
    }
}

async fn handle_flurl_resp(
    response: flurl::FlUrlResponse,
    request_json: Option<&str>,
    request_url: &str,
    request_method: &Method,
) -> Result<String, String> {
    let status_code = response.get_status_code();
    let result = response.receive_body().await;

    let Ok(body_bytes) = result else {
        return Err(format!("FlUrl failed to receive_body: {:?}", result.unwrap_err()).into());
    };

    let body_str = String::from_utf8(body_bytes).unwrap();

    if status_code > 299 {
        return Err(format!("Response code: {status_code:?}. Url: {request_method:?} {request_url}. Request: {request_json:?} Response: {body_str}"));
    }

    Ok(body_str)
}
