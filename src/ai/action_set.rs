

  
use serde_json::from_str;
use std::collections::HashMap;

use crate::types::openai_completion::OpenAiCallableFunction;

pub type AiActionSet  =   Vec<OpenAiCallableFunction> ;

pub type AiFunctionSetMap = HashMap< ActionSetType , AiActionSet > ;

#[derive( Hash ,Eq, PartialEq ,Clone, Debug )]
pub enum ActionSetType {


	OuterActions,

 

}



impl ActionSetType {


	pub fn get_action_set_file_contents(&self) -> &str {


		match self {

			Self::OuterActions => include_str!( "../.././actions/outer_actions.json") , 
			//Self::MoreActions => include_str!( "../.././actions/more_actions.json") ,
			  

		}


	}


	pub fn load_map() -> HashMap< Self, AiActionSet > {

		let sets_to_load = vec![   //use strum ? 
		  Self::OuterActions,
		 
		 ];



		let mut hash_map = HashMap::new (); 

		for set_type in sets_to_load {

		 	let file_content = set_type.get_action_set_file_contents() ;

		    let loaded_data: Vec<OpenAiCallableFunction> = from_str(&file_content) 
		    		.expect(  &format!(  "unable to parse action set file {:?}" , set_type ));

			hash_map.insert( set_type.clone(), loaded_data );


		}


		hash_map

	}

}