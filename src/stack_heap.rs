/*
 * File: stack_heap.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub(crate) fn stack_heap() {
    // Stack
    let _x = "hello";
    let _y = 22;
    // Heap
    let _x = String::from("world");
}
