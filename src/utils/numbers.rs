use rand::Rng;

fn get_random_value_between(max: i32, min: i32) -> i32 {
    if max == 1 && min == 0 {
        return 0;
    }
    rand::thread_rng().gen_range(min, max - 1)
}

pub fn get_random_to(max: i32) -> i32 {
    get_random_value_between(max, 0)
}
