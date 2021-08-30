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

//=============================================================================
// TestDirUtils
//-----------------------------------------------------------------------------
#[test]
fn test_testdirutils_new() {
    // Create normal
    let mut test_dir = TestDirUtils::new("test_testdirutils_new").unwrap();
    test_dir.set_delete_on_terminate(false);
    let curr_path = OsString::from(test_dir.test_dir());
    drop(test_dir);
    assert!(Path::new(&curr_path).exists());
    // Create normal
    let test_dir = TestDirUtils::new("test_testdirutils_new").unwrap();
    let contents = b"this is just a test!";
    test_dir.create_test_file("test", contents).unwrap();
    let actual = test_dir.read_test_file("test").unwrap();
    assert_eq!(actual.as_slice(), contents);
    test_dir.delete_test_file("test").unwrap();
    test_dir.delete_test_file("test").unwrap();
    let test_file = test_dir.get_test_file_path("test");
    assert!(!Path::new(&test_file).exists());
    drop(test_dir);
    assert!(!Path::new(&curr_path).exists());
}
