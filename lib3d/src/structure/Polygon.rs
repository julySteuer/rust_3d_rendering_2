use crate::structure::Vertex;
#[derive(Default, Debug)]
pub struct Polygon {
    pub vertecies: Vec<u16>,
    pub normal: u16
}

pub fn polygon_to_renderable(polygon: Polygon) -> Vec<u16> {
    polygon.vertecies
}