use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};
use futures::StreamExt;

pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
    }
    Ok(HttpResponse::Ok().into())
}

async fn create_local_file() -> std::io::Result<std::fs::File> {
    std::fs::File::create("")
}
