use actix_web::{get, post, put, patch, delete, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
         tera
    };
}

#[get("/")]
async fn index() -> impl Responder {
    let mut context = tera::Context::new();

    context.insert("rust_context_state", "Alo Medo!");

    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[post("/echo")]
async fn echo(req: String) -> impl Responder {
    HttpResponse::Ok().body(req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        // .service(hello)
        // .service(echo)
        .service(index)
    }
    ).bind(("127.0.0.1", 7878))?
    .run()
    .await
}
