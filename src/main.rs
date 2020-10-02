extern crate rand;

use std::collections::HashSet;

fn hash(size:u32) -> Box<dyn Fn(&str) -> u64> {
    let r: f32 = rand::random::<f32>();
    let seed: f32 = f32::floor(r * size as f32) + 32.0;
    return Box::new(move |str: &str| -> u64 {
        str.as_bytes()
            .into_iter()
            .fold(0, |acc, &ch| (seed as u64 * acc + ch as u64) & 0xFFFFFFFF) })
}

fn fun_count(error: f32) -> u32 {
    f32::round(1.0 / (error * error)) as u32
}

fn build_funcs(num:u32) -> Vec<Box<dyn Fn(&str) -> u64>> {
    (0 .. num).fold(Vec::<Box<dyn Fn(&str) -> u64>>::new(), |mut acc, _| {
        acc.push(hash(num));
        acc
    })
}

fn find_min(set:&HashSet<&str>, func: &Box<dyn Fn(&str) -> u64>) -> u64 {
    set.into_iter()
        .fold(u64::MAX, |cur:u64, el| {u64::min(cur, func(el))})
}

fn signature(set:&HashSet<&str>, funcs: &Vec<Box<dyn Fn(&str) -> u64>>) -> Vec<u64> {
    funcs.into_iter().fold(Vec::<u64>::new(), |mut acc, f| {
        acc.push(find_min(set, f));
        acc
    })
}

fn similarity(sig_a:&Vec<u64>, sig_b: &Vec<u64>, func_num: u32) -> f32 {
    let equal_count: u32 = (0 .. func_num).map(|x| { if sig_a[x as usize] == sig_b[x as usize] { 1 } else { 0 }}).sum();
    return equal_count as f32 / func_num as f32;
}

fn hashmin(max_error: f32, set_a:&HashSet<&str>, set_b:&HashSet<&str>) -> f32 {
    let fun_num = fun_count(max_error);
    let funcs = build_funcs(fun_num);
    let sig_a = signature(set_a, &funcs);
    let sig_b = signature(set_b, &funcs);
    similarity(&sig_a, &sig_b, fun_num)
}

fn main() {
    println!("Hello, world!");
    let mut set_a = HashSet::new();
    set_a.insert("apple");
    set_a.insert("orange");

    let mut set_b = HashSet::new();
    set_b.insert("apple");
    set_b.insert("peach");

    let score = hashmin(0.01, &set_a, &set_b);
    println!("{}", score)
}
