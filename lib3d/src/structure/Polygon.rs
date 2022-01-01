use crate::structure::Vertex;
#[derive(Default, Debug)]
pub struct Polygon {
    pub vertecies: Vec<Vertex::Vertex>,
    pub normal: Vertex::Vertex
}

pub fn polygon_to_renderable(polygon: Polygon, list:&mut Vec<Vertex::GLVertex3d>){
    for i in polygon.vertecies {
        list.push(i.to())
    }
}