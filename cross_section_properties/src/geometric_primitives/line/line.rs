use crate::geometric_primitives::{point::Point2D, entity::Entity};


pub struct Line {
    pub start: Point2D,
    pub terminate: Point2D,
}

impl Line {
    pub fn new(start: Point2D, terminate: Point2D) -> Self {
        Line { start, terminate }
    }

    fn max_point_y(&self) -> f64 {
        if self.start.y > self.terminate.y {
            self.start.y
        } else { self.terminate.y }
    }

    fn min_point_z(&self) -> f64 {
        if self.start.z < self.terminate.z {
            self.start.z
        } else { self.terminate.z }
    }

    fn max_point_z(&self) -> f64 {
        if self.start.z > self.terminate.z {
            self.start.z
        } else { self.terminate.z }
    }

    
}

impl Entity for Line {
    fn number_of_crossings_by_ray(&self, ray: &Ray) -> i32 {
        if self.max_point_y() > ray.0.y
            && ray.0.z > self.min_point_z()
            && ray.0.z < self.max_point_z() {
                1
        } else { 0 }
    }

}

pub struct Ray(pub Point2D, pub Point2D);