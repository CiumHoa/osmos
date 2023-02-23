use osmos_core::cell;

pub struct Object{
    pub cell: cell::Cell,
}

impl Object {
    pub fn new(rng: &mut rand::rngs::ThreadRng)->Self {
        Self { cell: cell::Cell::random(rng)}
    }
}


