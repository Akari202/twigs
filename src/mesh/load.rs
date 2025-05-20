use std::error::Error;
use std::fs;
use std::io::BufRead;
use log::{debug, trace};
use vec_utils::vec3d::Vec3d;
use crate::material::Material;
use crate::mesh::Mesh;

impl Mesh {
    pub fn load(name: String) -> Result<Self, Box<dyn Error>> {
        let elements_file_name = format!("{}Elements", &name);
        let elements_file = fs::File::open(elements_file_name)?;
        let elements = elements_file
            .lines()
            .enumerate()
            .map(|(i, j)| {
                let mut split = j.split(",");
                split.next();
                let element = split
                    .map(|k| {
                        k.trim()
                            .parse::<usize>()
                            .unwrap() - 1
                    })
                    .collect::<Vec<usize>>();
                trace!("Parsed element {}: {:?}", i, &element);
                element
            })
            .collect::<Vec<Vec<usize>>>();
        debug!("Parsed {} elements", elements.len());
        let nodes_file_name = format!("{}Nodes", &name);
        let nodes_file = fs::File::open(nodes_file_name)?;
        let mut nodes = nodes_file
            .lines()
            .enumerate()
            .map(|(i, j)| {
                let mut split = j.split(",");
                split.next();
                let x = split.next().unwrap().trim().parse::<f64>().unwrap();
                let y = split.next().unwrap().trim().parse::<f64>().unwrap();
                let z = split.next().unwrap().trim().parse::<f64>().unwrap();
                let point = Vec3d::new(x, y, z);
                trace!("Parsed point {}: {}", i, &point);
                point
            })
            .collect::<Vec<Vec3d>>();
        debug!("Parsed {} nodes", nodes.len());
        Ok(Self {
            nodes,
            elements,
            material: Material::
        })
    }
}

pub fn load_test_file() -> (Vec<Vec3d>, Vec<Vec<usize>>){
    let elements_file = include_str!("../../meshes/SimpleElements");
    let nodes_file = include_str!("../../meshes/SimpleNodes");
    let mut nodes = nodes_file
        .lines()
        .enumerate()
        .map(|(i, j)| {
            let mut split = j.split(",");
            split.next();
            let x = split.next().unwrap().trim().parse::<f64>().unwrap();
            let y = split.next().unwrap().trim().parse::<f64>().unwrap();
            let z = split.next().unwrap().trim().parse::<f64>().unwrap();
            let point = Vec3d::new(x, y, z);
            trace!("Parsed point {}: {}", i, &point);
            point
        })
        .collect::<Vec<Vec3d>>();
    debug!("Parsed {} nodes", nodes.len());
    let elements = elements_file
        .lines()
        .enumerate()
        .map(|(i, j)| {
            let mut split = j.split(",");
            split.next();
            let element = split
                .map(|k| {
                    k.trim()
                        .parse::<usize>()
                        .unwrap() - 1
                })
                .collect::<Vec<usize>>();
            trace!("Parsed element {}: {:?}", i, &element);
            element
        })
        .collect::<Vec<Vec<usize>>>();
    debug!("Parsed {} elements", elements.len());
    (nodes, elements)
}