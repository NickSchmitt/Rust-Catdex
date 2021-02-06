use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse};

use handlebars::Handlebars;
use serde_json::json;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse { 
    let data = json!({
        "project_name": "Catdex",
        "cats": [
            {
                "name":"Bengal",
                "image_path":"/static/image/Bengal_Cat.jpg"
            },
            {
                "name":"Persian",
                "image_path":"/static/image/White_Persian_Cat.jpg"
            },
            {
                "name":"Maine Coon",
                "image_path":"/static/image/Maine_Coon_Cat.jpg"
            }
        ]
    });

    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "/static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    println!("Listening on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(
                Files::new("/static", "static")
                    .show_files_listing(),
            )
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}