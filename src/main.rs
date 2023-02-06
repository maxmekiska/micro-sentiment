use actix_web::{web, App, HttpResponse, HttpServer};
mod sentiment_fn;


async fn root() -> HttpResponse {
    HttpResponse::Ok().body("Sentiment Microservice Standby")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(root))
            .route("/sentiment", web::post().to(sentiment_fn::sentiment_analysis))

    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}