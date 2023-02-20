use serde::Serialize;

#[derive(Serialize)]
pub struct NodeDto {
    pub id: String,
    pub links: Vec<String>,
    pub position: Vec<f64>,
}
