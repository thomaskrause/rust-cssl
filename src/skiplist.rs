//    Copyright 2017 Thomas Krause <thomaskrause@posteo.de>
// 
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
// 
//        http://www.apache.org/licenses/LICENSE-2.0
// 
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

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
