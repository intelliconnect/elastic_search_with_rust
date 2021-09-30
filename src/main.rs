use actix_web::{web, App, HttpServer};

mod elastic;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::scope("/elastic_search")
                    .route("/by", web::post().to(elastic::search))
                    .route("/include_exclude", web::post().to(elastic::search_complex))
                    .route("/create_user", web::post().to(elastic::create_index))
                    .route("/update_user", web::put().to(elastic::update))
                    .route("/delete_index", web::delete().to(elastic::delete_index)),
            )
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await
}
