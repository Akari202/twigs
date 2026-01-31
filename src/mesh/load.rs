use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};
use log::{debug, trace};
use vec_utils::vec3d::Vec3d;
use crate::material::Material;
use crate::mesh::{Element, Mesh};

impl Mesh {
    pub fn load(name: String, material: Material) -> Result<Self, Box<dyn Error>> {
        let elements_file_name = format!("./meshes/{}Elements", &name);
        debug!("Loading elements from {}", &elements_file_name);
        let elements_reader = BufReader::new(fs::File::open(&elements_file_name)?);
        let elements = elements_reader
            .lines()
            .enumerate()
            .map(|(i, j)| {
                let j = j.unwrap();
                let mut split = j.split(",");
                split.next();
                let element = Element::new(split
                    .map(|k| {
                        k.trim()
                            .parse::<usize>()
                            .unwrap() - 1
                    })
                    .collect::<Vec<usize>>()
                );
                trace!("Parsed element {}: {:?}", i, &element);
                element
            })
            .collect::<Vec<Element>>();
        debug!("Parsed {} elements", elements.len());
        let nodes_file_name = format!("./meshes/{}Nodes", &name);
        debug!("Loading nodes from {}", &nodes_file_name);
        let nodes_reader = BufReader::new(fs::File::open(nodes_file_name)?);
        let mut nodes = nodes_reader
            .lines()
            .enumerate()
            .map(|(i, j)| {
                let j = j.unwrap();
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
            material
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