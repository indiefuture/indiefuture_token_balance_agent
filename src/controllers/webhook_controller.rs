
use std::collections::HashMap;
use std::fmt::format;
use std::sync::Arc;

use actix_web::FromRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
 
 
 
 
use indiefuture_template_agent::app_state::AppState;
  
 use indiefuture_template_agent::types::indiefuture_types::IndiefutureAgentMessagePrimitives;
 

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

 
use indiefuture_template_agent::util::json_extract::ExtractableFromJson; 




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

 
        );
    }
}
 


//#[derive(Serialize, Deserialize, Debug )]
//pub struct HandleWebhookInput {

   // webhook_secret_token: Option<String> , 

    //function_name:  String, 
   // input_params: serde_json::Value , 

//}  

 
// Route Handler
async fn handle_webhook(
    input: Json< serde_json::Value >,
    app_state: Data<AppState>,
) -> impl Responder {
    

    let agent_message_primitives = serde_json::from_value::<IndiefutureAgentMessagePrimitives>( input.0.clone() ).unwrap();

    println!(" token balance agent handling webhook 1 {:?} ", agent_message_primitives);
    


    // for weatherbot! 
    let workspace_uuid = "0xd857721b5385e27d";


    let Some(client_api_key) =  input.get("client_api_key").map(|x|  <String as ExtractableFromJson >::extract( x  ) )  .flatten().clone() else {


                let output_json =    IndiefutureAgentMessagePrimitives {

                  
                    message: Some( format!(  "missing client_api_key for workspace {}", workspace_uuid.clone() )  ), 
                    structured_data: Some( 
                        json!(

                            { "error":  "MissingClientApiKey",
                                "workspace_uuid" : workspace_uuid .clone()  }
                            
                          )
                     ), 

                    ..Default::default()

                }  ;

                return   HttpResponse::Ok().json( json!( output_json )  ) ;

    } ;






    // ask defi relay if this apikey has enough credits !!! 


    let mut has_sufficient_credits =  false; 




   // ------------------------






    if !has_sufficient_credits {

                let output_json =    IndiefutureAgentMessagePrimitives {
                

                    message: Some( format!(  "InsufficientCreditsForWorkspace {}", workspace_uuid.clone() )  ), 
                    structured_data: Some( 
                        json!(

                            { "error":  "InsufficientCreditsForWorkspace",
                             "workspace_uuid" : workspace_uuid .clone() , }
                            
                          )
                     ), 
                     
                    ..Default::default()

                }  ;

                return   HttpResponse::Ok().json( json!( output_json )  ) ;

    }





 
        //--------------


            // DEDUCT CREDITS HERE !!! 
        // ------------
 
            let output_json =    IndiefutureAgentMessagePrimitives {
         
                    structured_data: Some( 
                        json!(
                            { "data" : " Heres the special data!!" }
                          )
                     ), 
                     
                    ..Default::default()

                }  ;

                return   HttpResponse::Ok().json( json!( output_json )  ) ;

}

 

#[derive(Deserialize,Serialize)]
struct WebhookOutput {

   // id: i32 ,
   success: bool 

}
 

 