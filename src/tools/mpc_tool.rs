 
 
use crate::types::indiefuture_types::AvailableTools;
use serde_json::json;


 
use ethers::types::U256;
use serde::Serialize;
use serde::Deserialize;
use crate::ai::action_set::ActionSetType;
use crate::ai::action_set::AiFunctionSetMap;
use crate::ai::ai_engine::AiEngineData;
use async_trait::async_trait;
 




 
use std::str::FromStr;
use std::sync::Arc;
use crate::types::openai_completion::GptToolCall;

 
 
#[derive(Serialize,Deserialize, Debug , Clone, Default   )] 
pub struct MpcToolOutput {

   pub message: Option< String > ,

   pub structured_data: Option< serde_json::Value  >

}

 



#[async_trait]
pub trait MpcTool: Send + Sync {
    async fn handle_mpc_tool(&self, input_body: String, input_props: serde_json::Value,    ai_engine_data: Arc< AiEngineData >, ) -> MpcToolOutput ;
}








  






pub struct ListTools ;  // loan id
 

 #[async_trait]
impl MpcTool for ListTools {



 async fn handle_mpc_tool( &self, input_body: String,  input_props: serde_json::Value,  ai_engine_data: Arc< AiEngineData >, )
	 	 ->  MpcToolOutput { 


	 	 	 let tools = ai_engine_data.ai_function_sets.get( &ActionSetType::OuterActions) ;

	 	  
	 	 		
	 	 			let msg =  format!( " 

	 	 		     Here is some information about available actions :  

	 	 		  	{:?}

	 	 		 " , tools ); 



	 	 			let available_tools = tools.map(|x|  AvailableTools::new( x.to_vec() ) ) ;


	 	 		MpcToolOutput {

	 	 		 message: Some(msg.to_string()) , 
	 	 		 structured_data:     available_tools  .map( |x|  serde_json::to_value( &x ).ok()  ) .flatten() ,


	 	 		 ..Default::default()


	 	 		  }
	 	 }	

}




 







// --------------------------------------------------



pub enum MpcToolType {
	ListTools,
 

}


impl MpcToolType {


	pub fn parse_from_str( input: &str ) -> Option<Self> {
 
		match input {

			//goes to deeper pages 
			"ListTools" => Some(Self::ListTools),
			 
			_ => None 
 

		} 

	}

}

 


impl MpcToolType {
    pub fn get_tool(&self) -> Arc<dyn MpcTool> {
        match self {
            Self::ListTools  =>  Arc::new(ListTools),
            
         
            // Other cases should return their respective tool implementations



        //    _ => unimplemented!("Tool not implemented"),
        }
    }
}



pub async fn handle_tool_call(
    ai_body_tool_call: GptToolCall,

    input_body: String,

   // input_props: serde_json::Value,  

    ai_engine_data: Arc< AiEngineData >,
) -> MpcToolOutput {
    println!( " HANDLE TOOL CALL {:?} "  , ai_body_tool_call );

    let tool_type = MpcToolType::parse_from_str(&ai_body_tool_call.function.name).unwrap_or_else(|| unimplemented!("Invalid tool function specified"));
   

    let tool = tool_type.get_tool();

    let input_props  = serde_json::from_str(&ai_body_tool_call.function.arguments) .ok() .unwrap_or( json!("{}") ) ;
  

    tool.handle_mpc_tool(input_body, input_props,  Arc::clone( &ai_engine_data ) ).await
}
 