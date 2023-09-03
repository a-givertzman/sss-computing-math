use std::f64::consts;

use crate::geometric_primitives::{point::Point, line::Line, entity::Entity, utilities::define_vector_angle};

pub struct Arc {
    // Represents a geometric primitive, an arc.
    start: Point,
    terminate: Point,
    center_point: Point,
    lines: Vec<Line>
}


impl Arc {
    pub fn new(start: Point, terminate: Point, center_point: Point) -> Self {
        Arc::approximate_by_line(start, terminate, center_point)
    }

    fn approximate_by_line(start: Point, terminate: Point, center_point: Point) -> Arc {
        // Approximate an arc by lines.
        let mut arc = Arc { start, terminate, center_point, lines: vec![]};
        let mut lines: Vec<Line> = vec![];
        let delta = (3.0 * consts::PI) / 180.0;
        let terminate_angle = arc.terminate_angle();
        let mut phi = arc.start_angle();
        let mut prev_point = arc.start;
        let arc_radius = arc.arc_radius();
        while phi < terminate_angle && phi >= terminate_angle - delta {
            if phi > consts::PI * 2.0 {
                phi = phi - consts::PI * 2.0;
            } else { phi += delta; }
            let x = arc.start.x + arc_radius * phi.cos();
            let y = arc.start.y + arc_radius * phi.sin();
            let next_point = Point::new(x, y);
            let line = Line::new(prev_point, next_point);
            lines.push(line);
            prev_point = next_point;
        }
        arc.lines = lines;
        arc

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
        // Arc angle.
        let x1 = self.start.x;
        let x2 = self.terminate.x;
        let y1 = self.start.y;
        let y2 = self.terminate.y;
        ((x1 * x2 + y1 * y2) / ((x1 * x1 + y1 * y1).powf(0.5) * (x2 * x2 + y2 * y2).powf(0.5))).acos()
        
    }

    fn start_angle(&self) -> f64 {
        // Arc start angle.
        let x1 = self.center_point.x + self.arc_radius();
        let x2 = self.start.x;
        let y1 = 0.0;
        let y2 = self.start.y;
        define_vector_angle(x1, x2, y1, y2)
    }

    fn terminate_angle(&self) -> f64 {
        // Arc terminate  angle.
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
        for line in self.lines.iter() {
            res += line.crossed(&ray) as u32
        }
        res > 0 && res % 2 == 1
    }
}