use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "static").show_files_listing())
            .service(Files::new("/pkg", "lib/pkg"))
            .service(Files::new(
                "/examples/circle/static",
                "examples/circle/static",
            ))
            .service(Files::new("/examples/circle/pkg", "examples/circle/pkg"))
            .service(Files::new(
                "/examples/perfchart/static",
                "examples/perfchart/static",
            ))
            .service(Files::new(
                "/examples/perfchart/pkg",
                "examples/perfchart/pkg",
            ))
            .service(Files::new(
                "/examples/simple/static",
                "examples/simple/static",
            ))
            .service(Files::new("/examples/simple/pkg", "examples/simple/pkg"))
            .service(Files::new(
                "/examples/triangle/static",
                "examples/triangle/static",
            ))
            .service(Files::new(
                "/examples/triangle/pkg",
                "examples/triangle/pkg",
            ))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
