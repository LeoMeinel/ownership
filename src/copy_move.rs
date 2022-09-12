/*
 * File: copy_move.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub(crate) fn copy_move() {
    let x = 5;
    let y = x; // Copy
    println!("{}", x);
    println!("{}", y);
    let s1 = String::from("hello");
    // Move
    let s2 = s1;
    // Copy
    let s3 = s2.clone();
    // ERROR: Borrow of moved value
    //println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    gives_copy();
    gives_ownership();
    gives_and_takes_back_ownership();
}

fn gives_copy() {
    let x = 5;
    makes_copy(x);
    println!("{}", x); // Copy
}

fn gives_ownership() {
    let s = String::from("hello");
    takes_ownership(s);
    // ERROR: Borrow of moved value
    //println!("{}", s);
}

fn gives_and_takes_back_ownership() {
    let s = takes_and_gives_back(String::from("hello"));
    println!("{}", s); // Takes and returns
}

fn takes_ownership(string: String) {
    println!("{}", string);
}

fn makes_copy(integer: i32) {
    println!("{}", integer);
}

fn takes_and_gives_back(string: String) -> String {
    println!("{}", string);
    string
}
