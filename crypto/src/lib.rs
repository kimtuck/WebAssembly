#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

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


pub struct Counter {
    letter: char,
    count: i32,
}

impl Counter {
    pub fn new(key: char, count: i32) -> Counter {
        Counter { letter: key, count: count }
    }
}

fn str_to_first_char(str: &str) -> char {
    str.chars().next().unwrap()
}

fn unique(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars.dedup_by(|a,b| *a == *b);
    chars
}

fn letters() -> Vec<Counter> {
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|x| Counter::new(x,0)).collect()
}

fn wordReducer(letterCounts: &mut Vec<Counter>, word: &str) {
    let lettersInWord = unique(word);
    lettersInWord.iter().for_each(|x| {
        let ch = *x;
        //log(format!("find pos for {}",ch));
        if let Some(pos) = letterCounts.iter().position(|ref x| x.letter == ch) {
            let elem = &mut letterCounts[pos];
            elem.count += 1;
        }
        //else {
        //    log(format!("unable to find position in letterCounts for {}",ch));
        //}
    })
}

fn count_letters_in_words_impl(str: &str) -> Vec<Counter> {
   let wordLetterCounts = letters();
   let counts = str
   .to_lowercase();

   let counts2 = counts.split_whitespace();
   let counts3 = counts2.fold(wordLetterCounts, |mut accum, word| { wordReducer(&mut accum, word); accum });
   counts3
}

// Public method: Creates DOM nodes with output
#[wasm_bindgen]
pub fn count_letters_in_words(str: &str) {
   let wordCounts: Vec<Counter> = count_letters_in_words_impl(str);
   count_letters_in_words_impl(str);
   count_letters_in_words_impl(str);
   count_letters_in_words_impl(str);
   count_letters_in_words_impl(str);
   count_letters_in_words_impl(str);
   count_letters_in_words_impl(str);
   count_letters_in_words_impl(str);
   count_letters_in_words_impl(str);
   count_letters_in_words_impl(str);

   display(wordCounts);

}

fn display(wordCounts: Vec<Counter>) {
   let ul = wordCounts.iter().fold(document.createElement("ul"), |ul, x|
        { let li = document.createElement("li");
        let label=document.createElement("span");
        let value=document.createElement("span");
        label.set_inner_html(x.letter.to_string().as_str());
        value.set_inner_html(x.count.to_string().as_str());
        li.append_child(label);
        li.append_child(value);
        ul.append_child(li);
        //update(&x.key, x.count)
        ul
        }
   );
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
    let mut expected :Vec<char> = "abcdef".chars().collect();  // ==> ["a", "b", "c", "d", "e", "f"]
    assert_eq!(expected,unique(s));
}
#[test]
fn counter_new() {
    let c = Counter::new('a',3);
    assert_eq!('a', c.letter);
}

#[test]
fn counter_new_with_addition() {
    let a = Counter::new('a',2);
    let b = Counter::new('a',4);
    let c = Counter::new('a', a.count + b.count);
    assert_eq!('a', c.letter);
}

#[test]
fn letters_test() {
    let l = letters();
    assert_eq!('a', l[0].letter);
    assert_eq!(0, l[0].count);
    assert_eq!(26, l.len());
}

#[test]
fn wordreducer_test() {
    let mut wordCounts = letters();
    wordReducer(&mut wordCounts, "abc");
    assert_eq!(1, wordCounts[0].count);
}

#[test]
fn wordreducer_test2() {
    let mut wordCounts = letters();
    wordReducer(&mut wordCounts, "a");
    wordReducer(&mut wordCounts,"a");
    assert_eq!(2, wordCounts[0].count);
}

#[test]
fn countLettersInWords_test() {
    let count = count_letters_in_words_impl("a");
    assert_eq!('a', count[0].letter);
    assert_eq!(1, count[0].count);
}

#[test]
fn countLettersInWords_test2() {
    let count = count_letters_in_words_impl("a a");
    assert_eq!('a', count[0].letter);
    assert_eq!(2, count[0].count);
}

#[test]
fn countLettersInWords_test3() {
    let count = count_letters_in_words_impl("ab ab");
    assert_eq!('a', count[0].letter);
    assert_eq!(2, count[0].count);
}

#[test]
fn countLettersInWords_test_mixed_upper_and_lower() {
    let count = count_letters_in_words_impl("a A a A");
    assert_eq!('a', count[0].letter);
    assert_eq!(4, count[0].count);
}


