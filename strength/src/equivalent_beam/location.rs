use crate::equivalent_beam::system_of_units::Meters;

pub struct Location {
    x: Meters,
    y: Meters,
    z: Meters,
}

impl Location {
    fn new(x: Meters, y: Meters, z: Meters) -> Self {
        Location { x, y, z }
    }

    fn x(&self) -> &Meters {
        &self.x
    }

    fn y(&self) -> &Meters {
        &self.y
    }

    fn z(&self) -> &Meters {
        &self.z
    }


}

