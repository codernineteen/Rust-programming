use ::sync::Mutex;
use actix_web::{web, App, HttpServer, Responder};

//State is shared with all routes and resources within same 'scope'
//state can be accessed with the web::Data<T>

//simple state example
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index_single(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; //<- get app name
    format!("hello {app_name}") //return response
}

//HttpServer accepts "application factory" rather than an application instance.
//In this sense, it constructs each application instance for each thread
//To avoid data creation duplicates, shareable object should be used(which implements Send + Sync trait)
//Internally, "web::Data" uses 'Arc'(Atomic reference counter)

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/")]
async fn index_mutex(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); //<- get app name
    *counter += 1;

    format!("Request number: {counter}") //response incremented counter
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //globally shared state
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(|| {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
