use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use std::env;

//use oauth2::http::{HeaderMap, HeaderValue};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use serde_json::json;

use crate::types::openai_completion::{
    OpenAiCallableFunction, OpenAiCompletion, OpenAiErrorResponse,
};

use super::backend_server_error::BackendServerError;

pub type CompletionOutputBody = OpenAiCompletion;

#[derive(Serialize)]
pub struct CompletionMessage {
    role: String,
    content: String,
}

pub struct OpenAiClient {
    api_url: String,

    openai_api_key: String,
    system_message: Option<String> ,

    model: String,

 
}


const BASE_SYSTEM_PROMPT:&str = "You are a helpful assistant." ;

#[derive(Serialize)]
pub struct FunctionTool {
    
    r#type: String,
    function: OpenAiCallableFunction 
}

impl OpenAiClient {
    pub fn new( system_message: Option<String> ) -> Self {
        Self {
            
            api_url: "https://api.openai.com/v1/chat/completions".into(),
            openai_api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found"),
            system_message, // system_message.unwrap_or("You are a helpful assistant.".into()),
          //  model: "gpt-3.5-turbo".into(),
             model: "gpt-4o".into(),
            //functions,
        }
    }

    pub fn set_system_prompt(&mut self,  system_msg: Option<String>) {

        self.system_message = system_msg; 
    } 

    pub async fn fetch_chat_completion(
        &self,
        prompt_base: String,
        system_message_override: Option<String>, 
         
        functions: Vec<OpenAiCallableFunction> , 
    ) -> Result<CompletionOutputBody, BackendServerError> {
        let client = reqwest::Client::new();

        let api_url = self.api_url.clone();

        //load me from env
        let api_key = self.openai_api_key.clone();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );

        let messages = vec![
            CompletionMessage {
                role: "system".to_string(),
                content: system_message_override.or( self.system_message.clone()) .unwrap_or(BASE_SYSTEM_PROMPT.to_string()),
            },
            CompletionMessage {
                role: "user".to_string(),
                content: prompt_base,
            },
        ];

        
        let enable_function_calling = !functions.is_empty();
 

        let mut function_tools = Vec::new() ;

        for function in functions {
            function_tools.push(
                FunctionTool{

                   r#type: "function".into(),
                   function: function.clone()
                }

            );
        }


        let body = match enable_function_calling {


            true => json!({
                    "model": self.model,
                    "messages": messages,
                    "tools": function_tools,
                    "tool_choice": "required"  // or 'automatic'
                
                }),

            false => json!({
                "model": self.model,
                "messages": messages,
               
            
            }),

        };

 
        println!("Headers: {:?}", headers);
        println!("Request body: {:?}", body);

        let res = client
            .post(api_url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(| e| BackendServerError::ReqwestError( e )  )?;

       // println!(" res   {:#?}", res);

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|_e| BackendServerError::InputParsingError)?;

        //println!(" res body {:#?}", body);

        let completion_body = serde_json::from_value::<CompletionOutputBody>(body.clone());
        //  .map_err(|_e| BackendServerError::InputParsingError)?;

        if let Ok(completion_message) = completion_body {
            return Ok(completion_message);
        } else {
            let response_error = serde_json::from_value::<OpenAiErrorResponse>(body.clone());

            if let Ok(res_err) = response_error {
                println!("Err: {:?}", res_err);
            }

            return Err(BackendServerError::UnknownError.into());
        }

        //Ok(  completion_body  )
    }
}
 