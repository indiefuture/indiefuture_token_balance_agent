
 
use rust_decimal::Decimal;
use crate::types::domains::eth_address::DomainEthAddress;
use crate::types::domains::uint256::DomainUint256;
 
 
 
use serde::Serialize;
use serde::Deserialize;

use super::openai_completion::OpenAiCallableFunction;

#[derive(Serialize, Deserialize,Debug,Clone )]
pub struct AvailableTools {
    pub tools:  Vec<OpenAiCallableFunction> ,

   
}

impl AvailableTools {

    pub fn new(input: Vec<OpenAiCallableFunction> ) -> Self {
        Self {
            tools:input 

        }

    }

}



use super::openai_completion::GptFunctionCall;


#[derive(Serialize,Deserialize,Default,Debug,Clone)]
pub struct IndiefutureAgentMessagePrimitives {

    //pub message_body: Option<String>,
    //pub structured_data: Option<serde_json::Value>, 
    pub input_prompt: Option<String>,
     pub message: Option<String>,
    pub structured_data : Option<serde_json::Value> , 
    pub tool_call: Option<GptFunctionCall>
}
