pub struct Simulator {
    pub rng: rand::rngs::ThreadRng,
    pub object_list: Vec<crate::object::Object>,
}

impl Default for Simulator {
    fn default() -> Self {
        let object_count = 500;
        let mut rng = rand::thread_rng();
        let object_list = (0..object_count)
            .map(|_| crate::object::Object::new(&mut rng))
            .collect();
        Self {
            rng: rng,
            object_list: object_list,
        }
    }
}

#[cfg(test)]
mod tests {
    mod default {
        #[test]
        fn test() {
            let sim = crate::simulator::Simulator::default();
            assert!(sim.object_list.len() == 500);
        }
    }
}
