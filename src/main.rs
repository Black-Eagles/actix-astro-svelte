use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo) // From getting started
            .route("/hey", web::get().to(manual_hello)) // From getting started
            .service(Files::new("/", "www/dist") // File server needed for Svelte GUI, has to be created as last
                .prefer_utf8(true)
                .index_file("index.html")
            )
    })
    .bind(("127.0.0.1", 8080))? // Sample bind
    .run()
    .await
}