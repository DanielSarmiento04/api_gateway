use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use reqwest;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

const USERS_MICROSERVER_URL: &str = "http://127.0.0.1:8000"; // Replace with actual URL
const CARS_MICROSERVER_URL: &str = "http://127.0.0.1:8001";  // Replace with actual URL

// Users endpoint handler
async fn users_handler(req: HttpRequest) -> HttpResponse {

    println!("Request: {:?}", req.uri());
    let request_url = format!("{}{}", USERS_MICROSERVER_URL, req.uri());
    match reqwest::Client::new().get(&request_url).send().await {
        Ok(response) => {
            HttpResponse::build(response.status())
                .body(response.text().await.unwrap_or_default())
        }
        Err(_) => HttpResponse::InternalServerError().body("Error forwarding request to users micro server"),
    }
}

// Cars endpoint handler
async fn cars_handler(req: HttpRequest) -> HttpResponse {

    println!("Request: {:?}", req.uri());
    let request_url = format!("{}{}", CARS_MICROSERVER_URL, req.uri());
    match reqwest::Client::new().get(&request_url).send().await {
        Ok(response) => {
            HttpResponse::build(response.status())
                .body(response.text().await.unwrap_or_default())
        }
        Err(_) => HttpResponse::InternalServerError().body("Error forwarding request to cars micro server"),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(web::resource("/users/{tail:.*}").to(users_handler))
            .service(web::resource("/cars/{tail:.*}").to(cars_handler))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}