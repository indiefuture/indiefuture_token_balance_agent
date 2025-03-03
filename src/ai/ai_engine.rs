


/*



    let system_message = None ;

    let openai_client = OpenAiClient::new(

        system_message, 

    );




        let ai_functions = ai_function_maps.get( "meta_function_map" ) .unwrap()  ;

        let ai_respones = openai_client.fetch_chat_completion(
          input_prompt.clone(),
          ai_functions.to_vec() 
        ).await;


*/

use log::info;
use log::warn;
use serde_json::json;
use crate::tools::mpc_tool::handle_tool_call;
use crate::util::backend_server_error::BackendServerError;
use std::sync::Arc;
 
use crate::util::openai::CompletionOutputBody;
use std::collections::HashMap;
use crate::util::openai::OpenAiClient;
use std::collections::HashSet;


use crate::types::evm_types::RawTxInput; 
use crate::ai::action_set::ActionSetType;
use crate::ai::action_set::AiActionSet;
 



pub struct AiEngineData {


	pub ai_function_sets:  HashMap<  ActionSetType, AiActionSet > ,

	pub ai_client: OpenAiClient , 

} 

impl AiEngineData {


    pub fn new( system_prompt: Option<String>  ) -> Self {


        Self  {

            ai_client: OpenAiClient::new( system_prompt ), 
            ai_function_sets: ActionSetType::load_map(),

        }

    }
}




    pub async  fn generate_ai_response(
            ai_engine_data: Arc<AiEngineData>, 

          
            input_body: String, 


            system_prompt: Option<String>, 

            action_set: ActionSetType 

    ) -> Result< ChatAiOutputBody , BackendServerError   >{


       // let functions_set_arc = Arc::new( self.ai_function_sets.clone() ) ;

        let functions = ai_engine_data.ai_function_sets.get( &action_set ) .unwrap()  ;

        let result = ai_engine_data.ai_client.fetch_chat_completion(

            input_body.clone() ,

            system_prompt.clone(), 

            functions.to_vec(), 

        ).await;

 


       match  result  {

        Ok( ai_body_response ) => {



                    let Some(ai_body) = ai_body_response.choices.first() else {


                           warn!(" no ai response choices...  ");

                         return Err(BackendServerError::UnknownError) ; 

                    };




               

                  //  let input_props = json!("{}"); //empty props to being with for now -- typically these are added by the ai /functions 


                  
                    let ai_body_tool_call = ai_body.message.tool_calls.as_ref().and_then(|tc| tc.first()).cloned();


                   // let ai_body_tool_call =  ai_body.message.tool_calls .clone() .map( |tc| tc.clone().first() ).flatten() ;

                     let mpc_tool_output =  match ai_body_tool_call {

                        // ai_body_content = Some( format!( "!!tool call!! {:?}", ai_body_tool_call )  );

                            Some( tool_call) => Some( handle_tool_call (  
                                            tool_call , 
                                            input_body.clone() ,
                                         //   input_props, 
                                            Arc::clone( &ai_engine_data ) 

                                     ).await ) ,

                            None => None , 
     

                        };


                       // println!( "mpc tool output {:?}" , mpc_tool_output );


                    let Some( message_body ) = mpc_tool_output.as_ref().map( |o|  o.message.clone() ).flatten()  else {

                            return Err(BackendServerError::UnknownError) ; 

                    };


                     let  structured_data  = mpc_tool_output.as_ref().map( |o|  o.structured_data.clone() ).flatten()  ;




                    // println!( "mpc tool output 2 {:?}" , mpc_tool_output );

                    let chat_ai_output_body = ChatAiOutputBody {

                        message:message_body , 

                        tx_array: None  ,

                        structured_data: structured_data ,
 
                    };





                    Ok( chat_ai_output_body ) 



          }
          Err (e ) => {
             warn!(" Could not access ai api 1 {:?} " , e);
             Err( e )  
          }
        } 

           
           
       
 


       // result.ok() 

    }

 
#[derive(Debug,Clone)]
pub struct ChatAiOutputBody {

    pub message: String, 

    pub tx_array: Option< Vec< RawTxInput > >,

    pub structured_data: Option< serde_json::Value  >, 



}


// ----------




 pub async  fn  generate_ai_output_pass (

      ai_engine_data: Arc<AiEngineData>, 

          
        input: ChatAiOutputBody, 


      //  system_prompt: Option<String>,  



 )-> String  {  // generate a new body -- one that is formatted and built form struct 






         let output_pass_system_prompt = "You are a helpful agent assistant helping users access api data \
     .  Using this users \
     origin input query and the structured data provided b  as context, help answer their question \
     using this structured data, omitting unnecessary data.".into() ;




       let chat_completion_input = format!(

        "  {}    

            ---------------    

            {:?}
        " , input.message.clone(), input.structured_data 
        );

        let result = ai_engine_data.ai_client.fetch_chat_completion(

            chat_completion_input ,

            Some(output_pass_system_prompt), 

            Vec::new() 

        ).await;

 
        match result {

            Ok( completion ) => {
                if let Some( first_choice ) = completion.choices.first() {

                    if let Some(content)  =   first_choice.message.content.clone()  {
                            return content; 
                    }     

                } 


            }

            Err( e ) =>   {
                warn!("{:?}", e );
            } 



        };



        return input.message

    
 }