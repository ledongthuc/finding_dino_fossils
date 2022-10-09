#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    use actix_files;

    HttpServer::new(|| {
            App::new().service(
                actix_files::Files::new("/", "./static")
                .prefer_utf8(true)
                .index_file("index.html"),
            )
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
