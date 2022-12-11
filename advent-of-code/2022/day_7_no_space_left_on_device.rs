use std::{env, fs, io, io::BufRead, path};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();

    if let Ok(lines) = read_lines(input) {
        // skip / since we already there
        for line in lines {
            if let Ok(line) = line {
                if line.starts_with("$") {
                    let line: Vec<&str> = line.split_whitespace().collect();
                    // only change directory command goes here
                    if let [_, _, path] = line[..] {
                        match path {
                            ".." => {
                                let parent = node.as_ref().borrow().parent.clone().unwrap();
                                node = parent;
                            }
                            _ => {
                                let child = node.as_ref().borrow_mut().children.entry(String::from(path)).or_default().clone();
                                node = child;
                            }
                        }
                    }
                } else {
                    let line: Vec<&str> = line.split_whitespace().collect();
                    if let [size, path] = line[..] {
                        let entry = node.as_ref().borrow_mut().children.entry(String::from(path)).or_default().clone();
                        entry.as_ref().borrow_mut().size = size.parse().unwrap_or(0);
                        entry.as_ref().borrow_mut().parent = Some(node.clone());
                    }
                }
            }
        }
    }

    let directories = root.as_ref().borrow()
        .flatten()
        .into_iter()
        .map(|child| child.as_ref().borrow().total_size());

    let sum_of_total_size: usize = directories.clone()
        .filter(|total_size| *total_size < 100_000)
        .sum();
    println!("first part answer is: {}", sum_of_total_size);

    let max = directories.clone().max().unwrap();
    let the_smallest_directory_to_be_deleted = directories
        .clone()
        .filter(|total_size| *total_size > (30_000_000 - (70_000_000 - max)))
        .min()
        .unwrap();
    println!("second part answer is: {}", the_smallest_directory_to_be_deleted);
}

#[derive(Default, Debug)]
struct Node {
    size: usize,
    parent: Option<Rc<RefCell<Node>>>,
    children: HashMap<String, Rc<RefCell<Node>>>,
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    fn flatten(&self) -> Vec<Rc<RefCell<Node>>> {
        self.children.values()
            .cloned()
            .filter(|child| child.as_ref().borrow().is_dir())
            .into_iter()
            .fold(vec![], |mut result, child| {
                result.push(child.clone());
                result.append(&mut child.as_ref().borrow().flatten());
                result
            })
    }

    fn total_size(&self) -> usize {
        self.children.values()
            .map(|child| child.as_ref().borrow().total_size())
            .sum::<usize>() + self.size
    }
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
