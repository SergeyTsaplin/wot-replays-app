mod api_models;
mod config;
mod indexer;
use actix_web::{
    middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};

async fn list_replays(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn server_info(_req: HttpRequest) -> impl Responder {
    let payload = api_models::ServerInfo {
        server_version: format!("{}.{}.{}{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH"),
            option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")),
        supported_api_versions: vec!["v1".to_owned()]
    };
    if let Ok(serialized_payload )= serde_json::to_string(&payload) {
        return HttpResponse::Ok()
            .content_type("application/json")
            .body(serialized_payload)
    };
    HttpResponse::InternalServerError().body("Internal server error")
}


/// 404 Custom hadler
async fn not_found_handler(req: HttpRequest) -> impl Responder {
    let headers = req.headers();
    if let Some(content_type) = headers.get("accept") {
        if let Ok(v) = content_type.to_str() {
            if v.eq_ignore_ascii_case("application/json") {
                let payload = api_models::WebError {
                    code: 404,
                    message: "Resource not found".to_owned(),
                };
                if let Ok(serialized_payload )= serde_json::to_string(&payload) {
                    return HttpResponse::NotFound()
                        .content_type("application/json")
                        .body(serialized_payload)
                }
            }
        }
    }
    HttpResponse::NotFound()
        .content_type("text/plain")
        .body("Resource not found")
}

#[actix_web::main]
async fn main() -> Result<(), config::StdErr> {
    env_logger::init();
    let config = match config::read_config() {
        Ok(config) => config,
        Err(e) => {
            log::error!("Invalid configuration: {}", e);
            return Err(e);
        }
    };
    let version = format!("{}.{}.{}{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
        option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));
    let lib_version = wot_replays::get_version();
    log::info!("Statring the WoT replays API-server (server version: {}, wot-replays-lib version: {})", version, lib_version);
    log::info!("Configuration: {:?}", config);

    let bind_address = format!("{}:{}", config.host, config.port);
    let service = HttpServer::new(|| {
        let logger = Logger::default();
        App::new().wrap(logger)
            .service(
                web::scope("/api").route("/server-info/", web::get().to(server_info))
            )
            .service(
                web::scope("/api/v1")
                    .route("/replays/", web::get().to(list_replays))
            ).default_service(web::route().to(not_found_handler))
    })
    .workers(1)
    .bind(bind_address);
    std::thread::spawn(|| indexer::index(config.path_to_watch));
    let result = match service {
        Ok(s) => match s.run().await {
            Err(e) => Err(e.into()),
            _ => Ok(())
        },
        Err(e) => Err(e.into())
    };
    result
}
