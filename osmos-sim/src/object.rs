
pub struct Object{
    pub cell: osmos_core::cell::Cell
}
impl Object {
    pub fn new(rng:&mut rand::rngs::ThreadRng) -> Object {
        Object {
            cell: osmos_core::cell::Cell::random(rng)
        }
    }
}
