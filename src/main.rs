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
