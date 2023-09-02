use std::f64::consts;

use crate::geometric_primitives::{point::Point, line::Line, entity::Entity, utilities::define_vector_angle};

pub struct Arc {
    // Represents a geometric primitive, an arc.
    start: Point,
    terminate: Point,
    center_point: Point,
    lines: Option<Vec<Line>>,
}


impl Arc {
    pub fn new(start: Point, terminate: Point, center_point: Point) -> Self {
        Arc { start, terminate, center_point, lines: None}
    }

    fn approximate_by_line(&self) {
        // Approximate an arc by lines.
        let delta = (3.0 * consts::PI) / 180.0;
        let phi = 0.0;
        let terminate_angle = self.terminate_angle();
        let start_angle = self.start_angle();
        while phi < terminate_angle && phi >= terminate_angle - delta {
            todo!()

        }

    }

    fn arc_radius(&self) -> f64 {
        // Arc radius.
        // |AB|² = (y2 - y1)² + (x2 - x1)².
        let x1 = self.center_point.x;
        let x2 = self.start.x;
        let y1 = self.center_point.y;
        let y2 = self.start.y;
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).powf(0.5)

    }

    fn arc_angle(&self) -> f64 {
        // Arc angle
        let x1 = self.start.x;
        let x2 = self.terminate.x;
        let y1 = self.start.y;
        let y2 = self.terminate.y;
        ((x1 * x2 + y1 * y2) / ((x1 * x1 + y1 * y1).powf(0.5) * (x2 * x2 + y2 * y2).powf(0.5))).acos()
        
    }

    fn start_angle(&self) -> f64 {
        // Arc start angle
        let x1 = self.center_point.x + self.arc_radius();
        let x2 = self.start.x;
        let y1 = 0.0;
        let y2 = self.start.y;
        define_vector_angle(x1, x2, y1, y2)
    }

    fn terminate_angle(&self) -> f64 {
        // Arc terminate  angle
        let x1 = self.center_point.x + self.arc_radius();
        let x2 = self.terminate.x;
        let y1 = 0.0;
        let y2 = self.terminate.y;
        define_vector_angle(x1, x2, y1, y2)

    }
}


impl Entity for Arc {
    fn crossed(&self, ray: &super::line::Ray) -> bool {
        let mut res = 0;
        let lines = {
            if let Some(ref lines) = self.lines {
                lines
            
        } else {
            self.aproximate_by_line();
            self.lines.as_ref().unwrap()
        }
        };
        for line in lines {
            res += line.crossed(&ray) as u32
        }
        res > 0 && res % 2 == 1
    }
}