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
