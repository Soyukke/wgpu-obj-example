// 頂点リスト
// 法線リスト

use std::ops::MulAssign;

use bytemuck::{Pod, Zeroable};
use ndarray::Array1;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Vertex {
    pos: [f32; 4],
}
impl Vertex {
    pub fn new(v: &Array1<f32>) -> Self {
        Self {pos: [v[0], v[1], v[2], 1.]}
    }
}

#[derive(Debug)]
pub struct Geometry {
    pub vs: Vec<Vec<Array1<f32>>>,
    pub ns: Vec<Array1<f32>>,
}

impl Geometry {
    pub fn new(vs: Vec<Vec<Array1<f32>>>, ns: Vec<Array1<f32>>) -> Self {
        Self { vs, ns }
    }

    pub fn scale(&mut self, v: Array1<f32>) {
        let x: Vec<Vec<Array1<f32>>> = self.vs.iter_mut().map(|vl|
            vl.iter_mut().map(|v1| v1.clone() * &v).collect()
        ).collect();
        //let mut p = Array1::from_vec(vec![0., 0., 0.]);
        //p = p * v;
        //for vl in self.vs.into_iter.() {
        //    for vt in vl.iter_mut() {
        //        //println!("vt {}", vt);
        //        //vt * 3.;
        //        //vt = vt * &v;
        //    }
        //}
        self.vs = x;
    }
    
    pub fn verts(&self) -> Vec<Vertex> {
        let mut verts = vec![];
        for vl in self.vs.iter() {
            for v in vl {
                verts.push(Vertex::new(&v));
            }
        }

        verts
    }
}

pub trait GeomMaker {
    fn new() -> Geometry;
    fn indices() -> Vec<u16>;
}

pub trait IGeometry {
    fn vertices(&self) -> Vec<Vertex>;
    fn indices(&self) -> Vec<u16>;
}
 
pub struct GeomUtil {}
impl GeomUtil {
    pub fn verts(vvs: Vec<Vec<Array1<f32>>>) -> Vec<Vertex> {
        let mut verts = vec![];
        for vl in vvs.iter() {
            for v in vl {
                verts.push(Vertex::new(&v));
            }
        }
        verts
    }
}
