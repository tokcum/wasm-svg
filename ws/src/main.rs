use actix_files::{Files};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(Files::new("/static", "static"))
      .service(Files::new("/pkg", "lib/pkg"))
      .route("/", web::get().to(index))
  })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
