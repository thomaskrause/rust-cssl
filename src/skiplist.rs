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
use std::collections::BinaryHeap;

struct ProxyNode {
    keys : Vec<u32>
}

pub struct SkipList {
    max_level: usize,
    skip: usize,
    proxy_lane: Vec<ProxyNode>,
    fast_lanes: Vec<u32>,
    nodes: LinkedList<u32>,
    items_per_level: Vec<usize>,
    start_of_fast_lanes : Vec<usize>
}

impl SkipList {

    pub fn new(max_level: usize, skip: usize, keys: &[u32]) -> SkipList {

        // build the fast lanes
        let mut items_per_level = vec![0; max_level];
        let mut start_of_fast_lanes = vec![0; max_level];
        let mut flane_size = 16 as usize;

        items_per_level[max_level-1] = flane_size;
        start_of_fast_lanes[max_level-1] = 0;

        for level in (max_level -2)..0 {
            items_per_level[level] = items_per_level[level+1] * skip;
            start_of_fast_lanes[level] = start_of_fast_lanes[level+1] + items_per_level[level+1];
            flane_size += items_per_level[level];
        }

        // create the SkipList datastructure
        let mut result = SkipList {
            max_level: max_level,
            skip: if skip > 1 {skip} else {2},
            proxy_lane: Vec::with_capacity(flane_size),
            fast_lanes: vec![u32::max_value(); flane_size],
            nodes : LinkedList::new(),
            items_per_level: items_per_level,
            start_of_fast_lanes: start_of_fast_lanes,
        };

        // sort the keys by inserting them into a binary heap
        let mut heap = BinaryHeap::new();
        for k in keys {
            heap.push(k);
        }

        // insert each key (in sorted order)
        for k in heap {
            result.insert_sorted_bulk(k)
        }

        // return the result
        result
    }

    fn insert_sorted_bulk(&mut self, key: &u32) {
       self.nodes.push_back(key.clone()); 
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn insert() {
        let vals = [2,1,3,10,0];
        let slist = super::SkipList::new(9,5, &vals);
        
    }
}
