
 
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


 