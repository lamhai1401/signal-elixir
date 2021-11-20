// use actix::prelude::{Addr, SyncArbiter};
// use actix_cors::Cors;
// use actix_web::{get, web, App, HttpServer, Responder};

// use std::env;

// #[get("/{id}/{name}/index.html")]
// async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Hello {}! id:{}", name, id)
// }

// pub async fn start() -> std::io::Result<()> {
//     // let frontend_origin = env::var("FRONTEND_ORIGIN").ok();
//     // let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");
//     HttpServer::new(|| App::new().service(index))
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
// }

// fn routes(app: &mut web::ServiceConfig) {}
