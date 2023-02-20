use crate::{
    domain::{Node, Position},
    dto::node::NodeDto,
};

fn position_to_vector(pos: &Position) -> Vec<f64> {
    vec![pos.x, pos.y, pos.z]
}

pub fn domain_to_node_dto(domain: &Node) -> NodeDto {
    NodeDto {
        id: domain.id.to_string(),
        links: domain
            .links
            .clone()
            .into_iter()
            .map(|l| l.to_string())
            .collect(),
        position: position_to_vector(&domain.position),
    }
}
