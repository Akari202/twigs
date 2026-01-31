use std::ops::Index;
use nalgebra::SMatrix;
use vec_utils::vec3d::Vec3d;
use crate::material::Material;
use crate::render::vertex::Vertex;

pub mod load;

#[derive(Debug, Clone)]
pub struct Mesh {
    nodes: Vec<Vec3d>,
    elements: Vec<Element>,
    material: Material
}

#[derive(Debug, Clone)]
pub struct Element {
    nodes: Vec<usize>
}

impl Mesh {
    pub fn new(nodes: Vec<Vec3d>, elements: Vec<Element>, material: Material) -> Self {
        Self {
            nodes,
            elements,
            material
        }
    }
    
    pub fn render_data(&self) -> (Vec<Vertex>, Vec<u16>) {
        let nodes = self.nodes
            .iter()
            .map(|i| {
                Vertex::from_vec3d(i, self.material.color).scale(50.0)
            })
            .collect::<Vec<Vertex>>();
        let connections = self.elements
            .iter()
            .map(|i| {
                vec![
                    i[0], i[1],
                    i[0], i[3],
                    i[0], i[4],
                    i[1], i[2],
                    i[1], i[5],
                    i[2], i[3],
                    i[2], i[6],
                    i[3], i[7],
                    i[4], i[5],
                    i[4], i[7],
                    i[5], i[6],
                    i[6], i[7],

                    // i[0], i[2],
                    // i[0], i[5],
                    // i[0], i[7],
                    // i[1], i[6],
                    // i[3], i[6],
                    // i[4], i[6]
                ]
            })
            .flatten()
            .map(|i| {
                i as u16
            })
            .collect::<Vec<u16>>();
        (nodes, connections)
    }
}

impl Element {
    pub fn new(nodes: Vec<usize>) -> Self {
        Self {
            nodes
        }
    }
}

impl Index<usize> for &Element {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}



