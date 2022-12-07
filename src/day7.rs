// TODO: wrap Item in another struct so we can manipulate Rc<RefCell<Item>>
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

#[derive(Debug)]
enum Item {
    Dir(String, String, Vec<Rc<RefCell<Item>>>), // path, name, items
    File(String, u64),                           // filename, size
}

impl Item {
    fn _to_string(&self) -> String {
        match self {
            Self::File(name, size) => format!("<File {} {}>", name, size),
            Self::Dir(path, name, items) => {
                format!(
                    "<Dir {} {} nitems={} size={}>",
                    path,
                    name,
                    items.len(),
                    self.size()
                )
            }
        }
    }

    /// Push a file onto dir.items
    fn push_file(&mut self, filename: &str, size: u64) {
        if let Self::Dir(_, _, items) = self {
            items.push(Rc::new(RefCell::new(Item::File(
                filename.to_string(),
                size,
            ))));
        }
    }

    fn push_dir(&mut self, dirname: &str) {
        if let Self::Dir(path, _, items) = self {
            let mut path = path.clone();
            if path != "/" {
                path.push_str("/");
            }
            path.push_str(dirname);
            items.push(Rc::new(RefCell::new(Item::Dir(
                path.to_string(),
                dirname.to_string(),
                vec![],
            ))));
        }
    }

    /// Takes a cursor, then return a cursor that points to the target dir
    fn chdir(&self, dirname: &str) -> Rc<RefCell<Item>> {
        if let Self::Dir(_, _, items) = self {
            for item in items {
                if let Item::Dir(_, n, _) = &*item.borrow() {
                    if n == dirname {
                        return Rc::clone(item);
                    }
                }
            }
        }
        unreachable!();
    }

    fn size(&self) -> u64 {
        match self {
            Self::File(_, size) => return *size,
            Self::Dir(_, _, items) => {
                return items.iter().map(|item| item.borrow().size()).sum();
            }
        }
    }
}

/// Recusively walk through all items and collect them into a single vector
fn flatten(cur: Rc<RefCell<Item>>) -> Vec<Rc<RefCell<Item>>> {
    let mut all = vec![];

    match &*cur.borrow() {
        Item::File(..) => all.push(Rc::clone(&cur)),
        Item::Dir(.., items) => {
            all.push(Rc::clone(&cur));
            items.iter().for_each(|item| {
                let sub_list = flatten(Rc::clone(item));
                for sub_item in sub_list {
                    all.push(sub_item);
                }
            });
        }
    }

    return all;
}

/// Return the arg sent to the shit
fn parse_cd(cd: &str) -> String {
    let line = cd.lines().next().unwrap(); // cd spans one line
    let mut tokens = line.split(" ");
    tokens.next().unwrap(); // cd
    return tokens.next().unwrap().to_string();
}

/// Return a list of directory names and a list of (filename, filesize)
fn parse_ls(ls: &str) -> (Vec<String>, Vec<(String, u64)>) {
    let mut dirs = vec![];
    let mut files = vec![];
    ls.lines().for_each(|line| {
        if line.contains("dir") {
            let mut tokens = line.split(" ");
            tokens.next(); // "dir"
            dirs.push(tokens.next().unwrap().to_string());
        } else if &line[..2] == "ls" {
            // do nothing
        } else {
            let mut tokens = line.split(" ");
            let size = tokens.next().unwrap().parse::<u64>().unwrap();
            let filename = tokens.next().unwrap().to_string();
            files.push((filename, size));
        }
    });

    return (dirs, files);
}

pub fn solve(input: &str) {
    // Parsing command and mutating the state
    let input = fs::read_to_string(input).unwrap();

    let root = Rc::new(RefCell::new(Item::Dir(
        "/".to_string(),
        "".to_string(),
        vec![],
    )));
    let mut cur = Rc::clone(&root);
    let mut stack: Vec<Rc<RefCell<Item>>> = vec![]; // push and pop

    input.split("$ ").for_each(|cmd| {
        if cmd.len() < 2 {
            ()
        } else if &cmd[0..2] == "ls" {
            let (dirs, files) = parse_ls(cmd);
            dirs.iter()
                .for_each(|dirname| cur.borrow_mut().push_dir(dirname));
            files
                .iter()
                .for_each(|(filename, size)| cur.borrow_mut().push_file(filename, *size));
        } else if &cmd[..2] == "cd" {
            let dest = parse_cd(cmd);
            match &dest[..] {
                "/" => {
                    cur = Rc::clone(&root);
                    while stack.len() > 0 {
                        stack.pop();
                    }
                }
                ".." => {
                    cur = stack.pop().unwrap();
                }
                _ => {
                    stack.push(Rc::clone(&cur));
                    let next = Rc::clone(&cur.borrow_mut().chdir(&dest));
                    cur = Rc::clone(&next);
                }
            }
        }
    });

    let items = flatten(Rc::clone(&root));
    let total_space = 70000000;
    let needed_space = 30000000;
    let available_space = total_space - root.borrow().size();
    let threshold = needed_space - available_space;
    let mut sum = 0;
    let mut min = total_space;

    for item in items {
        let item_ = &*item.borrow();
        match item_ {
            Item::Dir(..) => {
                // println!("{path}, {}", item_.size());
                if item_.size() <= 100000 {
                    sum += item_.size();
                }
                if item_.size() >= threshold && item_.size() < min {
                    min = item_.size();
                }
            }
            Item::File(..) => {
                // println!("{name} {size}");
            }
        }
    }
    println!("{sum}");
    println!("{min}");
}
