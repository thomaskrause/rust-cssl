use std::collections::LinkedList;

struct SkipList {
    max_level: u8,
    skip: u8,
    nodes: LinkedList<u32>,
}

impl SkipList {

    fn new(max_level: u8, skip: u8) -> SkipList {
        SkipList {
            max_level: max_level,
            skip: if skip > 1 {skip} else {2},
            nodes : LinkedList::new()
        }
    }

    fn insert_bulk(&mut self, key: i32) {
        self.max_level = 0;
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn insert() {
//        let slist = cssl::SkipList::new();
        
    }
}
