use super::environment::Resource;

pub trait Inventory {
    fn count(&self, resource: &Resource) -> i32;
    fn acquire(&mut self, resource: &Resource, quantity: i32) -> ();
    fn consume(&mut self, resource: &Resource, quantity: u32) -> () {
        self.acquire(
            resource,
            -1 * <u32 as TryInto<i32>>::try_into(quantity).unwrap(),
        )
    }
}
