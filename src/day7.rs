use crate::{AOCResult, AOCRunnable};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

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

    fn create_file(&mut self, dir: String, file: FsFile) -> FsNode {
        let path = Path::new(dir.as_str());

        for i in 0..path.iter().count() {
            let file_path = path.iter().nth(i).unwrap();
            let traversed = path.iter().skip(i + 1).collect::<Vec<&OsStr>>();
            let traversed_path = traversed.join("/".as_ref()).to_string_lossy().to_string();
            if file_path == "/" {
                continue;
            }

            for cld in self.children.iter_mut() {
                if let FsNode::Directory(mut child) = cld.clone() {
                    if child.name == file_path.to_string_lossy() {
                        child
                            .children
                            .push(child.clone().create_file(traversed_path, file));
                        *cld = FsNode::Directory(child);
                        return cld.clone();
                    }
                }
            }

            let mut child = FsDirectory::new(file_path.to_string_lossy());
            child.create_file(traversed_path, file);
            self.children.push(FsNode::Directory(child.clone()));
            return FsNode::Directory(child);
        }

        let file = FsNode::File(file);
        self.children.push(file.clone());
        file
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
                    current_fs.create_file(cwd.to_string_lossy().to_string(), file);
                }
            }
        }

        current_fs
    }
}

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> AOCResult<String> {
        let input = input
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let fs = FsDirectory::parse(input);

        // println!("{:#?}", fs);

        Ok(fs.find_pt1().to_string())
    }

    fn run_pt2(input: String) -> AOCResult<String> {
        todo!()
    }
}
