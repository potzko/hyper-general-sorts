use rand::{self, Rng};
pub fn rnd_arr(len: usize, range: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut ret = Vec::<i32>::with_capacity(len);
    for _ in 0..len {
        ret.push(rng.gen_range(0..range));
    }
    ret
}

pub fn is_sorted<T: Ord>(arr: &Vec<T>) -> usize {
    for ind in 1..arr.len() {
        if arr[ind] < arr[ind - 1] {
            return ind;
        }
    }
    arr.len()
}
