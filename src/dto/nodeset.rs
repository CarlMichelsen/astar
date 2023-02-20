use super::node::NodeDto;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CreateNodesetDto {
    pub name: String,
}

#[derive(Deserialize)]
pub struct AlterNodesetDto {
    pub name: String,
}

#[derive(Serialize)]
pub struct NodesetDto {
    pub name: String,
    pub nodes: HashMap<String, NodeDto>,
}
