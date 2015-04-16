// Copyright 2015 The Athena Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::process::{Command, Stdio};

// TODO: Replace with libgit2 perhaps

// On windows, it's probably installed here
#[cfg(windows)]
static GIT_PATH: &'static str = r#"C:\Program Files (x86)\Git\bin\git.exe"#;

// On linux/osx assume it's in path
#[cfg(not(windows))]
static GIT_PATH: &'static str = r#"git"#;

pub fn clone(url: &str, path: &str, branch_or_tag: &str) -> Result<(), ()> {
	let output = Command::new(GIT_PATH)
		.args(&[
			"clone",
			"--branch", branch_or_tag,
			"--depth", "1",
			url, path])
		.stdin(Stdio::null())
		.output()
		.unwrap_or_else(|_| {
			panic!("Failed to execute git process!")
		});

	if output.status.success() {
		return Ok(());
	} else {
		return Err(());
	}
}
