use super::point::Point;
use super::entity::Entity;

pub struct Line {
    pub start: Point,
    pub terminate: Point,
}

impl Line {
    pub fn new(start: Point, terminate: Point) -> Self {
        Line { start, terminate }
    }

    fn max_point_x(&self) -> f64 {
        if self.start.x > self.terminate.x {
            self.start.x
        } else { self.terminate.x }
    }

    fn min_point_y(&self) -> f64 {
        if self.start.y < self.terminate.y {
            self.start.y
        } else { self.terminate.y }
    }

    fn max_point_y(&self) -> f64 {
        if self.start.y > self.terminate.y {
            self.start.y
        } else { self.terminate.y }
    }

    
}

impl Entity for Line {
    fn crossed(&self, ray: &Ray) -> bool {
        if self.max_point_x() > ray.0.x
            && ray.0.y > self.min_point_y()
            && ray.0.y < self.max_point_y() {
                true
        } else { false }
    }

}

pub struct Ray(pub Point, pub Point);