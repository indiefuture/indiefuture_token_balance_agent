use reqwest::Response;
use reqwest::header::HeaderMap;
use reqwest::Error;
use reqwest::IntoUrl;
use serde_json::json;

pub async fn post_request<U: IntoUrl>(url: U, data: serde_json::Value) -> Result<Response, Error> {
    // Create a client instance
    let client = reqwest::Client::new();

   

    // Send a POST request
    let res = client.post(url).json(&data).send().await?;

    

    // Optionally, handle the response e.g., check status, parse body, etc.
    if res.status().is_success() {
        println!("Successfully sent the POST request");
    } else {
        println!("Failed to send POST request: {}", res.status());
    }

    Ok ( res )
}


pub async fn get_request<U: IntoUrl + std::fmt::Debug>(
    url: U, 
    params: Option<serde_json::Value>
) -> Result<Response, Error> {
   // Create a client instance
    let client = reqwest::Client::new();
    
    // Log the base URL
    println!("Base URL: {:?}", url);
    
    // Build the request with optional query parameters
    let mut request_builder = client.get(url);
    
    // Add query parameters if provided
    if let Some(parameters) = params {
        println!("Query parameters: {}", parameters);
        request_builder = request_builder.query(&parameters);
    }
    
    // Try to get the full URL with parameters for debugging
    // This is a bit of a hack, but helpful for debugging
    let debug_req = request_builder.try_clone().unwrap().build()?;
    println!("Full request URL: {:?}", debug_req.url());
    
    // Send the GET request
    println!("Sending request...");
    let res = request_builder.send().await?;
    
    // Log detailed response information
    println!("Response status: {}", res.status());
    println!("Response headers: {:#?}", res.headers());
    
    // Optionally, handle the response e.g., check status, parse body, etc.
    if res.status().is_success() {
        println!("Successfully sent the GET request");
    } else {
        // Log more details for failed requests
        println!("Failed to send GET request: {}", res.status());
        
        // Copy response for debugging
      /*  let status = res.status();
        let headers = res.headers().clone();
        
        // Try to get error body (this consumes the response)
        match res.text().await {
            Ok(error_body) => println!("Error response body: {}", error_body),
            Err(e) => println!("Could not read error response body: {}", e),
        }

        panic!(" o no");*/
        
        // Return a new error response since we consumed the original
       // return panic!("uhhh") 
        
    }
    
    Ok(res)
}
