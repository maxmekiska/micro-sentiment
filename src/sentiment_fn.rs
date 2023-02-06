use serde::Deserialize;
use rust_bert::pipelines::sentiment::SentimentModel;
use actix_web::{web, HttpResponse};

#[derive(Deserialize)]
pub struct TextRequest {
    pub text: String,
}

pub async fn sentiment_analysis(request: web::Json<TextRequest>) -> HttpResponse {
    let sentiment_model = SentimentModel::new(Default::default()).unwrap();
    let input = vec![request.text.as_str()];
    let output = sentiment_model.predict(&input);
    HttpResponse::Ok().json(output)
}
