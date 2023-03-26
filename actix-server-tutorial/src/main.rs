mod api; //create module -> compiler checks api.rs first, then check mod.rs 
mod repo;
mod model;

use api::task::{
    get_task
}

use actix_web::{HttpServer, App, web, middleware::Logger};
use repo::ddb::DDBRepository;

#[actix_web::main] //wrap main with macro of actix_web
async fn main() -> std::io::Result<()> { //add async keyword
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    //Don't need to understand ddb part for this tutorial
    let config = aws_config::load_from_env().await;
    //A closure passed into http server is set up for everything needs in web application
    HttpServer::new(move || { //use 'move' to move ownership of data inside closure
        let ddb_repo: DDBRepository = DDBRepository::init(
            String::from("task"),
            config.clone(),
        );
        let ddb_data = web::Data::new(
            ddb_repo
        );
        let logger = Logger::default();
        App::new()
            .wrap(logger) //use default logger for logging
            .app_data(ddb_data) // allows us to pass application state included shared_state
            .service(get_task) //service specifies handler functions.
    })
    .bind(("127.0.0.1", 8080))? //needs to append question mark
    .run()
    .await
}
