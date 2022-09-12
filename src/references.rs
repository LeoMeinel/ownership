/*
 * File: references.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub(crate) fn references() {
    //let _reference_to_nothing = dangle();
    mix_mutable_immutable();
    call_calculate_length();
    call_calculate_length_mutable();
    call_calculate_length_bad();
}
// ERROR: Scope of variable is already over and variable can't be referenced (dangling reference)
//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}

fn mix_mutable_immutable() {
    let mut s1 = String::from("hello");
    let a1 = &s1;
    let a2 = &s1;
    // ERROR: Cannot borrow as immutable and mutable
    //let a3 = &mut s1;
    //println!("{}, {}, {}", a1, a2, a3);
    println!("{}, {}", a1, a2);
    let a3 = &mut s1; // Scope of immutables has ended so it can be borrowed as mutable now
    println!("{}", a3);
}

fn call_calculate_length() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len);
}

fn calculate_length(string: &str) -> usize {
    string.len()
}

fn call_calculate_length_mutable() {
    let mut s1 = String::from("hello");
    let len = calculate_length_mutable(&mut s1);
    println!("The length of {} is {}.", s1, len);
    let a1 = &mut s1;
    // ERROR: Cannot borrow as mutable more than once!
    //let a2 = &mut s1;
    //println!("{}, {}", a1, a2);
    println!("{}", a1);
}

fn calculate_length_mutable(string: &mut String) -> usize {
    string.push_str(", world");
    string.len()
}

fn call_calculate_length_bad() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length_bad(s1);
    println!("The length of {} is {}.", s2, len);
}

fn calculate_length_bad(string: String) -> (String, usize) {
    let len = string.len();
    (string, len)
}
