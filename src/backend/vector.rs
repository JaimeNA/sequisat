#[derive(Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {

    pub fn new(x:f64, y: f64, z: f64) -> Self
    {
        Self {
            x, 
            y,
            z
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    pub fn sum(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
    
    pub fn sub(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }

    // Getters and Setters

    pub fn set_x(&mut self, x: f64)
    {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64)
    {
        self.y = y;
    }

    pub fn set_z(&mut self, z: f64)
    {
        self.z = z;
    }

    pub fn get_x(&self) -> f64 
    {
        self.x
    }

    pub fn get_y(&self) -> f64 
    {
        self.y
    }

    pub fn get_z(&self) -> f64 
    {
        self.z
    }
}