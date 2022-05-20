/*
 * ownership is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/TamrielNetwork/ownership/blob/main/LICENSE
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
