use actix_web::{web, App, HttpResponse, HttpServer};

//For simplicity and reusability, use configure method
//some of the resources'configure can be moved to a different module
//App and web::Scope provide configure method.

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse.Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse.Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config) //use config for app configuration
            .service(web::Scope("/api").configure(scoped_config)) //use scoped_config for web::Scope
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
