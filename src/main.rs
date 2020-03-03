use actix_web::{middleware, web, App, HttpServer};
mod config;
mod endpoint;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Get the config filepath from command arguments
    // If missing, use a default value
    let args: Vec<String> = std::env::args().collect();
    let conf_path = if args.len() == 1 {
        "config.toml"
    } else {
        &args[1]
    };
    let config = std::sync::Arc::new(load_config(conf_path));
    // Start the HTTP server proper
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::post().to(endpoint::upload_file)))
    })
    .bind(config.http.get_url())?
    .run()
    .await
}

fn load_config(filename: &str) -> config::ThingputConfig {
    if std::path::Path::new(filename).exists() {
        let contents = std::fs::read(filename).unwrap();
        toml::from_slice(&contents[..]).unwrap()
    } else {
        config::ThingputConfig::new()
    }
}
