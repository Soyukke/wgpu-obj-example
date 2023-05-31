use ndarray::Array1;

use crate::objs::geom::*;

pub struct Cube {}

impl IGeometry for Cube {
    fn vertices(&self) -> Vec<Vertex> {
        let mut vvs = vec![];
        let mut ns = vec![];
        //let n = 1. / f32::sqrt(2.);
        // bottom 
        let mut vs = vec![];
        vs.push(Array1::from_vec(vec![-1., 1., -1.]));
        vs.push(Array1::from_vec(vec![1., 1., -1.]));
        vs.push(Array1::from_vec(vec![1., -1., -1.]));
        vs.push(Array1::from_vec(vec![-1., -1., -1.]));
        ns.push(Array1::from_vec(vec![0., 0., -1.]));
        vvs.push(vs);

        // top 
        let mut vs = vec![];
        vs.push(Array1::from_vec(vec![-1., -1., 1.]));
        vs.push(Array1::from_vec(vec![1., -1., 1.]));
        vs.push(Array1::from_vec(vec![1., 1., 1.]));
        vs.push(Array1::from_vec(vec![-1., 1., 1.]));
        ns.push(Array1::from_vec(vec![0., 0., 1.]));
        vvs.push(vs);

        // front
        let mut vs = vec![];
        vs.push(Array1::from_vec(vec![1., 1., -1.]));
        vs.push(Array1::from_vec(vec![-1., 1., -1.]));
        vs.push(Array1::from_vec(vec![-1., 1., 1.]));
        vs.push(Array1::from_vec(vec![1., 1., 1.]));
        ns.push(Array1::from_vec(vec![0., 1., 0.]));
        vvs.push(vs);
        
        // back
        let mut vs = vec![];
        vs.push(Array1::from_vec(vec![1., -1., 1.]));
        vs.push(Array1::from_vec(vec![-1., -1., 1.]));
        vs.push(Array1::from_vec(vec![-1., -1., -1.]));
        vs.push(Array1::from_vec(vec![1., -1., -1.]));
        ns.push(Array1::from_vec(vec![0., -1., 0.]));
        vvs.push(vs);

        // left
        let mut vs = vec![];
        vs.push(Array1::from_vec(vec![-1., -1., 1.]));
        vs.push(Array1::from_vec(vec![-1., 1., 1.]));
        vs.push(Array1::from_vec(vec![-1., 1., -1.]));
        vs.push(Array1::from_vec(vec![1., -1., -1.]));
        ns.push(Array1::from_vec(vec![-1., 0., 0.]));
        vvs.push(vs);

        let mut vs = vec![];
        vs.push(Array1::from_vec(vec![1., -1., -1.]));
        vs.push(Array1::from_vec(vec![1., 1., -1.]));
        vs.push(Array1::from_vec(vec![1., 1., 1.]));
        vs.push(Array1::from_vec(vec![1., -1., 1.]));
        ns.push(Array1::from_vec(vec![-1., 0., 0.]));
        vvs.push(vs);

        GeomUtil::verts(vvs)
    }

    fn indices(&self) -> Vec<u16> {
        Vec::from([
            0, 1, 2, 2, 3, 0, // top
            4, 5, 6, 6, 7, 4, // bottom
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // front
            20, 21, 22, 22, 23, 20, // back
        ])
    }
}
