use reqwest::header::HeaderMap;
use reqwest::{Error, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub enum EndpointType {
    POST,
    GET,
}

pub trait IntoHttpRequest {
    fn get_url(&self) -> String;
    fn get_data(&self) -> serde_json::Value;
    fn get_headers(&self) -> Option<HeaderMap>;
    fn get_endpoint_type(&self) -> EndpointType;
}

pub struct EndpointUrlAndData {
    pub url: String,
    pub data: serde_json::Value,
    pub headers: HeaderMap,
    pub endpoint_type: EndpointType,
}

impl IntoHttpRequest for EndpointUrlAndData {
    fn get_url(&self) -> String {
        self.url.clone()
    }

    fn get_data(&self) -> serde_json::Value {
        self.data.clone()
    }

    fn get_headers(&self) -> Option<HeaderMap> {
        Some(self.headers.clone())
    }

    fn get_endpoint_type(&self) -> EndpointType {
        match self.endpoint_type {
            EndpointType::POST => EndpointType::POST,
            EndpointType::GET => EndpointType::GET,
        }
    }
}

// Helper functions for making HTTP requests
async fn post_request(url: &str, data: &Value, headers: &HeaderMap) -> Result<Response, Error> {
    let client = reqwest::Client::new();

    println!("sending post {:?} {:?} {:?}", url, data, headers);

    client
        .post(url)
        .headers(headers.clone())
        .json(data)
        .send()
        .await
}

async fn get_request(url: &str, data: &Value, headers: &HeaderMap) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client
        .get(url)
        .headers(headers.clone())
        .query(&data)
        .send()
        .await
}

impl EndpointUrlAndData {
    pub async fn perform_req(&self) -> Result<Response, Error> {
        let url = &self.url;
        let data = &self.data;
        let header_map = &self.headers;
        match self.endpoint_type {
            EndpointType::POST => post_request(url, data, header_map).await,
            EndpointType::GET => get_request(url, data, header_map).await,
        }
    }

    pub async fn perform_req_typed<T>(&self) -> Result<Option<T>, Error>
    where
        T: DeserializeOwned,
    {
        let url = &self.url;
        let data = &self.data;
        let header_map = &self.headers;

        let response_result = match self.endpoint_type {
            EndpointType::POST => post_request(url, data, header_map).await,
            EndpointType::GET => get_request(url, data, header_map).await,
        };

        match response_result {
            Ok(res) => {
                // Try to deserialize the response body into type T
                match res.json::<T>().await {
                    Ok(typed_response) => Ok(Some(typed_response)),
                    Err(_) => Ok(None), // Deserialization failed, return None
                }
            }
            Err(e) => Err(e),
        }
    }
}

// Generic versions of perform_req and perform_req_typed that work with any IntoHttpRequest
pub async fn perform_req<T: IntoHttpRequest>(request: &T) -> Result<Response, Error> {
    let url = request.get_url();
    let data = request.get_data();
    let headers = request.get_headers().unwrap_or_else(|| HeaderMap::new());

    match request.get_endpoint_type() {
        EndpointType::POST => post_request(&url, &data, &headers).await,
        EndpointType::GET => get_request(&url, &data, &headers).await,
    }
}

pub async fn perform_req_typed<T, R>(request: &T) -> Result<Option<R>, Error>
where
    T: IntoHttpRequest,
    R: DeserializeOwned,
{
    let url = request.get_url();
    let data = request.get_data();
    let headers = request.get_headers().unwrap_or_else(|| HeaderMap::new());

    let response_result = match request.get_endpoint_type() {
        EndpointType::POST => post_request(&url, &data, &headers).await,
        EndpointType::GET => get_request(&url, &data, &headers).await,
    };

    match response_result {
        Ok(res) => {
            // Try to deserialize the response body into type R
            match res.json::<R>().await {
                Ok(typed_response) => Ok(Some(typed_response)),
                Err(_) => Ok(None), // Deserialization failed, return None
            }
        }
        Err(e) => Err(e),
    }
}
 