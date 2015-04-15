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

use std::error::Error;
use std::fs;
use std::fs::{PathExt, File};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::path::PathBuf;

use git;

// ### File Templates ###

static PROJ_TOML: &'static str =
r#"[game]
name = "{{game_name}}"
version = "0.0.1"
authors = ["{{author_name}}"]"#;


// ### Create Error ###

#[derive(Debug)]
pub enum CreateError {
    AlreadyExists
}

impl Error for CreateError {
    fn description(&self) -> &str {
        match *self {
            CreateError::AlreadyExists => "Already Exists"
        }
    }
}

impl Display for CreateError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match *self {
            CreateError::AlreadyExists => "Destination path already exists."
        };

        return write!(f, "{}", message);
    }
}


// ### Open Error ###

#[derive(Debug)]
pub enum OpenError {
    NotAZeusProject
}

impl Error for OpenError {
    fn description(&self) -> &str {
        match *self {
            OpenError::NotAZeusProject => "Not a Zeus Project"
        }
    }
}

impl Display for OpenError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match *self {
            OpenError::NotAZeusProject => "Destination is not a Zeus project."
        };

        return write!(f, "{}", message);
    }
}


// ### Zeus Project ###

#[derive(Debug)]
pub struct ZeusProject {
    path: PathBuf
}

impl ZeusProject {
    // ## Constructors ##

    pub fn create(target_path: PathBuf) -> Result<ZeusProject, CreateError> {
        // Sanity check the path
        if target_path.exists() { return Err(CreateError::AlreadyExists); }

        // Create the actual project
        let project = ZeusProject {
            path: target_path
        };

        // Create the directory for this project
        fs::create_dir_all(project.path.clone()).unwrap();

        // Generate the sample project file
        // TODO: Use some templating library
        let proj_toml = str::replace(PROJ_TOML, "{{game_name}}", "My Game");
        let proj_toml = str::replace(&proj_toml, "{{author_name}}", "Jane Doe");

        // Actually write the project file to our project directory
        let mut proj_toml_path = project.path.clone();
        proj_toml_path.push("Zeus.toml");
        let mut proj_file = File::create(proj_toml_path).unwrap();
        proj_file.write_all(&proj_toml.into_bytes()).unwrap();

        return Ok(project);
    }

    pub fn open(target_path: PathBuf) -> Result<ZeusProject, OpenError> {
        return Err(OpenError::NotAZeusProject);
    }


    // ## Memeber Functions ##

    pub fn prepare_for_editor(&self) {
        self.update_athena();
    }

    pub fn update_athena(&self) {
        let mut athena_path = self.path.clone();
        athena_path.push("athena");

        // Delete the old folder if it exists
        if athena_path.exists() {
            fs::remove_dir_all(athena_path.clone()).unwrap();
        }

        // Clone in the latest version of Athena
        // TODO: Actually clone athena instead of zeus right now for testing
        git::clone("https://github.com/athena-org/zeus.git", athena_path.to_str().unwrap()).unwrap();
    }
}
