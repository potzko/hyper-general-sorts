use core::cmp::min;
use rand::{self, Rng};
use std::fmt::Debug;
mod arr_utils;

fn main() {
    heap_fibo(50000, 0);
    let mut rng = rand::thread_rng();
    let mut arr = vec![1, 2, 3];
    while arr_utils::is_sorted(&arr) == arr.len() {
        arr = arr_utils::rnd_arr(1000, 100000000);
        //println!("{:?}", arr);
        let a = random_sort::<Vec<i32>>(&mut rng);
        println!("{}", a.0);
        a.1(&mut arr);
    }
    println!("{}", arr_utils::is_sorted(&arr) == arr.len());

    /*
    let mut arr = arr_utils::rnd_arr(5000, 100000000);
    let a = random_quick_sort_build::<Vec<i32>>(&mut rng);
    a(&mut arr);
    println!("{}", arr_utils::is_sorted(&arr) == arr.len());
    let mut arr = arr_utils::rnd_arr(5000, 100000000);
    let a = random_shell_sort_build::<Vec<i32>>(&mut rng);
    a(&mut arr);
    println!("{}", arr_utils::is_sorted(&arr) == arr.len());*/
}
trait Sortable {
    fn cmp_index(&self, ind_a: usize, ind_v: usize) -> bool;
    fn exchange(&mut self, ind_a: usize, ind_b: usize);
    fn len(&self) -> usize;
    fn pp(&self);
}

impl<T: Ord + Debug> Sortable for Vec<T> {
    fn cmp_index(&self, ind_a: usize, ind_b: usize) -> bool {
        self[ind_a] <= self[ind_b]
    }
    fn exchange(&mut self, ind_a: usize, ind_b: usize) {
        self.swap(ind_a, ind_b)
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn pp(&self) {
        println!("{:?}", &self);
    }
}

impl<T: Sortable> Sortable for &mut T {
    fn cmp_index(&self, ind_a: usize, ind_b: usize) -> bool {
        (**self).cmp_index(ind_a, ind_b)
    }
    fn exchange(&mut self, ind_a: usize, ind_b: usize) {
        (**self).exchange(ind_a, ind_b)
    }
    fn len(&self) -> usize {
        (**self).len()
    }
    fn pp(&self) {
        (**self).pp();
    }
}

fn sequence_classic_log<T: Sortable>(arr: &mut T, iter: usize) -> usize {
    arr.len() / 2_usize.pow((iter + 1) as u32)
}
fn sequence_knuth<T: Sortable>(_arr: &mut T, iter: usize) -> usize {
    (3_usize.pow((iter + 1).try_into().unwrap()) - 1) / 2
}

fn sequence_lazarus<T: Sortable>(_arr: &mut T, iter: usize) -> usize {
    2_usize.pow((iter as u32) + 1) - 1
}

fn sequence_sedgewick<T: Sortable>(_arr: &mut T, iter: usize) -> usize {
    if iter == 0 {
        return 1;
    }
    4_usize.pow(iter as u32) + 3 * 2_usize.pow(iter as u32 - 1) + 1
}
fn sequence_sedgewick_branching<T: Sortable>(_arr: &mut T, iter: usize) -> usize {
    if iter % 2 == 1 {
        return 8 * 2_usize.pow(iter as u32) - 6 * 2_usize.pow((iter as u32 + 1) / 2) + 1;
    } else {
        9 * (2_usize.pow(iter as u32) - 2_usize.pow(iter as u32 / 2)) + 1
    }
}

fn random_shell_sort_build<T: Sortable>(rng: &mut rand::rngs::ThreadRng) -> impl Fn(&mut T) {
    let arr = [
        sequence_classic_log,
        sequence_lazarus,
        sequence_knuth,
        sequence_sedgewick,
        sequence_sedgewick_branching,
    ];
    let seq = arr[rng.gen_range(0..arr.len())];
    shell_sort_build::<T>(seq)
}

fn shell_sort_build<T: Sortable>(sequence: impl Fn(&mut T, usize) -> usize) -> impl Fn(&mut T) {
    move |arr: &mut T| shell_sort(arr, &sequence)
}

fn shell_sort<T: Sortable>(arr: &mut T, sequence: impl Clone + Fn(&mut T, usize) -> usize) {
    let mut jump: usize = sequence(arr, 0);
    let mut iter = 1;
    while jump < arr.len() && jump != 0 {
        for index in (0..arr.len()).step_by(jump) {
            let mut ind = index;
            while jump <= ind && arr.cmp_index(ind, ind - jump) {
                arr.exchange(ind, ind - jump);
                ind -= jump;
            }
        }
        jump = sequence(arr, iter);
        iter += 1;
    }
}

fn pivot_pick_start<T: Sortable>(start: usize, _end: usize, _arr: &mut T) -> usize {
    start
}

fn pivot_pick_end<T: Sortable>(_start: usize, end: usize, _arr: &mut T) -> usize {
    end - 1
}

fn pivot_pick_middle<T: Sortable>(start: usize, end: usize, _arr: &mut T) -> usize {
    (start + end) / 2
}

fn pivot_pick_5<T: Sortable>(start: usize, end: usize, _arr: &mut T) -> usize {
    if end - start <= 5 {
        return start;
    }
    start + 5
}

fn pivot_pick_sorted_middle<T: Sortable>(start: usize, end: usize, arr: &mut T) -> usize {
    if end - start < 3 {
        return start;
    }
    let middle = (start + end) / 2;
    if !arr.cmp_index(middle - 1, middle) {
        arr.exchange(middle - 1, middle)
    }
    if !arr.cmp_index(middle, middle + 1) {
        arr.exchange(middle, middle + 1)
    }
    if !arr.cmp_index(middle - 1, middle) {
        arr.exchange(middle - 1, middle)
    }
    middle
}

fn pivot_pick_sorted_middle_bad<T: Sortable>(start: usize, end: usize, arr: &mut T) -> usize {
    if end - start < 3 {
        return start;
    }
    let middle = (start + end) / 2;
    if !arr.cmp_index(middle - 1, middle) {
        arr.exchange(middle - 1, middle)
    }
    if !arr.cmp_index(middle, middle + 1) {
        arr.exchange(middle, middle + 1)
    }
    if !arr.cmp_index(middle - 1, middle) {
        arr.exchange(middle - 1, middle)
    }
    middle - 1
}

fn pivot_very_bad<T: Sortable>(start: usize, end: usize, arr: &mut T) -> usize {
    if end - start < 6 {
        return start;
    }
    let max = (start + end) / 2 - 3;
    for i in 0..6 {
        if arr.cmp_index(max, max + i) {
            arr.exchange(max, max + i)
        }
    }
    max
}

fn random_quick_sort_build<T: Sortable>(rng: &mut rand::rngs::ThreadRng) -> impl Fn(&mut T) {
    let arr = [
        pivot_pick_middle,
        pivot_pick_start,
        pivot_pick_end,
        pivot_pick_5,
        pivot_pick_sorted_middle,
        pivot_pick_sorted_middle_bad,
        pivot_very_bad,
    ];
    let piv = arr[rng.gen_range(0..arr.len())];
    return quick_sort_build(piv);
}

fn quick_sort_build<T: Sortable>(
    pivot_pick: impl Clone + Fn(usize, usize, &mut T) -> usize,
) -> impl Fn(&mut T) {
    move |arr: &mut T| quick_sort(arr, 0, arr.len(), &pivot_pick)
}

fn partition<T: Sortable>(arr: &mut T, start: usize, end: usize, mut pivot: usize) -> usize {
    let mut small: usize = start;
    let mut large: usize = end - 1;
    while small < large {
        while small < large && arr.cmp_index(small, pivot) {
            small += 1;
        }
        while !arr.cmp_index(large, pivot) {
            large -= 1;
        }
        /*if small == end {
            arr.exchange(pivot, end - 1);
            return end - 1;
        }*/
        if small < large {
            if large == pivot {
                pivot = small;
            }
            arr.exchange(small, large);
        }
    }
    arr.exchange(pivot, large);
    small
}

fn quick_sort<T: Sortable>(
    arr: &mut T,
    start: usize,
    end: usize,
    pivot_pick: impl Clone + Fn(usize, usize, &mut T) -> usize,
) {
    if end - start < 2 {
        return;
    }
    let pivot = pivot_pick(start, end, arr);
    let pivot_index = partition(arr, start, end, pivot);
    quick_sort(arr, start, pivot_index, pivot_pick.clone());
    quick_sort(arr, pivot_index, end, pivot_pick)
}

fn get_max_child<T: Sortable>(arr: &T, end: usize, range: (usize, usize)) -> Option<usize> {
    if range.0 >= end {
        return None;
    }
    let mut max = range.0;
    for ind in (range.0 + 1)..=min(end - 1, range.1) {
        if !arr.cmp_index(ind, max) {
            max = ind;
        }
    }
    Some(max)
}

fn heapify<T: Sortable>(
    arr: &mut T,
    ind: usize,
    end: usize,
    heap_type: impl Clone + Fn(usize, usize) -> (usize, usize),
) {
    let child_nodes = heap_type(ind, arr.len());
    //println!("{}, {:?}", ind, child_nodes);

    if let Some(max_child) = get_max_child(arr, end, child_nodes) {
        if arr.cmp_index(max_child, ind) {
            return;
        }
        arr.exchange(max_child, ind);
        heapify(arr, max_child, end, heap_type);
    }
}

fn deep_heapify<T: Sortable>(
    arr: &mut T,
    ind: usize,
    heap_type: impl Clone + Fn(usize, usize) -> (usize, usize),
) {
    if ind >= arr.len() {
        return;
    }
    let child_nodes = heap_type(ind, arr.len());
    for i in child_nodes.0..=child_nodes.1 {
        deep_heapify(arr, i, heap_type.clone())
    }
    heapify(arr, ind, arr.len(), heap_type)
}

fn tree_sort<T: Sortable>(arr: &mut T, heap_type: impl Clone + Fn(usize, usize) -> (usize, usize)) {
    deep_heapify(arr, 0, &heap_type);
    for i in (0..arr.len()).rev() {
        arr.exchange(0, i);
        heapify(arr, 0, i, &heap_type);
    }
}

fn heap_base_2(node: usize, _len: usize) -> (usize, usize) {
    return ((node << 1) + 1, (node << 1) + 2);
}

fn heap_base_2_reversed(node: usize, _len: usize) -> (usize, usize) {
    if node == 0 {
        return (1, 2);
    }
    if node % 2 == 1 {
        return ((node << 1) + 3, (node << 1) + 4);
    }
    return ((node << 1) - 1, (node << 1) - 0);
}

fn heap_base_3(node: usize, _len: usize) -> (usize, usize) {
    return (node * 3 + 1, node * 3 + 3);
}

fn heap_base_5(node: usize, _len: usize) -> (usize, usize) {
    return (node * 5 + 1, node * 5 + 5);
}

fn heap_base_7(node: usize, _len: usize) -> (usize, usize) {
    return (node * 7 + 1, node * 7 + 7);
}

static mut HEAPS: Vec<(usize, usize)> = Vec::new();
static mut FIBO_VALS: (usize, usize, usize) = (1, 2, 3);

fn heap_fibo(node: usize, _len: usize) -> (usize, usize) {
    unsafe {
        while HEAPS.len() <= node {
            for _ in 0..FIBO_VALS.1 {
                HEAPS.push((FIBO_VALS.0, FIBO_VALS.0 + FIBO_VALS.1 - 1));
                FIBO_VALS.0 += FIBO_VALS.1;
            }
            let tmp = FIBO_VALS.2;
            FIBO_VALS.2 = FIBO_VALS.2 + FIBO_VALS.1;
            FIBO_VALS.1 = tmp;
        }
        return HEAPS[node];
    }
}

fn random_heap_sort_build<T: Sortable>(rng: &mut rand::rngs::ThreadRng) -> impl Fn(&mut T) {
    let arr = [
        heap_base_2,
        heap_base_3,
        heap_base_5,
        heap_base_7,
        heap_base_2_reversed,
        heap_fibo,
    ];
    let base = arr[rng.gen_range(0..arr.len())];
    heap_sort_build::<T>(base)
}

fn heap_sort_build<T: Sortable>(
    heap_type: impl Clone + Fn(usize, usize) -> (usize, usize),
) -> impl Fn(&mut T) {
    move |arr: &mut T| tree_sort(arr, &heap_type)
}

fn random_sort<T: Sortable>(rng: &mut rand::rngs::ThreadRng) -> (&str, impl Fn(&mut T)) {
    let num = rng.gen_range(0..3);
    let q = random_quick_sort_build(rng);
    let s = random_shell_sort_build(rng);
    let h = random_heap_sort_build(rng);

    let ret = move |arr: &mut T| match num {
        0 => q(arr),
        1 => s(arr),
        _ => h(arr),
    };

    let sort_names = ["quick sort", "shell sort", "heap sort"];

    (sort_names[num], ret)
}
