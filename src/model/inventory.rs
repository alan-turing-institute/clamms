use super::environment::Resource;

pub trait Inventory {
    fn count(&self, resource: Resource) -> i32;
}
