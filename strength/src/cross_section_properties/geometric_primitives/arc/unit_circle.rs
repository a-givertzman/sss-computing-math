use std::f64::consts;

///
/// 
struct UnitCircle {

    
}

impl UnitCircle {
    pub(crate) fn angle(vector: Line) -> f64 {
        // Determine the maximum rotation angle of the vector on the unit circle.
        let angle = ((x1 * x2 + y1 * y2) / ((x1 * x1 + y1 * y1).powf(0.5) * (x2 * x2 + y2 * y2).powf(0.5))).acos();
            if x2 < 0.0 && y2 < 0.0 || x2 > 0.0 && y2 < 0.0{
                2.0 * consts::PI - angle
            } else { angle }
    }
}
