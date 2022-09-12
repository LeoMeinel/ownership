/*
 * File: slices.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub(crate) fn slices() {
    let a = [1, 2, 3, 4, 5, 6];
    let slice = &a[0..2];
    println!("{}, {}", slice[0], slice[1])
}
