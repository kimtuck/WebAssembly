#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "./index")]
extern {
     fn update(results: &Vec<Counter>);
}

#[derive(Debug)]
pub struct Counter {
    key: char,
    count: i32
}
impl Counter {
        pub fn new(key: char, count: i32) -> Counter {
            Counter { key: key, count: count }
        }
}

fn letters() -> Vec<Counter> {
    let s = "abcdefghijklmnopqrstuvwxyz";
    let mut vec: Vec<Counter> = Vec::new();
    for (i, c) in s.chars().enumerate() {
        vec.push(Counter::new(c,0));
    }
    vec
}

fn combine(a: &Vec<Counter>, b: &Vec<Counter>) -> Vec<Counter> {
    let zipiter = a.iter().zip(b.iter());
    zipiter.map(|(aelem, belem)| Counter::new(aelem.key, aelem.count + belem.count))
    .collect()
}

fn wordReducer(wordCounts: Vec<Counter>, word: &str) -> Vec<Counter> {
    let mut letterCounts = letters();
    for (i, c) in word.chars().enumerate() {
        if let Some(pos) = letterCounts.iter().position(|ref x| x.key == c) {
            let index = pos;
            if let Some(elem) = letterCounts.get_mut(index) {
                (*elem).count = 1;
            }
        }
    }
    /*
    println!("------");
    println!("After wordReducer: {}", word);
    println!("wordCounts {:?}", wordCounts);
    println!("letterCounts {:?}", letterCounts);
    println!("combined {:?}", combine(&wordCounts, &letterCounts));
    println!("");
    */
    combine(&wordCounts, &letterCounts)
}

/* a method we can call from javascript */
#[wasm_bindgen]
pub fn countLettersInWords(str: &str) -> Vec<Counter>{
    let lcase_str = str.to_lowercase();
   let words: Vec<&str> = lcase_str
    .split(" ")
    .collect();
   let wordLetterCounts = letters();
   let counts = words.iter().fold(wordLetterCounts, |accum, word| wordReducer(accum, word));
   counts
}




#[test]
fn counter_new() {
    let c = Counter::new('a',3);
    assert_eq!('a', c.key);
}

#[test]
fn counter_new_with_addition() {
    let a = Counter::new('a',2);
    let b = Counter::new('a',4);
    let c = Counter::new('a', a.count + b.count);
    assert_eq!('a', c.key);
}

#[test]
fn letters_test() {
    let l = letters();
    assert_eq!('a', l[0].key);
    assert_eq!(0, l[0].count);
    assert_eq!(26, l.len());
}

#[test]
fn combine_test() {
    let mut a = letters();
    let mut b = letters();
    a[0] = Counter::new('a',12);
    a[1] = Counter::new('b',4);
    b[1] = Counter::new('b',3);
    let c = combine(&a,&b);
    assert_eq!(12, c[0].count);
    assert_eq!(7, c[1].count)
}

#[test]
fn wordreducer_test() {
    let wordCounts = letters();
    let v = wordReducer(wordCounts, "abc");
    assert_eq!(1, v[0].count);
}
#[test]
fn wordreducer_test2() {
    let wordCounts = letters();
    let v = wordReducer(wordCounts, "a");
    let v2 = wordReducer(v,"a");
    assert_eq!(2, v2[0].count);
}

#[test]
fn countLettersInWords_test() {
    let count = countLettersInWords("a");
    assert_eq!('a', count[0].key);
    assert_eq!(1, count[0].count);
}
#[test]
fn countLettersInWords_test2() {
    let count = countLettersInWords("a a");
    assert_eq!('a', count[0].key);
    assert_eq!(2, count[0].count);
}
#[test]
fn countLettersInWords_test3() {
    let count = countLettersInWords("ab ab");
    assert_eq!('a', count[0].key);
    assert_eq!(2, count[0].count);
}

#[test]
fn countLettersInWords_test_mixed_upper_and_lower() {
    let count = countLettersInWords("a A a A");
    assert_eq!('a', count[0].key);
    assert_eq!(4, count[0].count);
}