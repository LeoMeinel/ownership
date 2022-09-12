/*
 * File: string_slices.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub(crate) fn string_slices() {
    slice_bad();
    slice();
}

fn slice() {
    let mut s = String::from("hello world");
    let s2 = "hello world";
    let _hello = &s[0..5]; // [..5] -> begin to 5(excluded)
    let _world = &s[6..11]; // [6..] -> 6 to end []; [..] -> entire string
    let word = first_word(s2);
    s.clear(); // Empty String
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

fn slice_bad() {
    let mut s = String::from("hello world");
    let word = first_word_bad(&s);
    s.clear(); // Empty String
    println!("{}", word);
}

#[allow(clippy::ptr_arg)]
fn first_word_bad(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
