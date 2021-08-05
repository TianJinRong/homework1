/*
    CIS198 Homework 1
    Part 3: Ownership, move semantics, and lifetimes

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
#![allow(dead_code)]
#![allow(unused_variables)]

/*
    Problem 1: Swap ints

    Implement the function that swaps two integers, and write unit tests.

    The Rust borrow checker may help avoid some possible bugs.

    Then answer this question:
    Q: A common source of error in swap implementations is failing to work if
       the two references are the same. Why don't you need to worry about this
       case in Rust?

    (Try writing a unit test where they are both
    the same, i.e. swap_ints(&mut x, &mut x).)
    答：因为 rust 中一个引用只能有一个持有者，所以不会出现两个引用的情况
*/
pub fn swap_ints(x1: &mut i32, x2: &mut i32) {
    let temp: i32 = *x1;
    *x1 = *x2;
    *x2 = temp;
}
#[test]
fn test_swap_ints() {
    let mut x1: i32 = 10;
    let mut x2: i32 = 20;
    swap_ints(&mut x1, &mut x2);
    assert_eq!(20, x1);
    assert_eq!(10, x2);
}

/*
    Problem 2: String duplication
*/
// #[test]
// fn copy_string_test() {
//     let str1 = String::from("foo");
//     let str2 = str1;
//     assert_eq!(str1, str2);
// }
// This test doesn't work. Fix it by copying strings properly.
// Q1. What went wrong?
// str1 是 String 类型，它在赋值给 str2 之后所有权就 move 到了 str2，
// str1 就销毁掉了，所以不能再调用了，assert_eq 时就拿不到它了
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1.clone();
    assert_eq!(str1, str2);
}
// Q2. How come it works fine here?
// 这里 i1 是 i32 类型，为标量，所以会自动复制，不存在所有权 move 的情况，所以没有报错
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Now implement the following function that duplicates a string n times.
fn duplicate_string(s: &str, times: usize) -> Vec<String> {
    let mut strings = Vec::new();
    while strings.len() < times {
        strings.push(s.to_string());
    }
    strings
}
#[test]
fn test_duplicate_string() {
    assert_eq!(
        duplicate_string("test", 3), 
        vec![String::from("test"),String::from("test"), String::from("test")]
    );
}

/*
    Problem 3: String duplication continued

    These two don't work either. Fix by changing the type of "string" in the
    function copy_me ONLY, and by adjusting the parameter to "copy_me" where
    it's called.
*/

fn copy_me(string: & String) -> String {
    string.clone()
}

#[test]
fn copy_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(& str1));
}

// #[test]
// fn copy_me_test2() {
//     let str1 = String::from("foo");
//     let str2 = copy_me(str1 /* Change in here only*/);
//     assert_eq!(str1, str2);
// }

/*
    Problem 4: Lifetime specifiers

    For each of the following three functions, either implement it by adding
    lifetime specifiers, or explain why this is not possible.

    (It's not truly impossible -- we will see later on that advanced features
    such as "unsafe code" can be used to turn off Rust's safety and lifetime
    checks.)
*/
// fn new_ref_string() -> &String {
//     unimplemented!();
// }
// 我们不能创建一个字符串，随后又返回它的指针。
// 因为这个字符串在调用完这个方法后就被销毁了，它的指针就是悬垂指针。

fn new_ref_str() -> &'static str {
    ""
}
#[test]
fn test_new_ref_str() {
    assert_eq!("", new_ref_str());
}

// The same function from part2
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() { s1 }
    else { s2 }
}
#[test]
fn test_pick_lingest2() {
    assert_eq!("abc", pick_longest2("a", "abc"));
    assert_eq!("ab", pick_longest2("ab", "cd"));
}

/*
    Problem 5: Using functions with lifetimes

    Write two versions of a function which returns the longest string in a
    vector, using pick_longest2 as a helper function.

    If the vector is empty, return "".

    Q1. In pick_longest_in_v2, if you were to explicitly specify the lifetime
        of the input and output, what should it be?

    Q2. What are the pros and cons of v1 and v2?
*/

fn pick_longest_in_v1(v: Vec<String>) -> String {
    if v.len() == 0 { "".to_string() }
    else {
        let mut result = String::new();
        for v1 in v {
            result = pick_longest2(&result, &v1).to_string();
        }
        result
    }
}
#[test]
fn test_pick_longest_in_v1() {
    assert_eq!(String::from("abc"), pick_longest_in_v1(vec![String::from("a"), String::from("abc")]));
    assert_eq!(String::from("ab"), pick_longest_in_v1(vec![String::from("ab"), String::from("cd")]));
}

fn pick_longest_in_v2<'a>(v: Vec<&'a str>) -> &'a str {
    if v.len() == 0 { "" }
    else {
        let mut result = "";
        for v1 in v {
            result = pick_longest2(&result, &v1);
        }
        result
    }
}

#[test]
fn test_pick_longest_in_v2() {
    assert_eq!("abc", pick_longest_in_v2(vec!["a", "abc"]));
    assert_eq!("ab", pick_longest_in_v2(vec!["ab", "cd"]));
}

/*
    Problem 6: Move semantics

    Write three versions of a function that pads a vector with zeros.
    Fail if the vector is larger than the desired length.

    Use .clone() if necessary to make any additional unit tests compile.

    Which of these functions do you prefer? Which is the most efficient?
    v1: 传进来的 v 是 move 进来的，原值就不能用了
    v2: 传进来的 v 是不可变引用，原值可用，但是新值就得完全创建一个全新的序列。如果需要生成新值，这个最好。
    v3: 传进来的 v 是可变引用，直接修改原值。如果需要直接修改原值的，用这个最好。
*/

fn pad_with_zeros_v1(v: Vec<usize>, desired_len: usize) -> Vec<usize> {
    let mut result = v;
    for i in result.len()..desired_len {
        result.push(0);
    }
    debug_assert_eq!(result.len(), desired_len);
    result
}

fn pad_with_zeros_v2(slice: &[usize], desired_len: usize) -> Vec<usize> {
    let mut result = vec![0;desired_len];
    for i in 0..slice.len() {
        result[i] = slice[i];
    }
    debug_assert_eq!(result.len(), desired_len);
    result
}

fn pad_with_zeros_v3(v: &mut Vec<usize>, desired_len: usize) {
    for i in v.len()..desired_len {
        v.push(0);
    }
    debug_assert_eq!(v.len(), desired_len);
}

#[test]
fn test_pad_twice_v1() {
    let v = vec![1];
    let v = pad_with_zeros_v1(v, 2);
    let v = pad_with_zeros_v1(v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v2() {
    let v = vec![1];
    let v = pad_with_zeros_v2(&v, 2);
    let v = pad_with_zeros_v2(&v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v3() {
    let mut v = vec![1];
    pad_with_zeros_v3(&mut v, 2);
    pad_with_zeros_v3(&mut v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

/*
    Problem 7: Move semantics continued

    Write a function which appends a row to a vector of vectors.
    Notice that it takes ownership over the row.
    You shouldn't need to use .clone().

    Why is this more general than being passed a &[bool]
    and cloning it?
    因为一般的用法就往一个vector中追加一个值，而不需要再对这个值进行额外的操作
    所以不需要使用引用或者克隆一个新值

    Second, write a function which returns whether
    a row equals the first row in the vector of vectors.
    Notice that it does not take ownership over the row.

    Why is this more general than being passed a Vec<bool>?
    因为一般情况下只是需要检查一下是否为第一个，并不需要因为而改变
    原值
*/

fn append_row(grid: &mut Vec<Vec<bool>>, row: Vec<bool>) {
    grid.push(row);
}

#[test]
fn test_append_row() {
    let mut grid = vec![vec![true]];
    let row = vec![false, true];
    append_row(&mut grid, row);
    assert_eq!(grid, vec![vec![true], vec![false, true]]);
}

fn is_first_row(grid: &[Vec<bool>], row: &[bool]) -> bool {
    // Check if row is the first row in grid
    // Remember to handle the case when grid is empty
    if grid.len() == 0 {
        return false;
    }
    return row == grid[0];
}

#[test]
fn test_is_first_row() {
    let grid = vec![vec![false,false], vec![true]];
    let row_true = vec![false,false];
    let row_false = vec![true];
    let empty_grid = vec![];

    assert_eq!(is_first_row(&grid, &row_true), true);
    assert_eq!(is_first_row(&grid, &row_false), false);
    assert_eq!(is_first_row(&empty_grid, &row_true), false);
}

/*
    Problem 8: Modifying while iterating

    In C and C++, you run into subtle bugs if you try to modify a data
    structure while iterating over it. Rust's move semantics prevents that.
*/

use std::collections::HashMap;

// To familiarize yourself with HashMaps,
// implement the following function which converts pairs from a slice
// into key-value pairs in a hashmap.
// Documentation:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

fn vector_to_hashmap(v: &[(i32, String)]) -> HashMap<i32, String> {
    // 写法一
    // let mut result = HashMap::new();
    // for (key, value) in v {
    //     result.insert(key.clone(), value.clone());
    // }
    // result

    // 写法二
    let result: HashMap<_, _> = v.iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    result
}
#[test]
fn test_vector_to_hashmap() {
    let v = vec![(3,String::from("c")), (2,String::from("b"))];
    let mut h: HashMap<i32, String> = HashMap::new();
    h.insert(3, String::from("c"));
    h.insert(2, String::from("b"));
    assert_eq!(vector_to_hashmap(&v), h);
}

// Now rewrite this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    // This fails, uncomment to see error.
    // for k in &mut h.keys() {
    //     if *k < 0 {
    //         h.remove(k);
    //     }
    // }
    
    // 最优雅写法
    h.retain(|k, _v| *k >= 0 )
}
#[test]
fn test_delete_negative_keys() {
    let mut h = HashMap::new();
    h.insert(1, 100);
    h.insert(-2, 200);
    h.insert(3, 300);
    delete_negative_keys(&mut h);

    let mut expect = HashMap::new();
    expect.insert(1, 100);
    expect.insert(3, 300);

    assert_eq!(expect, h);
}

/*
    Problem 9: The Entry API

    Move semantics present interesting API design choices not found in other
    languages.
    HashMap is an example of such a API.
    Specifically, the Entry API:
    https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

    This allows for efficient HashMap access because we only access
    the entry in the map (computing an expensive hash function) once.

    Implement a function which does the following:
        For all entries in `add`: (k, v)
        If `k` exists in `merged`, append `v` to the value of `merged[k]`.
        If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.
    Use `or_insert` and `and_modify`.
*/

fn merge_maps(
    merged: &mut HashMap<String, String>,
    add: HashMap<String,String>
) {
    unimplemented!()
}
