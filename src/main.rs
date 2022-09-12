/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use crate::copy_move::copy_move;
use crate::ownership::ownership;
use crate::references::references;
use crate::slices::slices;
use crate::stack_heap::stack_heap;
use crate::string_slices::string_slices;

mod copy_move;
mod ownership;
mod references;
mod slices;
mod stack_heap;
mod string_slices;

fn main() {
    println!("Hello, world!");
    copy_move();
    ownership();
    references();
    stack_heap();
    string_slices();
    slices();
}
