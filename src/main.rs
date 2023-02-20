mod domain;
mod dto;
mod endpoints;
mod global;
mod handlers;
mod mappers;
use crate::endpoints::{delete_nodeset, get_all_nodesets, get_nodeset, post_nodeset, put_nodeset};
use actix_web::{web, App, HttpServer};
use std::collections::HashMap;

fn add_test_data(name: &str) {
    let mut map = global::get_nodeset_hashmap();
    let identifier = name;

    let mut nodemap: HashMap<uuid::Uuid, domain::Node> = HashMap::new();

    let id = uuid::Uuid::new_v4();
    nodemap.insert(
        id,
        domain::Node {
            id: id,
            position: domain::Position {
                x: 200f64,
                y: 600f64,
                z: -200f64,
            },
            links: vec![],
        },
    );

    let id = uuid::Uuid::new_v4();
    nodemap.insert(
        id,
        domain::Node {
            id: id,
            position: domain::Position {
                x: 2200f64,
                y: 78300f64,
                z: -64300f64,
            },
            links: vec![],
        },
    );

    let set = domain::Nodeset {
        name: identifier.to_string(),
        nodes: nodemap,
    };

    map.insert(identifier.to_string(), set);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 3001;
    let bind_location: String = format!("{}:{}", "127.0.0.1", port.to_string());
    println!("starting server on {}", bind_location);

    add_test_data("test_set");
    add_test_data("many_things");
    add_test_data("many_more_things");
    add_test_data("funsies");

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api/v1").service(
                web::scope("/nodeset")
                    .service(
                        web::resource("/")
                            .route(web::post().to(post_nodeset))
                            .route(web::get().to(get_all_nodesets)),
                    )
                    .service(
                        web::resource("/{name}")
                            .route(web::delete().to(delete_nodeset))
                            .route(web::put().to(put_nodeset))
                            .route(web::get().to(get_nodeset)),
                    ),
            ),
        )
    })
    .bind(bind_location)?
    .run()
    .await
}
