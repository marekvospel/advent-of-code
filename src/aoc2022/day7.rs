use crate::AOCRunnable;
use anyhow::Result;
use std::path::PathBuf;

pub struct AOCDay;

#[derive(Clone, Eq, PartialEq, Debug)]
struct FsFile {
    name: String,
    size: i32,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct FsDirectory {
    name: String,
    children: Vec<FsNode>,
}

impl FsDirectory {
    fn new<S: Into<String>>(name: S) -> Self {
        FsDirectory {
            name: name.into(),
            children: Vec::new(),
        }
    }

    fn create_file(&mut self, dir: &str, file: FsFile) -> FsNode {
        let parts = dir.split("/");

        // No other directories to create, create file
        if dir == "/" || dir == "" {
            self.children.push(FsNode::File(file.clone()));
            FsNode::File(file)
        // Traverse directories and recurse
        } else {
            let path: Vec<&str> = parts
                .skip(match dir.starts_with("/") {
                    true => 1,
                    false => 0,
                })
                .collect();
            let dirname = path.first().unwrap();
            let next = path
                .iter()
                .skip(1)
                .map(|s| s.to_owned())
                .collect::<Vec<&str>>()
                .join("/");

            // Directory exists
            if let Some(FsNode::Directory(dir)) = self.children.iter_mut().find(|d| match d {
                FsNode::Directory(d) => &d.name == dirname,
                _ => false,
            }) {
                dir.create_file(&next, file)
            } else {
                self.children.push(FsNode::Directory(FsDirectory {
                    name: dirname.to_string(),
                    children: vec![],
                }));

                if let Some(FsNode::Directory(dir)) = self.children.iter_mut().find(|d| match d {
                    FsNode::Directory(d) => &d.name == dirname,
                    _ => false,
                }) {
                    dir.create_file(&next, file)
                } else {
                    unimplemented!()
                }
            }
        }
    }

    fn calc_size(&self) -> i32 {
        let mut result = 0;

        for child in self.children.iter() {
            match child {
                FsNode::File(file) => {
                    result += file.size;
                }
                FsNode::Directory(dir) => {
                    result += dir.calc_size();
                }
            }
        }

        result
    }

    fn find_pt1(&self) -> i32 {
        let mut result = 0;

        for child in self.children.iter() {
            if let FsNode::Directory(dir) = child {
                let size = dir.calc_size();
                if size <= 100000 {
                    result += size;
                }
                result += dir.find_pt1();
            }
        }

        result
    }

    fn find_pt2(&self, needed: i32) -> i32 {
        let mut result = i32::MAX;

        for child in self.children.iter() {
            if let FsNode::Directory(dir) = child {
                let size = dir.calc_size();
                println!("{}: {size}", dir.name);
                if size > needed && size < result {
                    println!("Setting A");
                    result = size;
                }
                let size = dir.find_pt2(needed);
                if size > needed && size < result {
                    println!("Setting B");
                    result = size;
                }
            }
        }

        result
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum FsNode {
    Directory(FsDirectory),
    File(FsFile),
}

impl FsDirectory {
    fn parse(input: Vec<String>) -> Self {
        let mut current_fs = FsDirectory::new('/');
        let mut cwd = PathBuf::new();

        let mut ls_command = false;

        for line in input {
            if line.starts_with("$ ") {
                ls_command = false;

                let command = line.split(' ').nth(1);
                // println!("command: {:?}", command);

                match command {
                    Some("ls") => {
                        ls_command = true;
                        continue;
                    }
                    Some("cd") => {
                        let target = line.split(' ').skip(2).collect::<Vec<&str>>().join(" ");
                        let target = target.as_str();

                        match target {
                            "/" => cwd = PathBuf::from("/"),
                            ".." => {
                                cwd.pop();
                            }
                            path => cwd.push(path),
                        };
                    }
                    _ => {}
                }
            }

            if ls_command {
                let file_name = line.split(' ').skip(1).collect::<String>();

                if !line.starts_with("dir") && !file_name.is_empty() {
                    let file = FsFile {
                        name: file_name.clone(),
                        size: line.split(' ').next().unwrap().parse::<i32>().unwrap(),
                    };
                    current_fs.create_file(&cwd.to_string_lossy(), file);
                }
            }
        }

        current_fs
    }
}

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> Result<String> {
        let input = input
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let fs = FsDirectory::parse(input);

        Ok(fs.find_pt1().to_string())
    }

    fn run_pt2(input: String) -> Result<String> {
        let input = input
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let fs = FsDirectory::parse(input);
        let free = 30000000 - (70000000 - fs.calc_size());

        Ok(fs.find_pt2(free).to_string())
    }
}
