pub fn project(aspect_ratio: f64, fov: f64, q: f64, near: f64, far: f64)-> [[f64; 4]; 4]{
    let proj: [[f64; 4]; 4] = [
        [aspect_ratio*fov, 0.0, 0.0, 0.0], 
        [0.0, fov, 0.0, 0.0], 
        [0.0,0.0,q,1.0], 
        [0.0,0.0,(-far * near)/(far - near),0.0],
        ];
    proj
}