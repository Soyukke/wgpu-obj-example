use ndarray::Array1;
use wave1::*;
use wave1::objs::*;
fn main() {
    println!("hel");


    let mut s = Sphere::new(5, 5);
    println!("vs: {:?}", s.verts().len());
    println!("indices: {:?}", s.indices0());

    let mut s = Cylinder::new(5, 1., 1.);
    println!("vs: {:?}", s.verts().len());
    println!("indices: {:?}", s.indices0());

}
