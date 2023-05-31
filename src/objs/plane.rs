use ndarray::Array1;

use crate::objs::geom::*;

pub struct Plane {
    n: i32,
    m: i32,
}

impl Plane {
    pub fn new(n: i32, m: i32) -> Self {
        Self { n, m }
    }
}

impl IGeometry for Plane {
    fn vertices(&self) -> Vec<Vertex> {
        let mut vvs = vec![];

        let n = self.n as f32;
        let m = self.m as f32;
        for i in 0..self.n {
            for j in 0..self.m {
                let i = i as f32;
                let j = j as f32;
                let x1 = i * 1.0 / n;
                let x2 = (i+1.0) * 1.0 / n;
                let y1 = j * 1.0 / m;
                let y2 = (j+1.0) * 1.0 / m;

                let mut vs = vec![];
                vs.push(Array1::from_vec(vec![x1, y1, 0.]));
                vs.push(Array1::from_vec(vec![x2, y1, 0.]));
                vs.push(Array1::from_vec(vec![x2, y2, 0.]));
                vs.push(Array1::from_vec(vec![x1, y2, 0.]));
                vvs.push(vs);
            }
        }


        GeomUtil::verts(vvs)
    }

    fn indices(&self) -> Vec<u16> {
        let mut ids: Vec<u16> = vec![];
        for i in 0..self.n {
            for j in 0..self.m {
                let i = i as u16;
                let j = j as u16;
                let m = self.m as u16;
                let offset = 4*m*i + 4*j;
                ids.append(&mut vec![0+offset, 1+offset, 2+offset]);
                ids.append(&mut vec![2+offset, 3+offset, 0+offset]);
            }
        }

        ids
    }
}
