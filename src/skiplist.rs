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
    keys: Vec<u32>,
}

pub struct SkipList {
    max_level: usize,
    skip: usize,
    proxy_lane: Vec<ProxyNode>,
    fast_lanes: Vec<u32>,
    nodes: Vec<u32>,
    max_items_per_level: Vec<usize>,
    start_of_fast_lanes: Vec<usize>,
    num_items_per_level: Vec<usize>,
}

impl SkipList {
    pub fn new(max_level: usize, skip: usize, keys: &[u32]) -> SkipList {

        // build the fast lanes
        let mut max_items_per_level = vec![0; max_level];
        let mut start_of_fast_lanes = vec![0; max_level];
        let mut fast_lane_size = 16 as usize;

        max_items_per_level[max_level - 1] = fast_lane_size;
        start_of_fast_lanes[max_level - 1] = 0;

        for level in (max_level - 2)..0 {
            max_items_per_level[level] = max_items_per_level[level + 1] * skip;
            start_of_fast_lanes[level] = start_of_fast_lanes[level + 1] +
                                         max_items_per_level[level + 1];
            fast_lane_size += max_items_per_level[level];
        }

        // create the SkipList datastructure
        let mut result = SkipList {
            max_level: max_level,
            skip: if skip > 1 { skip } else { 2 },
            proxy_lane: Vec::with_capacity(fast_lane_size),
            fast_lanes: vec![u32::max_value(); fast_lane_size],
            nodes: Vec::<u32>::with_capacity(keys.len()),
            max_items_per_level: max_items_per_level,
            start_of_fast_lanes: start_of_fast_lanes,
            num_items_per_level: vec![0, max_level],
        };

        // sort the keys by inserting them into a binary heap
        let mut copyKeys = keys.to_vec();
        copyKeys.sort();

        // insert each key (in sorted order)    
        for k in copyKeys {
            result.insert_sorted_bulk(&k)
        }

        // return the result
        result
    }

    fn insert_sorted_bulk(&mut self, key: &u32) {
        self.nodes.push(key.clone());

        let mut anything_inserted = false;
        let mut insert_success = true;
        for l in 0..self.max_level {
            if (self.nodes.len() % self.skip.pow((l + 1) as u32)) == 0 && insert_success {
                insert_success = self.insert_fast_lane(l, key);
                anything_inserted = true;
            } else {
                break;
            }
        }

        if !anything_inserted {
            //TODO: insert into proxy lane
//            self.proxy_lane.insert()
        }

    }

    fn insert_fast_lane(&mut self, level: usize, key: &u32) -> bool {
        let fastlane_idx = self.start_of_fast_lanes[level] + self.num_items_per_level[level];
        let level_limit = self.start_of_fast_lanes[level] + self.max_items_per_level[level];

        false
    }

    pub fn get_nodes(&self) -> &[u32] {
        &self.nodes
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn insert_retrieve_sorted() {
        let vals = [2, 1, 3, 10, 0];
        let slist = super::SkipList::new(9, 5, &vals);

        let sorted = slist.get_nodes();
        
        assert_eq!(0, sorted[0]);
        assert_eq!(1, sorted[1]);
        assert_eq!(2, sorted[2]);
        assert_eq!(3, sorted[3]);
        assert_eq!(10, sorted[4]);

    }
}
