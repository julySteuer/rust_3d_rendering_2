use std::fs;
use crate::structure::Polygon::{Polygon};
use glium::{implement_vertex, vertex::VerticesSource};
#[derive(Default, Clone, Copy, Debug)]
pub struct Vertex {
    x: f64,
    y: f64,
    z: f64,
}

pub struct Vertex2d {
    x: f64,
    y: f64
}

#[derive(Copy, Clone)]
pub struct GLVertex2d {
    pub position: [f32; 2]
}
implement_vertex!(GLVertex2d, position);

#[derive(Copy, Clone)]
pub struct GLVertex3d {
    position: [f32; 3]
}
implement_vertex!(GLVertex3d, position);

impl Vertex {
    pub fn new(x: f64, y: f64, z: f64) -> Vertex{
        Vertex {
            x,
            y,
            z,
        }
    }

    pub fn to(&self) -> GLVertex3d{
        GLVertex3d{
            position: [self.x as f32, self.y as f32, self.z as f32] 
        }
    }

    pub fn normalize(&mut self, magnitude:f64) -> Vertex{
        self.x = self.x/magnitude;
        self.y = self.y/magnitude;
        self.z = self.z/magnitude;
        *self
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

pub fn load_from_obj(path: String)-> Vec<Polygon> { // TODO: fore every face thees is a normal not for evey polygon 
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut polygons: Vec<Polygon> = Vec::new();
    let mut normals: Vec<Vertex> = Vec::new();
    let content = fs::read_to_string(path).unwrap();
    let lines:Vec<&str> = content.split("\n").collect();
    for i in lines {
        let tokens: Vec<&str> = i.split(" ").into_iter().collect();
        match tokens[0] {
            "v" => {
                vertices.push(Vertex {
                    x: tokens[1].parse().unwrap(),
                    y: tokens[2].parse().unwrap(),
                    z: tokens[3].trim_end().parse().unwrap()
                })
            },
            "f" => {
                if polygons.is_empty() {
                    calc_magnitude(&mut vertices);
                } 
                let mut polygon: Polygon = Default::default(); 
                for i in &tokens[1..=3]{
                    let elem: Vec<&str> = i.split("/").collect();
                    polygon.vertecies.push(vertices[elem[0].trim_end().parse::<usize>().unwrap()-1]);
                    polygon.normal = normals[elem[2].trim_end().parse::<usize>().unwrap()-1];
                }
                println!("{:?}", polygon);
                polygons.push(polygon);
            },
            "vn" => {
                normals.push(Vertex {
                    x: tokens[1].parse().unwrap(),
                    y: tokens[2].parse().unwrap(),
                    z: tokens[3].trim_end().parse().unwrap()
                })
            },
            _ => ()
        }
    }
    polygons
}

pub fn calc_magnitude(vertices: &mut Vec<Vertex>){
    let mut largest_magnitude:f64 = 0.0;
    for elem in vertices.clone(){
        if elem.magnitude() > largest_magnitude {
            largest_magnitude = elem.magnitude()
        }
    }
    for elem in vertices{
        elem.normalize(largest_magnitude);
    }
}

impl Vertex2d {
    pub fn new(x: f64, y: f64) -> Vertex2d{
        Vertex2d {
            x,
            y
        }
    }

    pub fn to(&self)->GLVertex2d{
        GLVertex2d{
            position: [self.x as f32, self.y as f32]
        }
    }
}