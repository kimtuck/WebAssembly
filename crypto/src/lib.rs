#![feature(use_extern_macros, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
extern crate indexmap;

use wasm_bindgen::prelude::*;
use std::collections::HashSet;
use indexmap::map::IndexMap;

//---------------------------------------------
// DOM
//
#[wasm_bindgen]
extern {
    type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;
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
    counts: IndexMap<char, i32>
}

impl Counter {
    pub fn new() -> Counter {
        Counter { counts:  IndexMap::new() }
    }
}

fn unique_letters_in_word(word: &str) -> HashSet<char> {
    let chars: HashSet<char> = word.chars().collect();
    chars
}

fn word_reducer(word_letter_counts: &mut Counter, word: &str) {
    let unique_letters = unique_letters_in_word(word);

    unique_letters.iter().for_each(|ch| {
        *word_letter_counts.counts.entry(*ch).or_insert(0) += 1;
    });
}

fn count_letters_in_words_impl(str: &str) -> Counter {
   let mut word_letter_counts = Counter::new();
   for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
        word_letter_counts.counts.entry(ch).or_insert(0);
   }

   let words = str.split_whitespace();
   words.fold(word_letter_counts, |mut accum, word| { word_reducer(&mut accum, word); accum })
}

// Public method: Creates DOM nodes with output
#[wasm_bindgen]
pub fn letter_count_webassembly(str: &str) {
   let word_counts: Counter = count_letters_in_words_impl(str);
   //display(word_counts);

}

fn display(word_counts: Counter) {
   let ul = document.createElement("ul");
   for (key, val) in word_counts.counts.iter() {
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
fn word_reducer_test() {
    let mut word_counts = Counter::new();
    word_reducer(&mut word_counts, "abc");
    assert_eq!(1, word_counts.counts[&'a']);
}

#[test]
fn word_reducer_test2() {
    let mut word_counts = Counter::new();
    word_reducer(&mut word_counts, "a");
    word_reducer(&mut word_counts,"a");
    assert_eq!(2, word_counts.counts[&'a']);
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

