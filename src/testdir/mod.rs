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
use std::fs::{create_dir_all, read, read_dir, remove_dir_all, remove_file, write};
use std::io::Result;
use std::path::Path;

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
/// Since *Rust* runs unit-tests using multiple threads, this instance will
/// create a unique subdirectory for each instance. By default, this
/// subdirectory is deleted when the instance goes out of scope.
pub struct TestDirUtils {
    test_dir: OsString,
    delete_on_terminate: bool,
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
    pub fn new(name: &str) -> Result<Self> {
        Self::with_root(Path::new(Self::DEFAULT_TEST_DIR), name)
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
    pub fn with_root(test_root: &Path, name: &str) -> Result<Self> {
        let unique_test_dir = Self::create_unique_name_for_thread(name);
        let full_path = test_root.join(Path::new(&unique_test_dir));
        if full_path.is_file() {
            remove_file(full_path.as_path())?;
        }
        if !full_path.exists() {
            create_dir_all(full_path.as_path())?;
        }
        Ok(Self {
            test_dir: full_path.into_os_string(),
            delete_on_terminate: true,
        })
    }

    /// Returns the current value of delete_on_terminate. If true, the
    /// test director will be destroyed when this struct is dropped. If it is
    /// set to false, the directory will not be deleted.
    ///
    /// This flag is true by default.
    pub fn delete_on_terminate(&self) -> bool {
        self.delete_on_terminate
    }

    /// Changes the flag delete_on_terminate.
    pub fn set_delete_on_terminate(&mut self, delete_on_terminate: bool) {
        self.delete_on_terminate = delete_on_terminate;
    }

    fn create_unique_name_for_thread(name: &str) -> String {
        format!("{}-{:?}", name, std::thread::current().id())
    }

    /// Returns the path of the test directory.
    pub fn test_dir(&self) -> &Path {
        Path::new(&self.test_dir)
    }

    /// Deletes all of the contents of the test directory without removing it.
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

    /// Creates a test file with the specfied name and write something into it.
    ///
    /// Arguments:
    /// - `name`: The name of the file to be created;
    /// - `contents`: The contents of the file;
    ///
    /// Returns the path to the newly created file.
    pub fn create_test_file(&self, name: &str, contents: &[u8]) -> Result<OsString> {
        let full_path = self.get_test_file_path(name);
        let p = Path::new(&full_path);
        write(p, contents)?;
        Ok(full_path)
    }

    /// Creates an empty test file with the specfied name. The file will have no contents as
    /// it is equivalent to call [`TestDirUtils::create_test_file()`] with `b""` as its
    /// contents.
    ///
    /// Arguments:
    /// - `name`: The name of the file to be created;
    ///
    /// Returns the path to the newly created file.
    pub fn touch_test_file(&self, name: &str) -> Result<OsString> {
        self.create_test_file(name, b"")
    }

    /// Reads all the contents of the specified test file. It uses [`std::fs::read()`]
    /// so it is subjected to the same restrictions.
    ///
    /// Arguments:
    /// - `name`: The name of the file to be created;
    ///
    /// Returns the contents of the file.
    pub fn read_test_file(&self, name: &str) -> Result<Vec<u8>> {
        let full_path = self.get_test_file_path(name);
        let p = Path::new(&full_path);
        Ok(read(p)?)
    }

    /// Deletes the specified file.
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
}

impl Drop for TestDirUtils {
    fn drop(&mut self) {
        if self.delete_on_terminate {
            remove_dir_all(Path::new(self.test_dir())).unwrap();
        }
    }
}
