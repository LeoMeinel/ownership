/*
 * File: ownership.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 *	---- Ownership rules ----
 *	1. Each value in Rust has a variable that is called its owner.
 *	2. There can only be one owner at a time.
 *	3. When the owner goes out of scope, the value will be dropped.
 */

pub(crate) fn ownership() {
    // variables are only valid inside this scope {}
    let _a = "hello"; // Stack
    let _b = String::from("hello"); // Heap
}
// variables are not valid anymore
