use crate::dto::nodeset::{AlterNodesetDto, CreateNodesetDto, NodesetDto};
use crate::handlers::nodeset::{
    handle_create_nodeset, handle_delete_nodeset, handle_get_all_nodesets, handle_get_nodeset,
    handle_put_nodeset,
};
use crate::mappers::nodeset::domain_to_nodeset_dto;
use actix_web::{web, HttpResponse, Responder};

pub async fn post_nodeset(dto: web::Json<CreateNodesetDto>) -> impl Responder {
    let add_if_not_exists = handle_create_nodeset(dto.into_inner().into()).await;
    match add_if_not_exists {
        Ok(value) => {
            let response = domain_to_nodeset_dto(&value);
            HttpResponse::Ok().json(response)
        }
        Err(()) => HttpResponse::Forbidden().body("already exsists"),
    }
}

pub async fn get_nodeset(name: web::Path<String>) -> impl Responder {
    let get_result = handle_get_nodeset(&name.to_string()).await;
    match get_result {
        Some(value) => {
            let response = domain_to_nodeset_dto(&value);
            HttpResponse::Ok().json(response)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn get_all_nodesets() -> impl Responder {
    let list = handle_get_all_nodesets().await;
    let dto_list = list
        .iter()
        .map(|x| domain_to_nodeset_dto(x))
        .collect::<Vec<NodesetDto>>();
    HttpResponse::Ok().json(dto_list)
}

pub async fn delete_nodeset(name: web::Path<String>) -> impl Responder {
    let delete_result = handle_delete_nodeset(&name.to_string()).await;

    match delete_result {
        Some(_value) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn put_nodeset(
    name: web::Path<String>,
    dto: web::Json<AlterNodesetDto>,
) -> impl Responder {
    let put_result = handle_put_nodeset(&name, dto.into_inner().into()).await;

    match put_result {
        Ok(value) => {
            let response = domain_to_nodeset_dto(&value);
            HttpResponse::Ok().json(response)
        }
        Err(()) => HttpResponse::NotFound().finish(),
    }
}
