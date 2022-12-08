const INPUT: &str = include_str!("input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut fs = process_input();
    fs.path = "/".to_string();
    let mut size = 0;
    let max_size = 100_000;
    let sub_dirs = fs.current_node().all_sub_directories();
    for sub_dir in sub_dirs {
        if sub_dir.total_size() <= max_size {
            size += sub_dir.total_size();
        }
    }
    println!("Total size: {}", size);
}

fn part2() {
    let mut fs = process_input();
    fs.path = "/".to_string();
    let free_space_require = 30_000_000;
    let minimum_delete_size = free_space_require - fs.free_space();
    let sub_dirs = fs.current_node().all_sub_directories();
    // find smallest sub directory above minimum delete size
    let mut smallest = sub_dirs
        .iter()
        .filter(|d| d.total_size() >= minimum_delete_size)
        .min_by_key(|d| d.total_size())
        .expect("Could not find smallest directory");
    println!("Smallest: {}", smallest.total_size());
}

fn process_input() -> FileSystem {
    let mut fs = FileSystem {
        root: Node::new("/".to_string(), 0, Some(true)),
        path: "/".to_string(),
        max_size: 7_0000_000,
    };

    for line in INPUT.lines() {
        if line.starts_with('$') {
            // process command
            let parts: Vec<&str> = line.split_whitespace().skip(1).collect();
            let command = parts[0];
            match command {
                "cd" => {
                    let path = parts[1];
                    if path.starts_with('/') {
                        fs.path = path.to_string();
                    } else if path.starts_with("..") {
                        let mut parts = fs.path.split('/').collect::<Vec<&str>>();
                        parts.pop();
                        fs.path = parts.join("/");
                    } else {
                        fs.path = format!("{}/{}", fs.path, path);
                    }
                }
                "ls" => {}
                _ => println!("Unknown command: {}", command),
            }
        } else if line.starts_with("dir") {
            // process directory
            let parts: Vec<&str> = line.split_whitespace().skip(1).collect();
            let name = parts[0];
            let node = Node::new(name.to_string(), 0, Some(true));
            fs.current_node_mut().add_child(node);
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let size = parts[0].parse::<usize>().unwrap_or(0);
            let name = parts[1];
            let node = Node::new(name.to_string(), size, None);
            fs.current_node_mut().add_child(node);
        }
    }

    fs
}

struct FileSystem {
    root: Node,
    path: String,
    max_size: usize,
}

impl FileSystem {
    fn current_node(&self) -> &Node {
        let mut current = &self.root;
        for part in self.path.split('/') {
            if part.is_empty() {
                continue;
            }
            current = current
                .children
                .iter()
                .find(|c| c.name == part)
                .expect("Could not find node");
        }
        current
    }

    fn current_node_mut(&mut self) -> &mut Node {
        let mut current = &mut self.root;
        for part in self.path.split('/') {
            if part.is_empty() {
                continue;
            }
            current = current
                .children
                .iter_mut()
                .find(|c| c.name == part)
                .expect("Could not find node");
        }
        current
    }

    fn used_space(&self) -> usize {
        self.root.total_size()
    }

    fn free_space(&self) -> usize {
        self.max_size - self.used_space()
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    size: usize,
    children: Vec<Node>,
    is_dir: bool,
}

impl Node {
    fn new(name: String, size: usize, is_dir: Option<bool>) -> Self {
        Self {
            name,
            size,
            is_dir: is_dir.unwrap_or(false),
            children: Vec::new(),
        }
    }

    fn total_size(&self) -> usize {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<usize>()
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    fn sub_directories(&self) -> Vec<&Node> {
        self.children.iter().filter(|c| c.is_dir).collect()
    }

    fn all_sub_directories(&self) -> Vec<&Node> {
        let mut dirs = Vec::new();
        for child in &self.children {
            if child.is_dir {
                dirs.push(child);
            }
            dirs.extend(child.all_sub_directories());
        }
        dirs
    }
}
