use itertools::Itertools;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::str::FromStr;

#[derive(Default)]
struct File {
    name: String,
    size: usize,
    parent: RefCell<Weak<Item>>,
}

#[derive(Default)]
struct Folder {
    name: String,
    content: RefCell<Vec<Rc<Item>>>,
    parent: RefCell<Weak<Item>>,
}

enum Item {
    File(File),
    Folder(Folder),
}

impl ItemGetters for Item {
    fn get_size(&self) -> usize {
        match self {
            Item::File(file) => file.get_size(),
            Item::Folder(folder) => folder.get_size(),
        }
    }
    fn get_name(&self) -> &str {
        match self {
            Item::File(file) => file.get_name(),
            Item::Folder(folder) => folder.get_name(),
        }
    }
    fn get_parent(&self) -> Weak<Item> {
        match self {
            Item::File(file) => file.get_parent(),
            Item::Folder(folder) => folder.get_parent(),
        }
    }

    fn set_parent(&self, parent: Weak<Item>) {
        match self {
            Item::File(file) => file.set_parent(parent),
            Item::Folder(folder) => folder.set_parent(parent),
        }
    }

    fn find_child(&self, name: &str) -> Option<Weak<Item>> {
        match self {
            Item::File(file) => file.find_child(name),
            Item::Folder(folder) => folder.find_child(name),
        }
    }

    fn find_or_add_folder(&self, name: &str) -> Weak<Item> {
        match self {
            Item::File(file) => file.find_or_add_folder(name),
            Item::Folder(folder) => folder.find_or_add_folder(name),
        }
    }

    fn add_file(&self, name: &str, size: usize) -> Weak<Item> {
        match self {
            Item::File(file) => file.add_file(name, size),
            Item::Folder(folder) => folder.add_file(name, size),
        }
    }
    fn add_folder(&self, name: &str) -> Weak<Item> {
        match self {
            Item::File(file) => file.add_folder(name),
            Item::Folder(folder) => folder.add_folder(name),
        }
    }

    fn get_folders(&self) -> Vec<Weak<Item>> {
        match self {
            Item::File(file) => file.get_folders(),
            Item::Folder(folder) => folder.get_folders(),
        }
    }
}

trait ItemGetters {
    fn get_size(&self) -> usize;
    fn get_name(&self) -> &str;
    fn get_folders(&self) -> Vec<Weak<Item>>;
    fn get_parent(&self) -> Weak<Item>;
    fn set_parent(&self, parent: Weak<Item>);

    fn find_child(&self, name: &str) -> Option<Weak<Item>>;
    fn find_or_add_folder(&self, name: &str) -> Weak<Item>;
    fn add_file(&self, name: &str, size: usize) -> Weak<Item>;
    fn add_folder(&self, name: &str) -> Weak<Item>;
}

impl ItemGetters for File {
    fn get_size(&self) -> usize {
        self.size
    }
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn get_parent(&self) -> Weak<Item> {
        self.parent.borrow().clone()
    }
    fn set_parent(&self, parent: Weak<Item>) {
        *self.parent.borrow_mut() = parent;
    }

    fn find_child(&self, _name: &str) -> Option<Weak<Item>> {
        unreachable!()
    }

    fn find_or_add_folder(&self, _name: &str) -> Weak<Item> {
        unreachable!()
    }

    fn add_file(&self, _name: &str, _size: usize) -> Weak<Item> {
        unreachable!()
    }
    fn add_folder(&self, _name: &str) -> Weak<Item> {
        unreachable!()
    }

    fn get_folders(&self) -> Vec<Weak<Item>> {
        unreachable!()
    }
}

impl ItemGetters for Folder {
    fn get_size(&self) -> usize {
        self.content
            .borrow()
            .iter()
            .fold(0, |acc, item| acc + item.get_size())
    }
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn get_parent(&self) -> Weak<Item> {
        self.parent.borrow().clone()
    }
    fn set_parent(&self, parent: Weak<Item>) {
        *self.parent.borrow_mut() = parent;
    }

    fn find_child(&self, name: &str) -> Option<Weak<Item>> {
        self.content
            .borrow()
            .iter()
            .find(|item| item.get_name() == name)
            .map(Rc::downgrade)
    }

    fn find_or_add_folder(&self, name: &str) -> Weak<Item> {
        let folder = self.find_child(name);
        if let Some(folder) = folder {
            return folder;
        }
        self.add_folder(name)
    }

    fn add_file(&self, name: &str, size: usize) -> Weak<Item> {
        let file = Rc::new(Item::File(File {
            name: name.to_owned(),
            size,
            ..Default::default()
        }));
        self.content.borrow_mut().push(file);
        Rc::downgrade(self.content.borrow().last().unwrap())
    }

    fn add_folder(&self, name: &str) -> Weak<Item> {
        let folder = Rc::new(Item::Folder(Folder {
            name: name.to_owned(),
            ..Default::default()
        }));
        self.content.borrow_mut().push(folder);
        Rc::downgrade(self.content.borrow().last().unwrap())
    }

    fn get_folders(&self) -> Vec<Weak<Item>> {
        let mut res = vec![];
        for item in self.content.borrow().iter() {
            if let Item::Folder(_) = item.borrow() {
                res.push(Rc::downgrade(item));
            }
        }
        res
    }
}

enum Command {
    Ls,
    Cd(Cd),
}

enum Cd {
    GoToRoot,
    GoBack,
    Goto(String),
}

enum CommandError {
    ParseError,
}
impl Cd {
    fn new(to: Option<&&str>) -> Self {
        match to {
            None => Cd::GoToRoot,
            Some(&"..") => Cd::GoBack,
            Some(name) => Cd::Goto((*name).to_owned()),
        }
    }
}

impl FromStr for Command {
    type Err = CommandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect_vec();
        if words[0] == "$" {
            return match words[1] {
                "cd" => Ok(Command::Cd(Cd::new(words.get(2)))),
                "ls" => Ok(Command::Ls),
                _ => Err(Self::Err::ParseError),
            };
        }
        Err(Self::Err::ParseError)
    }
}

fn input_generator(input: &str) -> Rc<Item> {
    let root: Rc<Item> = Rc::new(Item::Folder(Folder {
        name: "/".to_owned(),
        ..Default::default()
    }));

    let mut current_dir = Rc::downgrade(&root);

    for line in input.lines() {
        let command = Command::from_str(line);
        if let Ok(command) = command {
            if let Command::Cd(cd) = command {
                current_dir = match cd {
                    Cd::GoToRoot => Rc::downgrade(&root),
                    Cd::GoBack => current_dir.upgrade().unwrap().get_parent(),
                    Cd::Goto(name) => {
                        let folder = current_dir
                            .upgrade()
                            .unwrap()
                            .find_or_add_folder(name.as_str());
                        folder
                            .upgrade()
                            .unwrap()
                            .borrow_mut()
                            .set_parent(current_dir.clone());
                        folder
                    }
                }
            }
        } else {
            // file name
            let (size, name) = line.split_whitespace().collect_tuple().unwrap();
            match size {
                "dir" => (),
                _ => {
                    // let folder = *current_dir.upgrade().unwrap();
                    let mut folder = current_dir.upgrade().unwrap();

                    if folder.find_child(name).is_none() {
                        let size = size.parse::<usize>().unwrap();
                        let file = folder.borrow_mut().add_file(name, size);
                        file.upgrade()
                            .unwrap()
                            .borrow_mut()
                            .set_parent(current_dir.clone());
                    }
                }
            }
        }
    }

    root
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    get_part1_score(Rc::downgrade(&input_generator(input)), 100000)
}

fn get_part1_score(folder: Weak<Item>, max_size: usize) -> usize {
    let mut res = 0;
    let folder = folder.upgrade().unwrap();
    let size = folder.get_size();
    if size < max_size {
        res += size;
    }

    for child in folder.get_folders() {
        res += get_part1_score(child, max_size);
    }

    res
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let root = input_generator(input);
    let free_space = 70_000_000 - root.get_size();
    let space_needed = 30_000_000 - free_space;
    get_part2_score(Rc::downgrade(&root), space_needed)
}

fn get_part2_score(folder: Weak<Item>, space_needed: usize) -> usize {
    let mut res = 70_000_000;
    let folder = folder.upgrade().unwrap();
    let size = folder.get_size();

    if size > space_needed {
        res = size;
    }

    for child in folder.get_folders() {
        let size = get_part2_score(child, space_needed);
        if size >= space_needed && size < res {
            res = size;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    fn sample1() {
        assert_eq!(part1(EXAMPLE), 95437);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(EXAMPLE), 24933642);
    }
}
