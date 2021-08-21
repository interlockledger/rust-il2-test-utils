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
use std::fs::{remove_dir, remove_dir_all, remove_file, write};

//=============================================================================
// TestDirUtils
//-----------------------------------------------------------------------------
/// Returns a global lock to serialize the tests of this unit.
fn get_local_test_lock() -> MutexGuard<'static, usize> {
    static mut MUTEX: Option<Mutex<usize>> = None;
    static ONCE: Once = Once::new();
    unsafe {
        ONCE.call_once(|| {
            MUTEX.replace(Mutex::default());
        });
        MUTEX.as_ref().unwrap().lock().unwrap()
    }
}

#[test]
fn test_testdirutils_new() {
    let _test_lock = get_local_test_lock();
    // Cleanup
    let test_dir_path = Path::new(TestDirUtils::DEFAULT_TEST_DIR);
    if test_dir_path.exists() {
        if test_dir_path.is_dir() {
            remove_dir_all(test_dir_path).unwrap();
        } else {
            remove_file(test_dir_path).unwrap();
        }
    }

    // Create normal
    let test_dir = TestDirUtils::new();
    assert!(test_dir_path.is_dir());
    drop(test_dir);

    // Create with a file in the way
    remove_dir(test_dir_path).unwrap();
    write(test_dir_path, b"").unwrap();
    let test_dir = TestDirUtils::new();
    assert!(test_dir_path.is_dir());
    drop(test_dir);
}

#[cfg(not(target_os = "windows"))]
#[test]
#[should_panic(expected = "TestDirUtils said: It is not safe to run 'rm -Rf /', don't you agree?")]
fn test_testdirutils_with_path_safeguard() {
    let test_dir = TestDirUtils::with_path(Path::new("/"));
    drop(test_dir);
}

#[test]
fn test_testdirutils_with_path() {
    let _test_lock = get_local_test_lock();
    // Cleanup
    let test_dir_path = Path::new(TestDirUtils::DEFAULT_TEST_DIR);
    if test_dir_path.exists() {
        if test_dir_path.is_dir() {
            remove_dir_all(test_dir_path).unwrap();
        } else {
            remove_file(test_dir_path).unwrap();
        }
    }

    // Create normal
    let test_dir = TestDirUtils::new();
    assert!(test_dir_path.is_dir());
    drop(test_dir);

    // Create with a file in the way
    remove_dir(test_dir_path).unwrap();
    write(test_dir_path, b"").unwrap();
    let test_dir = TestDirUtils::new();
    assert!(test_dir_path.is_dir());
    drop(test_dir);
}
