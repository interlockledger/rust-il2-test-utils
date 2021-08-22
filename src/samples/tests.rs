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
use super::*;

#[test]
fn test_fill_with_value() {
    let mut v: [u32; 6] = [0; 6];
    fill_with_value(&mut v, 255);
    let exp: [u32; 6] = [255; 6];
    assert_eq!(&v, &exp);
}

#[test]
fn test_fill_with_seq() {
    let mut v: [u32; 6] = [0; 6];
    fill_with_seq(&mut v, 0, 3);
    let exp: [u32; 6] = [0, 3, 6, 9, 12, 15];
    assert_eq!(&v, &exp);
}

#[test]
fn test_fill_with_seq_gen() {
    let mut v: [u32; 6] = [0; 6];
    // Using a generator based on the Collatz conjecture.
    fill_with_seq_gen(&mut v, 5, |v| if v % 2 == 0 { v / 2 } else { 3 * v + 1 });
    let exp: [u32; 6] = [5, 16, 8, 4, 2, 1];
    assert_eq!(&v, &exp);
}

#[test]
fn test_fill_with_generator() {
    struct Gen(u32, u32);
    let mut v: [u32; 6] = [0; 6];
    let mut g: Gen = Gen { 0: 0, 1: 1 };

    // A Fibonacci Generator
    fill_with_generator(&mut v, &mut g, |g| {
        let r = g.0;
        g.0 = g.1;
        g.1 += r;
        r
    });
    let exp: [u32; 6] = [0, 1, 1, 2, 3, 5];
    assert_eq!(&v, &exp);
}
