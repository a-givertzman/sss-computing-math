use super::line::Ray;

pub trait Entity {
    fn crossed(&self, ray: &Ray) -> bool;

}