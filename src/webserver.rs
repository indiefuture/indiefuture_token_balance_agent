 
 
use indiefuture_template_agent::ai::ai_engine::AiEngineData;
use indiefuture_template_agent::app_state::AppState;
use tokio::io;
use std::sync::Arc;
use dotenvy::dotenv;

 use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

 
mod controllers;

 
use controllers::webhook_controller::WebhookController; 
use controllers::web_controller::WebController; 
 
/*

Serves the HTTP api 

See 'controllers' for routes and functions 

*/


#[tokio::main]
async fn main()  -> io::Result<()> {
   

    dotenv().ok();

    // Initialize the logger
   // std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info"); // Adjust as per your needs
    env_logger::init();

 //   println!("connecting to db.");

  
  //  let db_conn_url = std::env::var("DB_CONN_URL").expect(" DB_CONN_URL must be set in env ");

  //  let database = Arc::new(  Database::new(db_conn_url, None) .unwrap()  );

    let system_prompt = Some ( "you are a helpful agent assistant helping users access api data  ".into()   ) ; 

     let ai_engine_data = Arc::new( AiEngineData::new( system_prompt )  );

   // println!("connected to db.");


    println!("starting webserver");

    //setup and launch the http server
    HttpServer::new(move || {

 
        let cors = Cors::default()
            //  .allowed_origin("http://localhost:3000")
            // .allowed_origin("http://localhost:8080")
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Authorization", "Accept", "Content-Type"])
            .supports_credentials()
            .max_age(3600);

         let app_state = AppState {

            ai_engine_data: Arc::clone( &ai_engine_data )
           //  database: Arc::clone(&database),
        };  

        App::new()
            .app_data(Data::new(app_state)) // Clone your db connection or use Arc

       

            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default()) // Enable logger middleware
            

              .configure(WebhookController::config)
             
             
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await  


}
