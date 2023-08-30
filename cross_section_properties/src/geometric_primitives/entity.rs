use super::line::Ray;

pub trait Entity {
    fn crossed(&self, line: &Ray) -> bool;

}