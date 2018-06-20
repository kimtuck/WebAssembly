#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

//---------------------------------------------
// DOM
//
#[wasm_bindgen]
extern {
    type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;
    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;
    #[wasm_bindgen(method, js_name = getElementById)]
    fn get_element_by_id(this: &HTMLDocument, id: &str) -> Element;

    type Element;
    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner_html(this: &Element, html: &str);
    #[wasm_bindgen(method, js_name = appendChild)]
    fn append_child(this: &Element, other: Element);

}


//------------------------------------------------------------------------


#[derive(Debug)]
pub struct Counter {
    counts: HashMap<char, i32>
}

impl Counter {
    pub fn new() -> Counter {
        let mut hash = HashMap::new();
        "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .for_each(|x| { hash.insert(x,0); });

        Counter { counts: hash }
    }
}

fn unique(word: &str) -> HashSet<char> {
    let mut chars: HashSet<char> = word.chars().collect();
    chars
}

fn wordReducer(letterCounts: &mut Counter, word: &str) {
    let lettersInWord = unique(word);
    //log(format!("{:?}",lettersInWord));

    lettersInWord.iter().for_each(|x| {
        let entry = letterCounts.counts.entry(*x).or_insert(0);
        *entry += 1;
    });
}

fn count_letters_in_words_impl(str: &str) -> Counter {
   let wordLetterCounts = Counter::new();
   let counts2 = str.split_whitespace();
   let counts3 = counts2.fold(wordLetterCounts, |mut accum, word| { wordReducer(&mut accum, word); accum });
   counts3
}

// Public method: Creates DOM nodes with output
#[wasm_bindgen]
pub fn count_letters_in_words(str: &str) {
   let wordCounts: Counter = count_letters_in_words_impl(str);
   display(wordCounts);

}

fn display(wordCounts: Counter) {
   let ul = document.createElement("ul");
   for (key, val) in wordCounts.counts.iter() {
        let li = document.createElement("li");
        let label=document.createElement("span");
        let value=document.createElement("span");
        label.set_inner_html(key.to_string().as_str());
        value.set_inner_html(val.to_string().as_str());
        li.append_child(label);
        li.append_child(value);
        ul.append_child(li);
   };
   let root = document.get_element_by_id("results-wasm");
   root.append_child(ul);
}

fn log(str: String) {
   let root = document.get_element_by_id("results-wasm");
   let value=document.createElement("div");
   value.set_inner_html(str.as_str());
   root.append_child(value);
}


#[test]
fn unique_test(){
    let s= "cbaabcdefa";
    let mut expected :HashSet<char> = "abcdef".chars().collect();  // ==> ["a", "b", "c", "d", "e", "f"]
    assert_eq!(expected,unique(s));
}

#[test]
fn counter_new() {
    let c = Counter::new();
    assert_eq!(0, c.counts[&'a']);
}


#[test]
fn wordreducer_test() {
    let mut wordCounts = Counter::new();
    wordReducer(&mut wordCounts, "abc");
    assert_eq!(1, wordCounts.counts[&'a']);
}

#[test]
fn wordreducer_test2() {
    let mut wordCounts = Counter::new();
    wordReducer(&mut wordCounts, "a");
    wordReducer(&mut wordCounts,"a");
    assert_eq!(2, wordCounts.counts[&'a']);
}

#[test]
fn countLettersInWords_test() {
    let count = count_letters_in_words_impl("a");
    assert_eq!(1, count.counts[&'a']);
}

#[test]
fn countLettersInWords_test2() {
    let count = count_letters_in_words_impl("a a");
    assert_eq!(2, count.counts[&'a']);
}

#[test]
fn countLettersInWords_test3() {
    let count = count_letters_in_words_impl("ab ab");
    assert_eq!(2, count.counts[&'a']);
    assert_eq!(2, count.counts[&'b']);
}

