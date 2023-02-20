use super::node::domain_to_node_dto;
use crate::domain::Nodeset;
use crate::dto::nodeset::{AlterNodesetDto, CreateNodesetDto, NodesetDto};
use std::collections::HashMap;

pub fn create_nodeset_to_domain(create: &CreateNodesetDto) -> Nodeset {
    Nodeset {
        name: create.name.to_string(),
        nodes: HashMap::new(),
    }
}

pub fn alter_nodeset_to_domain(exsisting: &Nodeset, altered: &AlterNodesetDto) -> Nodeset {
    Nodeset {
        name: altered.name.to_string(),
        nodes: exsisting.nodes.clone(),
    }
}

pub fn domain_to_nodeset_dto(domain: &Nodeset) -> NodesetDto {
    NodesetDto {
        name: domain.name.to_string(),
        nodes: domain
            .nodes
            .clone()
            .into_iter()
            .map(|(key, value)| (key.to_string(), domain_to_node_dto(&value)))
            .collect(),
    }
}
