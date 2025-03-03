
use actix_web::web::Json;
use actix_web::body::to_bytes;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use actix_web::body::MessageBody;
use actix_web::web::Data;
use actix_web::HttpResponse;
use indiefuture_template_agent::{ai::ai_engine::AiEngineData, app_state::AppState, util::openai::OpenAiClient};
use dotenvy::dotenv;
 
use tokio::fs;
use std::error::Error;
use dialoguer::{Input, Confirm};
use crossterm::{
    ExecutableCommand,
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
};
 

mod controllers;

 
 
 
use crate::controllers::webhook_controller::handle_chat_message;
use crate::controllers::webhook_controller::ChatMessageInputs;

 
 
 /*


There will be various 'domains' of action-maps.  

For example: The meta-action map is a map of all of the domains (how to fetch each..)


The github action map is a map of all of the actions that can be performed w github api 
The digitalocean action map is a map of all the actions that can be performed a digitalocean 


 */


 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

     dotenv().ok();


   // let client = Client::new();
    // std::io::stdout().execute(Clear(ClearType::All))?;
    std::io:: stdout().execute(Clear(ClearType::CurrentLine))?;
    enable_raw_mode()?;
    // Clear the line before writing new text
    // std::io::stdout().execute(Clear(ClearType::CurrentLine))?;




      let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        running.store(false, Ordering::SeqCst);
        cleanup();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");





    let system_message = None ;

    let openai_client = OpenAiClient::new(

        system_message,

        

    );


       let system_prompt = None ; 


       let ai_engine_data = Arc::new( AiEngineData::new( system_prompt )  );

    let app_state =   AppState {
    	ai_engine_data  
     }   ;


     let app_state_data =  Data::new ( app_state ) ;  // basically an ARC 




    loop {
        let input_prompt: String = Input::new()
            .with_prompt("What would you like to do ? ")
            .interact_text()?;
        
        println!(" got: {} .  ", input_prompt);


 	
 	let chat_inputs = ChatMessageInputs {
 		api_key: None, 

 		body: input_prompt.clone() 

 	} ;
 

        let ai_response =  handle_chat_message(


        	Json( chat_inputs ) , 
          
             app_state_data.clone()
        ).await;


        let body_bytes = to_bytes( ai_response .into_body()).await?;
 
      if let Ok(body_str) = String::from_utf8( body_bytes.to_vec()  ) {
		        println!("Response Body: {}", body_str);
		    } else {
		        println!("Response Body is not valid UTF-8");
		    }


       
 

        let continue_interaction = Confirm::new()
            .with_prompt("Do you want to continue?")
            .default(true)
            .interact()?;
        
        if !continue_interaction {
            break;
        }
    }

    disable_raw_mode()?;

    Ok(())
}


fn cleanup() {
    println!("Process is being terminated! Cleaning up...");
    // Add cleanup logic here

      disable_raw_mode() ;
}
