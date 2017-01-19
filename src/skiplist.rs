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

const MAX_SKIP: usize = 5;

struct ProxyNode {
    keys: Vec<u32>,
}

pub struct SkipList {
    max_level: usize,
    skip: usize,
    proxy_lane: Vec<ProxyNode>,
    linearized_fast_lanes: Vec<u32>,
    level_start_pos: Vec<usize>,
    level_num_items: Vec<usize>,
    nodes: Vec<u32>,
}


fn binary_search(key: u32, lane: &[u32]) -> usize {
    let mut first = 0;
    let mut last = lane.len();

    while first <= last {
        let middle = (first + last) / 2;
        if lane[middle] < key {
            first = middle + 1;
        } else if lane[middle] == key {
            return middle;
        } else {
            last = middle - 1;
        }
    }

    // exact key not found, return the nearest starting point
    if first > last {
        return last
    } else {
        return 0
    }
}

impl SkipList {
    pub fn new(max_level: usize, skip: usize, keys: &[u32]) -> SkipList {

        // sort the keys by inserting them into a binary heap
        let mut copy_keys = keys.to_vec();
        copy_keys.sort();

        let filtered_skip = if skip > MAX_SKIP {MAX_SKIP} else if skip < 2 {2} else {skip};

        // build the initial separated fast lanes
        let mut fast_lane_buffer = vec![Vec::<u32>::new(); max_level];
        let mut nodes = Vec::<u32>::new();
        let mut proxy_lane = Vec::<ProxyNode>::new();

        // insert each key into the corresponding fast lanes
        let mut current_proxy = ProxyNode {keys: Vec::<u32>::with_capacity(filtered_skip)};

        for k in copy_keys {
            nodes.push(k);
            for l in 0..max_level {
                if (nodes.len() % filtered_skip.pow((l + 1) as u32)) == 0 {
                    fast_lane_buffer[l].push(k);
                } else {
                    break;
                }
            }
            // insert into proxy lane            
            current_proxy.keys.push(k);
            if nodes.len() % filtered_skip == (filtered_skip-1) {
                proxy_lane.push(current_proxy);
                current_proxy = ProxyNode {keys: Vec::<u32>::with_capacity(filtered_skip)};
            }
        }
        // add the last proxy as well
        proxy_lane.push(current_proxy);

        let mut level_start_pos = vec![0; max_level];
        let mut level_num_items = vec![0; max_level];
        // linearize all the fast lanes
        let mut linearized_fast_lanes = Vec::<u32>::new();
        for level in max_level..0 {
            level_start_pos[level] = linearized_fast_lanes.len();
            level_num_items[level] = fast_lane_buffer[level].len();
            linearized_fast_lanes.append(&mut fast_lane_buffer[level]);
        }


        // create the SkipList datastructure
        return SkipList {
            max_level: max_level,
            skip: filtered_skip,
            proxy_lane: proxy_lane,
            linearized_fast_lanes: linearized_fast_lanes,
            nodes: nodes,
            level_start_pos: level_start_pos,
            level_num_items: level_num_items,
        };
    }


    pub fn get_nodes(&self) -> &[u32] {
        return &self.nodes
    }


    pub fn find(&self, key: u32) -> Option<usize> {
        
        // binary search for the starting position in the top lane
        let top_start_pos = self.level_start_pos[0];
        let top_end_pos = top_start_pos + self.level_num_items[0];
        let top_lane = &self.linearized_fast_lanes[top_start_pos .. top_end_pos];
        let mut pos = top_start_pos + binary_search(key, top_lane);
        
        for level in self.max_level-1..0 {
            let mut relative_pos = pos - self.level_start_pos[level];
            while key >= self.linearized_fast_lanes[pos+1] {
                pos += 1;
                relative_pos += 1;
            }
            if level == 0 {break;}
            // reset the pos to the next level offset
            pos = self.level_start_pos[level-1] + self.skip * relative_pos; 
        }
        let relative_pos = pos - self.level_start_pos[0];
        if key == self.linearized_fast_lanes[pos] {
            // the key is directly included in  the level 1 fast lane, calculate the position of the key in the original list
            return Some(self.skip * relative_pos);
        };
        // get the proxy node and find the key inside it 
        let proxy = &self.proxy_lane[pos - self.level_start_pos[0]];
        for i in 1..proxy.keys.len() {
            if key == proxy.keys[i] {
                return Some((self.skip * relative_pos) + i);
            }
        }
        return None;
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
