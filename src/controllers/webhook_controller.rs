
use std::collections::HashMap;
use std::fmt::format;
use std::sync::Arc;

use actix_web::FromRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
 
 
 
 
use indiefuture_template_agent::app_state::AppState;
  
 use indiefuture_template_agent::types::domains::decimal::DomainDecimal;
use indiefuture_template_agent::types::domains::eth_address::DomainEthAddress;
use indiefuture_template_agent::types::indiefuture_types::IndiefutureAgentMessagePrimitives;
 

use indiefuture_template_agent::ai::ai_engine::generate_ai_output_pass;


use indiefuture_template_agent::types::selected_record::SelectedRecord;
use indiefuture_template_agent::util::header_map_preset::HeaderMapPreset;
use indiefuture_template_agent::util::http_request::perform_req;
use indiefuture_template_agent::util::http_request::perform_req_typed;
use indiefuture_template_agent::util::http_request::EndpointType;
use indiefuture_template_agent::util::http_request::IntoHttpRequest;
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

 
curl -X POST http://localhost:9000/api/webhook \
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


    let Some(client_api_key) =  input.get("indiefuture_client_api_key").map(|x|  <String as ExtractableFromJson >::extract( x  ) )  .flatten().clone() else {


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


     let get_workspace_credits_amount = get_refill_workspace_credits_amount (
        &workspace_uuid.to_string(),
        &client_api_key
      ).await .unwrap_or (0) ;

     println!("got workspace credits amt {:?}", get_workspace_credits_amount );
    

     // min credits is 25 cents 
    if get_workspace_credits_amount >= 25  {

        has_sufficient_credits = true ;

    }
   // ------------------------






    if !has_sufficient_credits {

                let output_json =    IndiefutureAgentMessagePrimitives {
                

                    message: Some( format!(  "InsufficientCreditsForWorkspace {}", workspace_uuid.clone() )  ), 
                    structured_data: Some( 
                        json!(

                            {
                                "error":  "InsufficientCreditsForWorkspace",
                                 "workspace_uuid" : workspace_uuid .clone() 
                              }
                            
                          )
                     ), 
                     
                    ..Default::default()

                }  ;

                return   HttpResponse::Ok().json( json!( output_json )  ) ;

    }





 
        //--------------


           // DEDUCT CREDITS HERE !!! 
        // ------------



     let Some( get_workspace_client_key_data ) = get_refill_client_data (
       
         client_api_key.clone() 

      ).await else  {

              return   HttpResponse::InternalServerError().json( json!( {

                   "error":  "Could not get client data",

               } ) ); 

        };

        let client_public_address =  &get_workspace_client_key_data.entry.client_address; 




        let Some( workspace_api_key)  =  std::env::var("DEFIRELAY_WORKSPACE_API_KEY").ok() else  {

              return   HttpResponse::InternalServerError().json( json!( {

                   "error":  "Misconfiguration",

               } ) ); 

        };


        let credit_deduction_amount_cents = 2; 

        let credits_deducted_result = deduct_credits_for_client  (
             workspace_uuid.to_string(),
            workspace_api_key, 
             client_public_address.to_string_full() ,
            credit_deduction_amount_cents
         ).await   ;

        println!( "credits_deducted_result {:?}", credits_deducted_result );

          // ------------- 

             if credits_deducted_result.is_none()  {

              return   HttpResponse::InternalServerError().json( json!( {

                   "error":  "Unable to deduct credits ",

               } ) ); 

        };






        //  -------------------------------- 

        // this is where you would do the special API stuff now that the agent has been paid in credits !! 


 
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

 
 
 

 


// ----------------------






#[derive(Serialize, Deserialize, Debug )]
pub struct GetApiCreditsOutput {
    credits: DomainDecimal,
    credits_cents: i64,
}

#[derive(Serialize)]
 pub struct GetCreditsAmountEndpoint {
    workspace_uuid: String, 
    client_api_key: String ,
 } 


 impl IntoHttpRequest for GetCreditsAmountEndpoint {
    fn get_url(&self) -> String {
        "https://api.defirelay.com/api/client_key/get_api_credits" .into() 
    }

    fn get_data(&self) -> serde_json::Value {
        
        serde_json::to_value( self  ) .unwrap_or_default()
    }

    fn get_headers(&self) -> Option<reqwest::header::HeaderMap> {
        HeaderMapPreset::ApplicationJson.build() .into() 
    }

    fn get_endpoint_type(&self) ->  EndpointType {
         EndpointType::GET
    }
}


 async fn get_refill_workspace_credits_amount( 
    workspace_uuid: &String,
     client_api_key: &String ) 

 -> Option< i64 > {


    let endpoint = GetCreditsAmountEndpoint { 
        workspace_uuid: workspace_uuid.clone(),
         client_api_key:client_api_key.clone()
          };

     let response: Result< Option< AuthResponse< GetApiCreditsOutput> >  , reqwest::Error  > 
        = perform_req_typed( &endpoint ).await ;



    match response {

        Ok(Some( res )) => {  

                let api_credits_output = res.data ;


                // println!( "got {:?}", res );

                return api_credits_output.map(|x| x.credits_cents ) 



        },

        _ => None 

    }




 }



// ----------------------












#[derive(Serialize)]
 pub struct GetClientKeyDataEndpoint {
   
    client_key : String ,
   
 } 


 impl IntoHttpRequest for GetClientKeyDataEndpoint {
    fn get_url(&self) -> String {
        "https://api.defirelay.com/api/client_key/find_by_client_key" .into() 
    }

    fn get_data(&self) -> serde_json::Value {
        
        serde_json::to_value( self  ) .unwrap_or_default()
    }

    fn get_headers(&self) -> Option<reqwest::header::HeaderMap> {
        HeaderMapPreset::ApplicationJson.build() .into() 
    }

    fn get_endpoint_type(&self) ->  EndpointType {
         EndpointType::POST
    }
}




#[derive(Serialize, Deserialize, Debug )]
pub struct RefillClientKeyData {
   
    pub name: Option<String>,
    pub client_key: String,
    pub client_address: DomainEthAddress,
    pub workspace_uuid: String ,
    pub credits: DomainDecimal,
}



 async fn get_refill_client_data (
        
        client_key: String ,
        

    ) ->    Option< SelectedRecord< RefillClientKeyData >  >    {




    let endpoint = GetClientKeyDataEndpoint { 
         client_key  , 
       
      };

        let response_one   = perform_req ( &endpoint ).await  ;

         println!("response_one {:?}", response_one.unwrap().text().await );


     let response: Result< Option< AuthResponse< SelectedRecord< RefillClientKeyData >  > >  , reqwest::Error > 
        = perform_req_typed( &endpoint ).await  ;



     match response {

         Ok( Some( res  ) )  => {   

             println!("res {:?}", res );
                return res.data 



        },

        Err( e ) => {


            println!("error {:?}", e );


            None 
        }

        _ => None 

    }


 




 }




















// ----------------------

#[derive(Serialize)]
 pub struct DeductCreditsEndpoint {
    session_token: String , //apikey  
    workspace_uuid: String, 
    client_address: String ,
    credits_delta_cents: i64 

 } 


 impl IntoHttpRequest for DeductCreditsEndpoint {
    fn get_url(&self) -> String {
        "https://api.defirelay.com/api/client_key/deduct_api_credits" .into() 
    }

    fn get_data(&self) -> serde_json::Value {
        
        serde_json::to_value( self  ) .unwrap_or_default()
    }

    fn get_headers(&self) -> Option<reqwest::header::HeaderMap> {
        HeaderMapPreset::ApplicationJson.build() .into() 
    }

    fn get_endpoint_type(&self) ->  EndpointType {
         EndpointType::POST
    }
}




#[derive(Serialize, Deserialize, Debug )]
pub struct UpdateApiCreditsOutput {
    new_credits: DomainDecimal,
    new_credits_cents: i64,
}



 async fn deduct_credits_for_client (
        
        workspace_uuid: String, 
        session_token: String , 
        client_address: String ,
        credits_delta_cents: i64  
    ) ->  Option< AuthResponse< UpdateApiCreditsOutput> >   {




    let endpoint = DeductCreditsEndpoint { 
         session_token  , 
        workspace_uuid , 
        client_address  ,
        credits_delta_cents
      };

     let response: Option< AuthResponse< UpdateApiCreditsOutput> >  
        = perform_req_typed( &endpoint ).await.ok().flatten()  ;

        response
 




 }