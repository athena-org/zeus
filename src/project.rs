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
use std::io::{Read, Write};
use std::path::PathBuf;
use toml;

use git;

// ### File Templates ###

static PROJ_TOML: &'static str =
r#"[game]
name = "{{game_name}}"
version = "0.0.1"
zeus_version = "develop"
athena_version = "develop"
authors = ["{{author_name}}"]"#;

static GITIGNORE: &'static str =
r#"/athena"#;


// ### Create Error ###

#[derive(Debug)]
pub enum ZeusProjectError {
    AlreadyExists,
    NotAZeusProject,
    InvalidPath
}

impl Error for ZeusProjectError {
    fn description(&self) -> &str {
        match *self {
            ZeusProjectError::AlreadyExists => "Already Exists",
            ZeusProjectError::NotAZeusProject => "Not a Zeus Project",
            ZeusProjectError::InvalidPath => "Not a Valid Path"
        }
    }
}

impl Display for ZeusProjectError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match *self {
            ZeusProjectError::AlreadyExists => "Destination path already exists.",
            ZeusProjectError::NotAZeusProject => "Destination path is not a Zeus project.",
            ZeusProjectError::InvalidPath => "Destination path is not valid."
        };

        return write!(f, "{}", message);
    }
}


// ### Zeus Project ###

#[derive(Debug)]
pub struct ZeusProject {
    directory: PathBuf,
    game_name: String
}

impl ZeusProject {
    // ## Accessors ##

    pub fn game_name(&self) -> &str {
        &self.game_name
    }


    // ## Constructors ##

    pub fn create(target_dir: PathBuf) -> Result<ZeusProject, ZeusProjectError> {
        // Sanity check the path
        if target_dir.to_str().unwrap().is_empty() { return Err(ZeusProjectError::InvalidPath) }
        if target_dir.exists() { return Err(ZeusProjectError::AlreadyExists); }

        // Create the actual project
        let project = ZeusProject {
            directory: target_dir,
            game_name: String::from("My Game")
        };

        // Create the directory for this project
        fs::create_dir_all(project.directory.clone()).unwrap();

        // Generate the sample project file
        // TODO: Use some templating library
        let proj_toml = str::replace(PROJ_TOML, "{{game_name}}", "My Game");
        let proj_toml = str::replace(&proj_toml, "{{author_name}}", "Jane Doe");

        // Create basic
        project.create_file("Zeus.toml", &proj_toml);
        project.create_file(".gitignore", GITIGNORE);

        return Ok(project);
    }

    pub fn open(target_dir: PathBuf) -> Result<ZeusProject, ZeusProjectError> {
        // Sanity check the path
        let mut toml_path = target_dir.clone();
        toml_path.push("Zeus.toml");
        if !toml_path.exists() { return Err(ZeusProjectError::NotAZeusProject); }

        // Get data from the toml file
        let mut toml_file = File::open(toml_path).unwrap();
        let mut toml = String::new();
        toml_file.read_to_string(&mut toml).unwrap();
        let value: toml::Value = toml.parse().unwrap();

        let name = value.lookup("game.name").unwrap().as_str().unwrap();

        // We're in a valid project
        let project = ZeusProject {
            directory: target_dir,
            game_name: String::from(name)
        };

        return Ok(project);
    }


    // ## Helpers Functions ##

    pub fn build_editor(&self) {
        self.redownload_athena();
    }

    fn redownload_athena(&self) {
        let mut athena_dir = self.directory.clone();
        athena_dir.push("athena");

        // Delete the old folder if it exists
        if athena_dir.exists() {
            // Set readonly on all files and directories to false
            for file in fs::walk_dir(&athena_dir).unwrap() {
                let path = file.unwrap().path();
                let mut permissions = fs::metadata(&path).unwrap().permissions();
                permissions.set_readonly(false);
                fs::set_permissions(&path, permissions).unwrap();
            }

            // Actually remove the directory
            fs::remove_dir_all(&athena_dir).unwrap();
        }

        // Clone in the latest version of Athena
        // TODO: Actually clone athena instead of zeus right now for testing
        let athena_dir_str = athena_dir.to_str().unwrap();
        git::clone("https://github.com/athena-org/zeus.git", athena_dir_str, "develop").unwrap();
    }

    fn create_file(&self, name: &str, data: &str) {
        let mut path = self.directory.clone();
        path.push(name);
        let mut file = File::create(path).unwrap();
        file.write_all(&data.as_bytes()).unwrap();
    }
}
