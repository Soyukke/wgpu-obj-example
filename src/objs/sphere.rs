use std::f32::consts::PI;
//
use ndarray::Array1;
use crate::objs::geom::*;

pub struct Sphere {
    n: i32,
    m: i32,
}

impl Sphere {
    pub fn new(n: i32, m: i32) -> Self {
        Self {n, m}
    }

    fn vert(&self, i: i32, j: i32) -> Array1<f32> {
        let i = i as f32;
        let j = j as f32;
        let n = self.n as f32;
        let m = self.m as f32;
        let r = 1.;
        let xy = f32::sin(PI*i/n);
        let z = r * f32::cos(PI*i/n);
        let x = r * xy * f32::cos(2.*PI*j/m);
        let y = r * xy * f32::sin(2.*PI*j/m);
        let p = Array1::from_vec(vec![x, y, z]);
        p
    }

    pub fn verts(&self) -> Vec<Vec<Array1<f32>>> {
        let mut vvs:Vec<Vec<Array1<f32>>> = vec![];
        for i in 0..self.n {
            for j in 0..self.m {
                let i2 = i+1 % self.n;
                let j2 = j+1 % self.m;
                let mut vs: Vec<Array1<f32>> = vec![];
                vs.push(self.vert(i, j));
                vs.push(self.vert(i2, j));
                vs.push(self.vert(i2, j2));
                vs.push(self.vert(i, j2));
                vvs.push(vs);
            }
        }
        vvs
    }

    pub fn indices0(&self) -> Vec<u16> {
        let mut ids = vec![];
        for i in 0..self.n {
            for j in 0..self.m {
                // i*m + jが進むごとに4進む
                let i = i as u16;
                let j = j as u16;
                let m = self.m as u16;
                let offset = (i*m + j)*4;
                // 0 1 2 / 2 3 0
                ids.push(0+offset);
                ids.push(1+offset);
                ids.push(2+offset);
                ids.push(2+offset);
                ids.push(3+offset);
                ids.push(0+offset);
            }
        }
        ids
    }
}

impl IGeometry for Sphere {
    fn vertices(&self) -> Vec<Vertex> {
        GeomUtil::verts(self.verts())
    }
    fn indices(&self) -> Vec<u16> {
        self.indices0()
    }
}

