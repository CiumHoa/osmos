pub struct Cell {
    pub position: nalgebra::Point2<f64>,
    pub acceleration: nalgebra::Vector2<f64>,
    pub velocity: nalgebra::Vector2<f64>,
    pub energy: usize,
    pub sensor: crate::sensor::Sensor,
}

impl Cell {
    pub fn random(rng: &mut rand::rngs::ThreadRng) -> Self {
        let random_x = rand::Rng::gen_range(rng, 0.0..=1.0);
        let random_y = rand::Rng::gen_range(rng, 0.0..=1.0);
        Self {
            position: nalgebra::Point2::new(random_x, random_y),
            acceleration: nalgebra::Vector2::new(0.0, 0.0),
            velocity: nalgebra::Vector2::new(0.0, 0.0),
            energy: rand::Rng::gen_range(rng, 1..=2),
            sensor:crate::sensor::Sensor::new(0.5),
        }
    }
    pub fn get_max_velocity_magnitude(&self) -> f64 {
        0.001 + (1.0 / self.energy as f64) * 0.0005
    }
}

#[cfg(test)]
mod tests {
    mod random {
        #[test]
        fn test() {
            let mut rng = rand::thread_rng();
            let cell_list = (0..100)
                .map(|_| crate::cell::Cell::random(&mut rng))
                .collect::<Vec<crate::cell::Cell>>();
            for cell in cell_list {
                assert!(cell.energy <= 2);
                assert!(cell.energy >= 1);
                assert!(cell.position.x >= 0.0);
                assert!(cell.position.y >= 0.0);
                assert!(cell.position.x <= 1.0);
                assert!(cell.position.y <= 1.0);
            }
        }
    }
}
