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
//! This module contains utilities that helps the usage of a test directory by
//! unit-tests.
#[cfg(test)]
mod tests;

use std::ffi::OsString;
use std::fs::{read_dir, remove_dir_all, remove_file, write, DirBuilder};
use std::io::Result;
use std::path::Path;
use std::sync::{Mutex, MutexGuard, Once};

//=============================================================================
// TestDirUtils
//-----------------------------------------------------------------------------
/// This struct implements a set of utilities that helps with the management of
/// test files used inside the unit tests.
///
/// By default, it creates the test files inside a directory called
/// "test_dir.tmp". It is recommended to add this directory to the ignore list
/// of your version control system in order to prevent the addition of the
/// test files into the version control by accident.
///
/// Since *Rust* runs unit-tests using multiple threads, instances of this
/// struct will try to hold a global lock that will serialize the execution of
/// all tests that uses this struct. In other words, only one instance of this
/// struct should be created inside a given unit-test function or it will
/// end-up in a dead-lock.
pub struct TestDirUtils {
    test_dir: OsString,
    _lock: MutexGuard<'static, usize>,
}

impl TestDirUtils {
    /// Directory to be used by the unit tests. Its is always "test_dir.tmp".
    pub const DEFAULT_TEST_DIR: &'static str = "test_dir.tmp";

    /// Creates a new `TestDirUtils` with the default name.
    /// It will automatically create the test directory if it does not exist.
    /// If the default path points to a file or a symlink, it will be deleted
    /// and recreated as a directory.
    ///
    /// Returns the new instance of an error if the test directory is invalid
    /// or cannot be created.
    pub fn new() -> Result<Self> {
        Self::with_path(Path::new(Self::DEFAULT_TEST_DIR))
    }

    /// Creates a new `TestDirUtils`. It will automatically create
    /// the test directory if it does not exist. If the path points to a file or a
    /// symlink, it will be deleted and recreated as a directory.
    ///
    /// As a safeguard, this constructor will panic if `test_dir` points to a root
    /// or a prefix (see [`std::path::Path::parent()`] for further details about how
    /// the root is detected).
    ///
    /// Arguments:
    /// - `test_dir`: The path to the test directory;
    ///
    /// Returns the new instance of an error if the test directory is invalid
    /// or cannot be created.
    pub fn with_path(test_dir: &Path) -> Result<Self> {
        if test_dir.parent().is_none() {
            panic!("TestDirUtils said: It is not safe to run 'rm -Rf /', don't you agree?");
        }
        let lock = Self::get_global_lock();
        if !test_dir.is_dir() {
            if test_dir.exists() {
                remove_file(test_dir)?;
            }
            DirBuilder::new().recursive(true).create(test_dir)?;
        }
        Ok(Self {
            test_dir: OsString::from(test_dir),
            _lock: lock,
        })
    }

    /// Returns the path to the test directory.
    pub fn test_dir(&self) -> &Path {
        Path::new(&self.test_dir)
    }

    /// Deletes all of the contents of the test directory.
    pub fn reset(&self) -> Result<()> {
        for entry in read_dir(self.test_dir())? {
            match entry {
                Ok(e) => {
                    let file_type = e.file_type()?;
                    if file_type.is_file() || file_type.is_symlink() {
                        remove_file(e.path())?;
                    } else if file_type.is_dir() {
                        remove_dir_all(e.path())?;
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Get the path of a file inside the test directory.
    pub fn get_test_file_path(&self, name: &str) -> OsString {
        let path = Path::new(&self.test_dir);
        path.join(name).into_os_string()
    }

    /// Create a test file inside the specified path. This file
    /// will have as its contents, the UTF-8 bytes that represents
    /// the name of the file.
    ///
    /// Arguments:
    /// - `name`: The name of the file to be created;
    ///
    /// Returns the path to the newly created file.
    pub fn create_test_file(&self, name: &str) -> Result<OsString> {
        let full_path = self.get_test_file_path(name);
        let p = Path::new(&full_path);
        write(p, full_path.to_str().unwrap().as_bytes())?;
        Ok(full_path)
    }

    /// Create a test file inside the specified path. This file
    /// will have as its contents, the UTF-8 bytes that represents
    /// the name of the file.
    ///
    /// This method does nothing if the test file does not exist.
    ///
    /// Arguments:
    /// - `name`: The name of the file to be removed;
    pub fn delete_test_file(&self, name: &str) -> Result<()> {
        let full_path = self.get_test_file_path(name);
        let p = Path::new(&full_path);
        if p.exists() {
            remove_file(p)
        } else {
            Ok(())
        }
    }

    fn get_global_lock() -> MutexGuard<'static, usize> {
        static mut MUTEX: Option<Mutex<usize>> = None;
        static ONCE: Once = Once::new();
        unsafe {
            ONCE.call_once(|| {
                MUTEX.replace(Mutex::default());
            });
            MUTEX.as_ref().unwrap().lock().unwrap()
        }
    }
}
