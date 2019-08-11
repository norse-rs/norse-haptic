use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Path(u64);
impl Path {
    pub const NULL: Path = Path(0);
}

pub struct Interner {
    path_to_str: Vec<Arc<String>>,
    str_to_path: HashMap<Arc<String>, Path>,
}

// TODO: slow
impl Interner {
    pub fn new() -> Self {
        Interner {
            path_to_str: Vec::new(),
            str_to_path: HashMap::new(),
        }
    }

    pub fn intern(&mut self, string: &str) -> Path {
        match self.str_to_path.entry(Arc::new(string.into())) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let path = Path(1 + self.path_to_str.len() as u64);
                let key = entry.key().clone();
                entry.insert(path);
                self.path_to_str.push(key);
                path
            }
        }
    }

    pub fn untern(&self, path: Path) -> Arc<String> {
        self.path_to_str[path.0 as usize - 1].clone()
    }
}
