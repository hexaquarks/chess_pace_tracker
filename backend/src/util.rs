pub fn compute_average(times: &[i32]) -> i32 {
    times.iter().sum::<i32>() / times.len() as i32
}

pub fn convert_centiseconds_to_seconds(time: i32) -> f32 {
    (time as f32 / 100 as f32) as f32
}
