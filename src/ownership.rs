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
