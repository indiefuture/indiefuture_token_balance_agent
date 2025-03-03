
use std::collections::HashMap;
use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::Responder;
 
 

use ethers::types::Address;
use ethers::types::U256;
use indiefuture_template_agent::ai::ai_engine::generate_ai_response;
use indiefuture_template_agent::ai::ai_engine::ChatAiOutputBody;
use indiefuture_template_agent::app_state::AppState;
 
use indiefuture_template_agent::types::domains::eth_address::DomainEthAddress;
use indiefuture_template_agent::types::evm_types::RawTxInput;
use indiefuture_template_agent::util::backend_server_error::BackendServerError;
use indiefuture_template_agent::util::openai::CompletionOutputBody;

use indiefuture_template_agent::ai::ai_engine::generate_ai_output_pass;


use serde::{Deserialize, Serialize};

use actix_web::web::{self, Data, Json, ServiceConfig}; 
 use serde_json::json;

use serde_json::Value;

 use log::info;

use indiefuture_template_agent::ai::action_set::ActionSetType; 
use tokio_postgres::types::ToSql;
 

use super::web_controller::AuthResponse;
use super::web_controller::WebController;

 




/*

 
curl -X POST http://localhost:8080/api/webhook \
     -H "Content-Type: application/json" \
     -d '{ "session_token": " ", "wallet_public_address":" "  }'



curl -X POST http://localhost:8080/api/webhook \
     -H "Content-Type: application/json" \
     -d '{ "session_token": " ", "wallet_public_address":" "  }'



curl -X POST http://localhost:9000/api/chat_message \
     -H "Content-Type: application/json" \
     -d '{  "body":"give me info about teller loan with id 1743 "  }'


 
curl -X POST https://sea-lion-app-idb6k.ondigitalocean.app/api/chat_message \
     -H "Content-Type: application/json" \
     -d '{  "body":"give me info about teller loan with id 1743"  }'

 

*/

pub struct WebhookController {}

impl WebhookController {}

impl WebController for WebhookController {
    fn config(cfg: &mut ServiceConfig) {
        cfg.service(
            web::scope("/api")
                // Add your routes here, e.g.,
                .route(
                    "/webhook",
                    web::post().to(handle_webhook),
                ) 


                 .route(
                    "/chat_message",
                    web::post().to(handle_chat_message),  // fix this !! 
                ),
        );
    }
}
 


#[derive(Serialize, Deserialize, Debug )]
pub struct HandleWebhookInput {

    webhook_secret_token: String , 

    function_name:  String, 
    input_params: serde_json::Value , 

}  

 
// Route Handler
async fn handle_webhook(
    input: Json<HandleWebhookInput>,
    app_state: Data<AppState>,
) -> HttpResponse {
    

    info!("handling webhook 1");

    match input.function_name.as_str() {




        "chat_message" => { 

              let Ok(chat_message_inputs) = serde_json::from_value::<ChatMessageInputs>(input.input_params.clone() )  else {
                // If parsing fails, return an error response
                   return   HttpResponse::BadRequest().json("Invalid message format") 
                } ;  

            //   let input_json =  serde_json::to_value( chat_message_inputs ) .unwrap()  ;
       

            handle_chat_message(  Json( chat_message_inputs ) , app_state).await

         },





        

        _ =>   HttpResponse::BadRequest().body("Unsupported function")  
    }
}


#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct ChatMessageInputs {

    //from_address: String,
    //to_address: String, 

    pub api_key: Option<String>, 

    pub body: String, 

}
 
#[derive(Clone,Serialize)]
struct SendChatMessageOutput {

        

         body: String , 

         tx_array: Option<Vec<RawTxInput>> ,

         structured_data: Option< serde_json::Value  >

}


pub async fn   handle_chat_message(
    chat_message_inputs:   Json< ChatMessageInputs > , 
 
 //   state: &web::Data<AppState>,

     app_state: Data<AppState>,


    ) -> HttpResponse {
    // Attempt to parse the JSON value into ChatMessageInputs struct
    
    /*  let Ok(chat_message_inputs) = serde_json::from_value::<ChatMessageInputs>(params.clone() )  else {
        // If parsing fails, return an error response
       return   HttpResponse::BadRequest().json("Invalid message format") 
    } ;  */








    // Logic to handle the chat message
    // For example, you might log the message or store it in a database
    println!("Received chat message from {:?} ", chat_message_inputs.api_key );


    let ai_action_set = ActionSetType::OuterActions; 
    let ai_engine = & app_state.ai_engine_data;
     let input_pass_system_prompt = "You are a helpful agent assistant helping users access api data \
     for teller protocol, a lending smart contract and protocol on the ethereum blockchain.".into() ;


    //make this a result ? 
    let ai_body_response :Result< ChatAiOutputBody , BackendServerError >  = generate_ai_response(
          Arc::clone( ai_engine ),
          //  chat_message_inputs.from_address .clone()  ,
            chat_message_inputs.body .clone() ,

            Some( input_pass_system_prompt ), 

            ai_action_set
        ).await;


    println!("generated res {:?}", ai_body_response );
    let ai_body_response = match ai_body_response {

        Ok(res) => res, 

        Err( e ) => return HttpResponse::InternalServerError().json( format!("{:?}", e  ) ) 

    } ; 
    




    let ai_body_content = generate_ai_output_pass(

         Arc::clone( ai_engine ),  

         ai_body_response.clone() ,

    //     Some( output_pass_system_prompt ), 

       )  .await ; 

 

    let structured_data = ai_body_response.structured_data;  
    let tx_array = ai_body_response.tx_array ; 

   

    
    let output_chat_message = SendChatMessageOutput {



        body: ai_body_content ,

        tx_array ,

        structured_data , 


    };


     let Ok( chat_message_json   )  = serde_json::to_value( output_chat_message ) else {
            
        return  HttpResponse::InternalServerError().json(" No chat message ") 
        //return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Could not parse params")))
    };

 /*   let teller_api_url = "https://webapi.teller.org/api/chat_message/send" ;


    let response_result = post_request( 
       protocol_url,
        chat_message_json 
     ) .await  ;

    // println!(" meep ");
    info!( "{:?}", response_result );*/



    // You can access your state here, e.g., state.db.send(chat_message).await;

    HttpResponse::Ok().json( chat_message_json ) 
  
}


/* 


     // ------------ 


     // verify that the provider  id  matches up w the   wallet address  .. 


    let scopes = None ; // for now 
    
        
    let new_api_key = ProviderApiKey::new( DomainEthAddress( wallet_address ), input.name.clone(), scopes) ;
 
    let inserted = ProviderApiKeysModel::insert_one(new_api_key.clone() , &app_state.database) .await ;
  
   





    match inserted {
        Ok(  new_id  ) => {



            let api_key_created_output = ProviderApiCreatedOutput{
               // id: new_id ,
                api_key : new_api_key.apikey, 
           }  ;


        HttpResponse::Ok().json(AuthResponse {
            success: true,
            data: Some( api_key_created_output   ),
            error: None,
        })



        },
        Err(_) => HttpResponse::InternalServerError().json(AuthResponse ::<String >{
            success: false,
            data: None,
            error: Some("Database error".to_string()),
        }),
    }


      

} */

#[derive(Deserialize,Serialize)]
struct WebhookOutput {

   // id: i32 ,
   success: bool 

}
 

 