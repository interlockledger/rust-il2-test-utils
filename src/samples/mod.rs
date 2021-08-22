/*
 * BSD 3-Clause License
 *
 * Copyright (c) 2019-2020, InterlockLedger Network
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * * Redistributions of source code must retain the above copyright notice, this
 *   list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright notice,
 *   this list of conditions and the following disclaimer in the documentation
 *   and/or other materials provided with the distribution.
 *
 * * Neither the name of the copyright holder nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
//! This module contains functions that can be used to create sample values
//! to be used in tests.
use std::ops::AddAssign;

#[cfg(test)]
mod tests;

/// Fills the mutable slice with the specified value.
///
/// Arguments:
/// - `target`: The slice to be filled;
/// - `value`: The value;
pub fn fill_with_value<T: Copy>(target: &mut [T], value: T) {
    for t in target {
        *t = value;
    }
}

/// Fills the mutable slice with a sequence of values.
///
/// Arguments:
/// - `target`: The slice to be filled;
/// - `initial`: The initial value;
/// - `inc`: The increment;
pub fn fill_with_seq<T: AddAssign + Copy>(target: &mut [T], initial: T, inc: T) {
    let mut curr = initial;
    for t in target {
        *t = curr;
        curr += inc;
    }
}

/// Fills the mutable slice with a sequence of values generated by a generetor
/// function.
///
/// This function receives the current value and must return the next value of the
/// sequence.
///
/// ```
/// let mut v: [u32; 6] = [0; 6];
/// // Using a generator based on the Collatz conjecture.
/// fill_with_seq_gen(&mut v, 5, |v| if v % 2 == 0 { v / 2 } else { 3 * v + 1 });
/// let exp: [u32; 6] = [5, 16, 8, 4, 2, 1];
/// assert_eq!(&v, &exp);
/// ```
///
/// Arguments:
/// - `target`: The slice to be filled;
/// - `initial`: The initial value;
/// - `gen`: The function used to generate the next state;
pub fn fill_with_seq_gen<T: Copy>(target: &mut [T], initial: T, gen: fn(T) -> T) {
    let mut curr = initial;
    for t in target {
        *t = curr;
        curr = gen(curr);
    }
}

/// Fills the mutable slice with a sequence of values generated by a generetor.
/// This function extracts the next value form the generator.
///
/// ```
/// struct Gen(u32, u32);
/// let mut v: [u32; 6] = [0; 6];
/// let mut g: Gen = Gen { 0: 0, 1: 1 };
///
/// // Using Fibonacci generator
/// fill_with_generator(&mut v, &mut g, |g| {
///     let r = g.0;
///     g.0 = g.1;
///     g.1 += r;
///     r
/// });
/// let exp: [u32; 6] = [0, 1, 1, 2, 3, 5];
/// assert_eq!(&v, &exp);
/// ```
///
/// Arguments:
/// - `target`: The slice to be filled;
/// - `generator`: The generator;
/// - `next`: The function used to extract the next value from the generator;
pub fn fill_with_generator<T: Copy, G>(target: &mut [T], generator: &mut G, next: fn(&mut G) -> T) {
    for t in target {
        *t = next(generator);
    }
}
