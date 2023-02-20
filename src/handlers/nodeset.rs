use crate::domain::Nodeset;
use crate::dto::nodeset::{AlterNodesetDto, CreateNodesetDto};
use crate::global::get_nodeset_hashmap;
use crate::mappers::nodeset::{alter_nodeset_to_domain, create_nodeset_to_domain};

pub async fn handle_create_nodeset(nodeset_dto: CreateNodesetDto) -> Result<Nodeset, ()> {
    let mut map = get_nodeset_hashmap();
    let get_result = map.get(&nodeset_dto.name.clone());

    match get_result {
        Some(_value) => Err(()),
        None => {
            let nodeset = create_nodeset_to_domain(&nodeset_dto);
            let identifier = nodeset.name.to_string();
            map.insert(identifier.clone(), nodeset);
            Ok(map.get(&identifier).unwrap().clone())
        }
    }
}

pub async fn handle_get_nodeset(identifier: &String) -> Option<Nodeset> {
    let map = get_nodeset_hashmap();
    map.get(identifier).cloned()
}

pub async fn handle_get_all_nodesets() -> Vec<Nodeset> {
    let map = get_nodeset_hashmap();
    map.values().cloned().collect()
}

pub async fn handle_delete_nodeset(name: &String) -> Option<Nodeset> {
    let mut map = get_nodeset_hashmap();
    map.remove(&name.to_string())
}

pub async fn handle_put_nodeset(name: &String, dto: AlterNodesetDto) -> Result<Nodeset, ()> {
    let unwrapped_dto = dto.into();
    let mut map = get_nodeset_hashmap();
    let identifier = name.to_string();
    let get_result = map.get(&identifier);

    match get_result {
        Some(value) => {
            let altered_domain = alter_nodeset_to_domain(value, &unwrapped_dto);
            map.insert(identifier.clone(), altered_domain);
            Ok(map.get(&identifier).unwrap().clone())
        }
        None => Err(()),
    }
}
