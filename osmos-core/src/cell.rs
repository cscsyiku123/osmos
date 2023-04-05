use rand::Rng;

pub struct Cell {
    pub position: nalgebra::Point2<f64>,
    pub acceleration: nalgebra::Vector2<f64>,
    pub velocity: nalgebra::Vector2<f64>,
    pub energy: usize,
}

impl Cell {
    pub fn random(rng: &mut rand::rngs::ThreadRng) -> Cell {
        Cell {
            position: nalgebra::Point2::new(rand::Rng::gen_range(rng, 0.0..=1.0),
                                            rand::Rng::gen_range(rng, 0.0..=1.0)),
            acceleration: nalgebra::Vector2::new(0.0, 0.0),
            velocity: nalgebra::Vector2::new(0.0, 0.0),
            energy: rand::Rng::gen_range(rng, 1..=2),
        }
    }
}

#[cfg(test)]
mod tests {
    mod random {
        use super::super::*;

        #[test]
        fn test_random() {
            let mut rng = rand::thread_rng();
            let cell = (0..100).map(|_| Cell::random(&mut rng)).collect::<Vec<Cell>>();
            for cell in cell {
                assert!(cell.position.x >= 0.0 && cell.position.x <= 1.0);
                assert!(cell.position.y >= 0.0 && cell.position.y <= 1.0);
                assert!(cell.energy >= 1 && cell.energy <= 2);
            }

        }
    }
}
