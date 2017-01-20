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
const MIN_FAST_LANE_SIZE: usize = 16;

struct ProxyNode {
    keys: Vec<u32>,
}

pub struct SkipList {
    max_level: usize,
    skip: usize,
    proxy_lane: Vec<ProxyNode>,
    linearized_fast_lanes: Vec<u32>,
    fast_lane_start: Vec<usize>,
    fast_lane_end: Vec<usize>,
    nodes: Vec<u32>,
}


fn binary_search(key: u32, lane: &[u32]) -> usize {
    let mut first = 0;
    let mut last = lane.len() - 1;

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
    if first > last { return last } else { return 0 }
}

impl SkipList {
    pub fn new(max_level: usize, skip: usize, sorted_keys: &[u32]) -> SkipList {


        let filtered_skip = if skip > MAX_SKIP {
            MAX_SKIP
        } else if skip < 2 {
            2
        } else {
            skip
        };

        // build the initial separated fast lanes
        let mut fast_lanes = vec![Vec::<u32>::new(); max_level];
        let mut nodes = Vec::<u32>::new();
        let mut proxy_lane = Vec::<ProxyNode>::new();

        // insert each key into the corresponding fast lanes
        let mut current_proxy = ProxyNode { keys: Vec::<u32>::with_capacity(filtered_skip) };

        for k in sorted_keys {
            let idx_item = nodes.len();
            nodes.push(k.clone());
            for level in 0..max_level {
                if (idx_item % filtered_skip.pow((level + 1) as u32)) == 0 {
                    fast_lanes[level].push(k.clone());
                } else {
                    break;
                }
            }
            // insert into proxy lane
            current_proxy.keys.push(k.clone());
            if idx_item % filtered_skip == (filtered_skip - 1) {
                proxy_lane.push(current_proxy);
                current_proxy = ProxyNode { keys: Vec::<u32>::with_capacity(filtered_skip) };
            }
        }

        // make sure each vector is a multiple of MIN_FAST_LANE_SIZE
        let mut sum_fast_line_size = 0;
        for i in 0..fast_lanes.len() {
            let lane = &mut fast_lanes[i];
            let modulo = lane.len() % MIN_FAST_LANE_SIZE;
            if modulo > 0 {
                for _ in 0..(MIN_FAST_LANE_SIZE - modulo) {
                    lane.push(u32::max_value())
                }
            }
            sum_fast_line_size += lane.len();
        }

        // add the last proxy as well
        proxy_lane.push(current_proxy);

        // linearize the fast lanes into one array
        let mut linearized_fast_lanes = Vec::<u32>::with_capacity(sum_fast_line_size);
        let mut fast_lane_start = vec![0; max_level];
        let mut fast_lane_end = vec![0; max_level];

        for i in 0..fast_lanes.len() {
            fast_lane_start[i] = linearized_fast_lanes.len();
            linearized_fast_lanes.append(&mut fast_lanes[i]);
            fast_lane_end[i] = linearized_fast_lanes.len();
        }

        // create the SkipList datastructure
        return SkipList {
            max_level: max_level,
            skip: filtered_skip,
            proxy_lane: proxy_lane,
            linearized_fast_lanes: linearized_fast_lanes,
            fast_lane_start: fast_lane_start,
            fast_lane_end: fast_lane_end,
            nodes: nodes,
        };
    }

    pub fn find(&self, key: u32) -> Option<usize> {

        // binary search for the starting position in the top lane
        let top_lane = &self.linearized_fast_lanes[self.fast_lane_start[self.max_level - 1]..self.fast_lane_end[self.max_level - 1]];
        let mut pos = binary_search(key, top_lane);

        for level in (0..(self.max_level - 1)).rev() {
            pos = self.skip * pos;
            let lane =  &self.linearized_fast_lanes[self.fast_lane_start[level]..self.fast_lane_end[level]];
            while pos < lane.len() && key >= lane[pos + 1] {
                pos += 1;
            }
        }

        let bottom_lane = &self.linearized_fast_lanes[self.fast_lane_start[0]..self.fast_lane_end[0]];
        if key == bottom_lane[pos] {
            // the key is directly included in  the level 1 fast lane, calculate the position of the key in the original list
            return Some(self.skip * pos);
        }
        // get the proxy node and find the key inside it
        let proxy = &self.proxy_lane[pos];
        for i in 1..proxy.keys.len() {
            if key == proxy.keys[i] {
                return Some((self.skip * pos) + i);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn find_single() {
        let sorted = [0, 1, 2, 3, 10, 20, 23, 24, 25, 26, 40, 400, 421, 422, 423];
        let slist = super::SkipList::new(3, 2, &sorted);

        for idx in 0..sorted.len() {
            assert_eq!(Some(idx), slist.find(sorted[idx]));
        }
    }
}
