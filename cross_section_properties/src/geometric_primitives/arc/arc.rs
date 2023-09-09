use std::f64::consts;

use crate::geometric_primitives::{point::Point2D, entity::Entity,
    utilities::define_vector_angle, line::line::{Ray, Line}};

pub struct Arc {
    // Represents a geometric primitive, an arc.

    start: Point2D,
    terminate: Point2D,
    center_point: Point2D,
    lines: Vec<Line>
}


impl Arc {
    pub fn new(start: Point2D, terminate: Point2D, center_point: Point2D) -> Self {
        let mut arc = Arc { start, terminate, center_point, lines: vec![]};
        arc.approximate_by_line();
        arc
    }

    fn approximate_by_line(&mut self) {
        // Approximate an arc by lines.
        
        let mut lines: Vec<Line> = vec![];
        let delta = (3.0 * consts::PI) / 180.0;
        let terminate_angle = self.terminate_angle();
        let mut phi = self.start_angle();
        let mut prev_point = self.start;
        let arc_radius = self.arc_radius();
        while !(phi < terminate_angle && phi >= terminate_angle - delta) {
            phi += delta;
            if phi > consts::PI * 2.0 {
                phi = phi - consts::PI * 2.0;
            }
            let y = arc_radius * phi.cos();
            let z = arc_radius * phi.sin();
            let next_point = Point2D::new(y, z);
            let line = Line::new(prev_point, next_point);
            lines.push(line);
            prev_point = next_point;
        }
        let line = Line::new(prev_point, self.terminate);
        lines.push(line);
        self.lines = lines;


    }

    pub(crate) fn arc_radius(&self) -> f64 {
        // Arc radius.
        // |AB|² = (z2 - z1)² + (y2 - y1)².

        let y1 = self.center_point.y;
        let y2 = self.start.y;
        let z1 = self.center_point.z;
        let z2 = self.start.z;
        ((y2 - y1).powi(2) + (z2 - z1).powi(2)).powf(0.5)

    }

    pub(crate) fn start_angle(&self) -> f64 {
        // Arc start angle.

        let y1 = self.center_point.y + self.arc_radius();
        let y2 = self.start.y;
        let z1 = 0.0;
        let z2 = self.start.z;
        define_vector_angle(y1, y2, z1, z2)
    }

    pub(crate) fn terminate_angle(&self) -> f64 {
        // Arc terminate  angle.

        let y1 = self.center_point.y + self.arc_radius();
        let y2 = self.terminate.y;
        let z1 = 0.0;
        let z2 = self.terminate.z;
        define_vector_angle(y1, y2, z1, z2)

    }
}


impl Entity for Arc {
    fn number_of_crossings_by_ray(&self, ray: &Ray) -> i32 {
        let mut res = 0;
        for line in self.lines.iter() {
            res += line.number_of_crossings_by_ray(&ray)
        }
        res
    }
}