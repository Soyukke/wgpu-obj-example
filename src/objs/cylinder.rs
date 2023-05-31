use std::f32::consts::PI;

// 三角でsphereを作る
//
use ndarray::Array1;
use crate::objs::geom::*;

pub struct Cylinder {
    n: i32,
    l: f32,
    r: f32,
}


impl Cylinder {
    pub fn new(n: i32, l: f32, r: f32) -> Self {
        Self {n, l, r}
    }

    fn vert_top(&self) -> Array1<f32> {
        let p0 = Array1::from_vec(vec![0., 0., self.l]);
        p0
    }
    fn vert_bot(&self) -> Array1<f32> {
        let p0 = Array1::from_vec(vec![0., 0., 0.]);
        p0
    }

    fn vert_xy(&self, i: i32) -> Array1<f32> {
        let i = i as f32;
        let n = self.n as f32;
        let x = self.r * f32::cos(2.*PI*i/n);
        let y = self.r * f32::sin(2.*PI*i/n);
        let p = Array1::from_vec(vec![x, y]);
        p
    }

    pub fn verts(&self) -> Vec<Vertex> {
        let mut vs: Vec<Vertex> = vec![];
        vs.push(Vertex::new(&self.vert_top()));
        vs.push(Vertex::new(&self.vert_bot()));

        for i in 0..self.n {
            let xy = self.vert_xy(i);
            let p = Vertex::new(&Array1::<f32>::from_vec(vec![xy[0], xy[1], self.l, 1.]));
            vs.push(p)
        }

        for i in 0..self.n {
            let xy = self.vert_xy(i);
            let p = Vertex::new(&Array1::<f32>::from_vec(vec![xy[0], xy[1], 0., 1.]));
            vs.push(p)
        }
        vs
    }

    pub fn indices0(&self) -> Vec<u16> {
        let mut ids = vec![];
        // 0-1-2, 0-2-3, ..., 
        // 1-1-2, 1-2-3, ..., 
        // (0+offset0)-(1+offset0)-(0+offset1), (1+offset1)-(0+offset1)-(1+offset0), 
        let n = self.n as u16;
        let offset0 = 2;
        let offset1 = 2 + n;

        for i in 0..self.n {
            let i = i as u16;
            let i2 = i+1 % n;
            let mut v = vec![0, i+offset0, i2+offset0];
            ids.append(&mut v);
        }

        for i in 0..self.n {
            let i = i as u16;
            let i2 = i+1 % n;
            let mut v = vec![1, i+offset1, i2+offset1];
            ids.append(&mut v);
        }

        for i in 0..self.n {
            let i = i as u16;
            let i2 = i+1 % n;
            let mut v = vec![i+offset0, i2+offset0, i+offset1, i2+offset1, i+offset1, i2+offset0];
            ids.append(&mut v);
        }

        ids
    }
}

impl IGeometry for Cylinder {
    fn vertices(&self) -> Vec<Vertex> {
        self.verts()
    }
    fn indices(&self) -> Vec<u16> {
        self.indices0()
    }
}
