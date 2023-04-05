use crate::object::Object;

pub struct Simulator {
    pub rng: rand::rngs::ThreadRng,
    pub object_list: Vec<Object>,

}
impl Default for Simulator {
    fn default() -> Self {
        let mut  rng = rand::thread_rng();
        let object_count = 500;
        let object_list = (0..object_count).map(|_| crate::object::Object::new(&mut rng))
            .collect::<Vec<Object>>();
        Self{
            rng,
            object_list,
        }
    }
}
#[cfg(test)]
mod tests{
    mod default{
        use super::super::*;

        #[test]
        fn test() {
            let mut sim = Simulator::default();
            assert!(sim.object_list.len() == 500);
        }
    }
}
