#[macro_use]
extern crate diesel;

use actix_files::Files;
use actix_web::{http, web, App, Error, HttpResponse, HttpServer};
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::Serialize;

use self::models::*;

use handlebars::Handlebars;

mod models;
mod schema;
use self::schema::cats::dsl::*; // cats alias

// PgConnection comes from diesel::prelde
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize)]
struct IndexTemplateData {
    project_name: String,
    cats: Vec<self::models::Cat>
}

async fn index(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> { 
        let connection = pool.get()
            .expect("Can't get db connection from pool");
        let cats_data = web::block(move || { cats.limit(100).load::<Cat>(&connection)
        })
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
        let data = IndexTemplateData { 
            project_name: "Catdex".to_string(), 
            cats: cats_data,
        };
        let body = hb.render("index", &data).unwrap();
        Ok(HttpResponse::Ok().body(body))
}

async fn add(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse, Error> {

    let body = hb.render("add", &{}).unwrap();

    Ok(HttpResponse::Ok().body(body))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "/static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    // setting up daabase conneciton pool
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");

    println!("Listening on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .data(pool.clone())
            .service(
                Files::new("/static", "static")
                    .show_files_listing(),
            )
            .route("/", web::get().to(index))
            .route("/add", web::get().to(add))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
