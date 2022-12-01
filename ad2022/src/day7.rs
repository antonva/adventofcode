use std::{cell::RefCell, rc::Rc};
/// --- Day 7: No Space Left On Device ---
///
/// You can hear birds chirping and raindrops hitting leaves as the expedition proceeds. Occasionally, you can even hear much louder sounds in the distance; how big do the animals get out here, anyway?
///
/// The device the Elves gave you has problems with more than just its communication system. You try to run a system update:
///
/// $ system-update --please --pretty-please-with-sugar-on-top
/// Error: No space left on device
///
/// Perhaps you can delete some files to make space for the update?
///
/// You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input). For example:
///
/// $ cd /
/// $ ls
/// dir a
/// 14848514 b.txt
/// 8504156 c.dat
/// dir d
/// $ cd a
/// $ ls
/// dir e
/// 29116 f
/// 2557 g
/// 62596 h.lst
/// $ cd e
/// $ ls
/// 584 i
/// $ cd ..
/// $ cd ..
/// $ cd d
/// $ ls
/// 4060174 j
/// 8033020 d.log
/// 5626152 d.ext
/// 7214296 k
///
/// The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.
///
/// Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:
///
///     cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
///         cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
///         cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
///         cd / switches the current directory to the outermost directory, /.
///     ls means list. It prints out all of the files and directories immediately contained by the current directory:
///         123 abc means that the current directory contains a file named abc with size 123.
///         dir xyz means that the current directory contains a directory named xyz.
///
/// Given the commands and output in the example above, you can determine that the filesystem looks visually like this:
///
/// - / (dir)
///   - a (dir)
///     - e (dir)
///       - i (file, size=584)
///     - f (file, size=29116)
///     - g (file, size=2557)
///     - h.lst (file, size=62596)
///   - b.txt (file, size=14848514)
///   - c.dat (file, size=8504156)
///   - d (dir)
///     - j (file, size=4060174)
///     - d.log (file, size=8033020)
///     - d.ext (file, size=5626152)
///     - k (file, size=7214296)
///
/// Here, there are four directories: / (the outermost directory), a and d (which are in /), and e (which is in a). These directories also contain files of various sizes.
///
/// Since the disk is full, your first step should probably be to find directories that are good candidates for deletion. To do this, you need to determine the total size of each directory. The total size of a directory is the sum of the sizes of the files it contains, directly or indirectly. (Directories themselves do not count as having any intrinsic size.)
///
/// The total sizes of the directories above can be found as follows:
///
///     The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
///     The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
///     Directory d has total size 24933642.
///     As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size of every file.
///
/// To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes. In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)
///
/// Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?

pub fn exec(input: &str) {
    let res1 = testable_exec_part1(input);
    let res2 = testable_exec_part2(input);

    println!("Day 6\n\tpart 1:\t{res1}\n\tpart 2:\t{res2}");
}

fn testable_exec_part1(input: &str) -> usize {
    //parse_log(input);
    0
}
fn testable_exec_part2(input: &str) -> usize {
    0
}

struct FSNode {
    name: String,
    parent: Option<Rc<RefCell<FSNode>>>,
    children: Option<Rc<RefCell<Vec<FSNode>>>>,
    is_directory: bool,
    size: usize,
}

impl Default for FSNode {
    // New filesystem tree
    fn default() -> Self {
        Self {
            name: "/".to_string(),
            parent: None,
            children: None,
            is_directory: true,
            size: 0,
        }
    }
}

impl FSNode {
    // New filesystem node
    fn new(name: &str, is_directory: bool, size: usize) -> Self {
        Self {
            name: name.to_string(),
            parent: None,
            children: None,
            is_directory,
            size,
        }
    }

    fn add_child(self, name: &str, is_directory: bool, size: usize) {
        if is_directory {
            match self.children {
                None => {
                    let new_node = Rc::new(RefCell::new(FSNode::new(name, is_directory, size)));
                    if let None = self.parent {
                        let rc = Rc::new(RefCell::new(self));
                        new_node.borrow_mut().parent = Some(rc);
                    }
                }
                Some(children) => {}
            }
        }
    }
}

//fn parse_log(input: &str) -> bool {
//    let mut path: Vec<String> = vec![];
//    let x: Vec<u32> = input
//        .lines()
//        .map(|line| {
//            let split_line: Vec<&str> = line.split(" ").collect();
//            match split_line[0] {
//                "$" => match split_line[1] {
//                    "cd" => {
//                        print!("cd {path:?} -> ");
//                        let mut cpath = path.clone();
//                        //path = change_directory(&path, split_line[2]);
//                        //println!("{cpath:?}");
//                    }
//                    "ls" => println!("list directory"),
//                    _ => (),
//                },
//                "dir" => {
//                    println!("dir found");
//                }
//                filesize => {
//                    println!("filesize found: {filesize}");
//                }
//            }
//            0
//        })
//        .collect();
//    false
//}

//fn change_directory(mut path: &Vec<String>, pattern: &str) -> &Vec<String> {
//    if path.is_empty() {
//        if pattern == "/" {
//            path.push("/".to_string())
//        } else {
//            panic!("Trying to add non root to root")
//        }
//    } else {
//        match pattern {
//            ".." => {
//                path.pop();
//            }
//            dir => path.push(dir.to_string()),
//        }
//    }
//    path
//}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1_part_1_works() {
        // $ cd /
        // $ ls
        // dir a
        // 14848514 b.txt
        // 8504156 c.dat
        // dir d
        // $ cd a
        // $ ls
        // dir e
        // 29116 f
        // 2557 g
        // 62596 h.lst
        // $ cd e
        // $ ls
        // 584 i
        // $ cd ..
        // $ cd ..
        // $ cd d
        // $ ls
        // 4060174 j
        // 8033020 d.log
        // 5626152 d.ext
        // 7214296 k
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n";
        assert_eq!(testable_exec_part1(input), 95437);
    }
    #[test]
    fn example2_part_1_works() {
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n";
        assert_eq!(testable_exec_part1(input), 6);
    }

    #[test]
    fn new_filesystem_works() {
        //let fsn = FSNode::new();
        //assert_eq!(fsn.name, "/".to_string());
        //assert_eq!(fsn.size, 0);
    }
}
