// Used to transform cartesian plane to canvas coordinates according to a reference point
pub struct Camera {
    pub x: f32,
    pub y: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32) -> Camera {
        Camera { x, y }
    }

    pub fn transform(&self, x: f32, y: f32) -> (f32, f32) {
        (x - self.x, y - self.y)
    }

    pub fn displace(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }
}
