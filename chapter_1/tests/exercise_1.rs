pub fn global_sum_initialize(num_values: usize, num_cores: usize, core: usize) -> (usize, usize) {
    let block_size = num_values / num_cores;

    if (num_values % num_cores) == 0 {
        // n is evenly divisible by p

        ((core * block_size), ((core + 1) * block_size))
    } else {
        // n is not evenly divisible by p

        if core < (num_values % num_cores) {
            ((core * (block_size + 1)), ((core + 1) * (block_size + 1)))
        } else {
            ((core * block_size), ((core + 1) * block_size))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_1_global_sum_num_values_evenly_divisible_by_num_cores() {
        let values = [
            1, 4, 3, 9, 2, 8, 5, 1, 1, 6, 2, 7, 2, 5, 0, 4, 1, 8, 6, 5, 1, 2, 3, 9,
        ];
        let num_cores = 8;

        let core = 5;

        let (my_first_i, my_last_i) = global_sum_initialize(values.len(), num_cores, core);

        assert_eq!(my_first_i, 15);
        assert_eq!(my_last_i, 18);
    }

    #[test]
    fn exercise_1_global_sum_num_values_not_evenly_divisible_by_num_cores() {
        let values = [
            1, 4, 3, 9, 2, 8, 5, 1, 1, 6, 2, 7, 2, 5, 0, 4, 1, 8, 6, 5, 1, 2, 3, 9,
        ];
        let num_cores = 7;

        let core = 0;

        let (my_first_i, my_last_i) = global_sum_initialize(values.len(), num_cores, core);

        assert_eq!(my_first_i, 0);
        assert_eq!(my_last_i, 4);
    }
}
