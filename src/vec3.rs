pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    fn x(&self) -> f64 { self.e[0] }
    fn y(&self) -> f64 { self.e[1] }
    fn z(&self) -> f64 { self.e[2] }
}