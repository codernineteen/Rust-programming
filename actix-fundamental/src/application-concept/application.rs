use actix_web::{web, App, HttpServer, Responder};

//actix-web built on App instance
//App instance : register routes for middleware,pre-processing of req and res
//+ stores application state shared by across all handlers

async fn index() -> impl Responder {
    "Hello World" // no return keyword when it is alone in scope.
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/app") //scope is namespace for all routes
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1" , 8080))?
    .run()
    .await
}
