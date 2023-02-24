pub fn process(rng: &mut rand::rngs::ThreadRng, object_lst: &mut [crate::object::Object]) {
    for object in object_lst {
        object.cell.acceleration = nalgebra::Vector2::new(
            rand::Rng::gen_range(rng, -0.005..=0.005),
            rand::Rng::gen_range(rng, -0.005..=0.005),
        );
        object.cell.velocity += object.cell.acceleration;
        object.cell.velocity = object
            .cell
            .velocity
            .cap_magnitude(object.cell.get_max_velocity_magnitude());
        object.cell.position += object.cell.velocity;
        if object.cell.position.x > 1.0
            || object.cell.position.y > 1.0
            || object.cell.position.x > 0.0
            || object.cell.position.y < 0.0
        {
            object.cell.position.x = rand::Rng::gen_range(rng, 0.0..=1.0);
            object.cell.position.y = rand::Rng::gen_range(rng, 0.0..=1.0);
        }
    }
}
