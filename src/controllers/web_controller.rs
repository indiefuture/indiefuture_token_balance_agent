 
 
use actix_web::web::ServiceConfig;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use ethers::types::Address;
 


pub trait WebController {
    fn config(cfg: &mut ServiceConfig);
}





#[derive(   Debug   )]
pub struct WebControllerSuccessResponse  ;
impl Serialize for WebControllerSuccessResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
       
       


        let mut state = serializer.serialize_struct("WebControllerSuccessResponse", 1)?;
        state.serialize_field("success", &true)?;
        state.end()


    }
}




#[derive(Serialize, Deserialize, Debug)]
pub struct WebControllerErrorResponse {

    error_message: String ,

}


impl WebControllerErrorResponse{

    pub fn new (input: impl ToString ) -> Self {

        Self {

            error_message: input.to_string() 
        }

    }
}





#[derive(Deserialize, Serialize)]
pub struct AuthSessionOutput {
    pub public_address: String ,
    pub session_token: String,
    pub expires_at: i64
}
 


 #[derive(Deserialize, Serialize, Debug ,Clone )]
pub struct AuthResponse <T>  {
   pub success: bool,
   pub data: Option< T >,
   pub error: Option<String>,
}