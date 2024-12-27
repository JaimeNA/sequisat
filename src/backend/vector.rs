
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    fn sum(&self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
    
    fn sub(&self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}