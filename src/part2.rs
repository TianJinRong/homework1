/*
    CIS198 Homework 1
    Part 2: Strings, files, and mutability

    Make the following failing functions/tests pass.
    Answer the questions as a comment next to the problems.
*/

// Remove these once you are done editing the file!
#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::Read;

/*
    Problem 1: Split variants

    Create functions split_ref and split_clone such that
    all the following tests will pass. Feel free to use Rust's split method
    (https://doc.rust-lang.org/std/primitive.slice.html#method.split)
    as needed.
*/

// split_ref must have the return type Vec<&str>
fn split_ref(input: &str) -> Vec<&str> {
    input.split(" ").collect()
}

#[test]
fn test_split_ref(){
    let string = "Hello World!".to_string();
    assert_eq!(split_ref(& string), ["Hello", "World!"]);
    assert_eq!(split_ref("Hello World!"), & ["Hello", "World!"]);
    assert_eq!(split_ref("Hello World!"), vec!["Hello", "World!"]);
}

// split_clone must have the return type Vec<String>
fn split_clone(input: &str) -> Vec<String> {
    input.split(" ")
        .map(|x| x.to_string())
        .collect()
}

#[test]
fn test_split_clone(){
    let string = "Hello World!".to_string();
    assert_eq!(split_clone(& string), ["Hello", "World!"]);
    assert_eq!(split_clone("Hello World!"), & ["Hello", "World!"]);
    assert_eq!(split_clone("Hello World!"), vec!["Hello", "World!"]);
}

/*
    Problem 2: Longest string

    Write function pick_longest which picks the longests of two &str arguments.
    Taking &str arguments makes it more general than taking Strings.
    Return a new String (we will see later how to return a &str.)
*/

fn pick_longest(x1: &str, x2: &str) -> String {
    if x1.len() >= x2.len() {
        x1.to_string()
    }
    else {
        x2.to_string()
    }
}

#[test]
fn test_pick_longest(){
    assert_eq!(
        pick_longest(& "cat".to_string(), & "dog".to_string()),
        "cat".to_string()
    );
}

// Question 1:
// For the curious, attempt to return reference, that is:
//
// fn pick_longest(s1: &str, s2: &str) -> &str
//
// What goes wrong when you try to implement this function? Why is this
// the case?

// 该方法实现如下所示
// fn pick_longest<'a>(x1: &'a str, x2: &'a str) -> &'a str {
//     if x1.len() >= x2.len() {
//         &(x1.to_string())
//     }
//     else {
//         &(x2.to_string())
//     }
// }
// 因为要返回的字符串是在方法内创建的，是一个临时变量，该字符串的生命周期只在本方法内。
// 在逻辑离开本方法时，该字符串就被销毁了。
// 它的指针也就销毁了。
// Because cannot return reference to temporary value.

/*
    Problem 3: File to string

    Write a function that returns all the contents of a file as a single String.

    DO NOT USE the assocated function std::fs::read_to_string

    Instead use File::open, and the method read_to_string
    (https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string)

    You can use .expect("ignoring error: ") to ignore the Result<...> type in open()
    and read_to_string. We will discuss error handling later.
*/
pub fn file_to_string(path: &str) -> String {
    let mut f = File::open(path).expect("ignoring error: ");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).expect("ignoring error: ");
    buffer
}
#[test]
fn test_file_to_string() {
    assert_eq!("Hello world!\r\nHello rust!", file_to_string("./src/part2_test.txt"));
}
#[test]
#[should_panic(expected = "ignoring error:")]
fn test_file_to_string_err() {
    file_to_string("part2_test");
}

/*
    Problem 4: Mutability

    Why does the following implementation not work as expected?
    Fix by changing the type signature of add1 and the way it's called on add1_test().
    do NOT change the return type.
*/

// 原方法为
// pub fn add1(mut x : i32) -> () {
//     x += 1;
// }
// 测试结果是x并没有+1成功。
// 原因是：x 到了方法中被复制了一个新的，+1的也是这个新的变量，而该方法结束后，x就被销毁了
// 所以，在调用完该方法后，再使用 x，还是调用的外边的 x，所以还是1

#[test]
fn test_add1() {
    let mut x: i32 = 1;
    add1(&mut x);
    assert_eq!(x, 2);
}

pub fn add1(x : &mut i32) -> () {
    *x += 1;
}

/*
    Problem 5: Mutability continued

    The error says: cannot assign to immutable borrowed content `*str1`
    But we declared it mutable? Fix by changing only the line below.
*/

pub fn mut2() {
    let hello = String::from("hello");

    // CHANGE ONLY THIS LINE:
    // let mut str1: &String = &String::from("str1");
    // 旧写法是定义了一个可变的指针，该指针指向的String是不可变的，所以没有什么用
    // 我们应该定义它指向一个可以变的String
    let str1: &mut String = &mut String::from("str1");

    *str1 = hello;
}
