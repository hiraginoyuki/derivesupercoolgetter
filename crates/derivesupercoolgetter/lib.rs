pub trait SuperCoolGetter<'a> {
    type FieldRef: 'a;
    type FieldMut: 'a;

    fn get(&'a self, field: &str) -> Option<Self::FieldRef>;
    fn get_mut(&'a mut self, field: &str) -> Option<Self::FieldMut>;
    fn get_as<T: 'static>(&'a self, field: &str) -> Option<&'a T>;
    fn get_mut_as<T: 'static>(&'a mut self, field: &str) -> Option<&'a mut T>;
}

pub use derivesupercoolgetter_derive::SuperCoolGetter;
