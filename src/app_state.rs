
use crate::ai::ai_engine::AiEngineData;
use std::sync::Arc;

use tokio::sync::Mutex;

use degen_sql::db::postgres::postgres_db::Database; 

 
pub struct AppState {
   pub ai_engine_data:  Arc<AiEngineData>
     // pub database: Arc< Database >,
}