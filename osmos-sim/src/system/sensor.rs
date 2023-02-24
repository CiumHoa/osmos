pub fn process(object_lst: &mut [crate::object::Object]) {
    for current_object_index in 0..object_lst.len() {
        let other_object_index_list = object_lst
            .iter()
            .enumerate()
            .map(|(index, _)| index)
            .filter(|&index| index != current_object_index)
            .collect::<Vec<usize>>();
        let in_sensor_range_other_object_list = other_object_index_list
            .iter()
            .copied()
            .filter(|&other_object_index| {
                let distance = nalgebra::distance(
                    &object_lst[current_object_index].cell.position,
                    &object_lst[other_object_index].cell.position,
                );
                distance <= object_lst[current_object_index].cell.sensor.range
            })
            .collect::<Vec<usize>>();
    }
}
