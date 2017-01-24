extern crate cssl;
extern crate rand;
extern crate time;

use std::env;
use rand::Rng;


fn main() {

    let mut rng = rand::thread_rng();

    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();

    if args.len() != 3 {
        println!("Usage: {} <num_elements> 0|1 (0=dense, 1=sparse)", args[0]);
        std::process::exit(1);
    }

    let num_elements = args[1].parse::<usize>().unwrap();

    let mut keys = Vec::<u32>::with_capacity(num_elements);

    if args[2].parse::<usize>().unwrap() == 0 {
        // dense
        for i in 0..num_elements {
            keys.push((i as u32) + 1);
        }
    } else {
        // sparse (random)
        for _ in 0..num_elements {
            keys.push(rng.next_u32() % ((32767/2-1) + 1));
        }
        keys.sort();
    }
    
    let start_time_insert = time::precise_time_s();
    let slist = cssl::skiplist::SkipList::new(9, 5, &keys);
    let end_time_insert = time::precise_time_s();
    println!("Insertion: {} ops/s.", ((num_elements as f64) / (end_time_insert - start_time_insert)) as u64 );

    let mut random_keys = keys.clone();
    rng.shuffle(&mut random_keys[..]);

    let repeat=100000000/num_elements;
    
    let start_time_lookup = time::precise_time_s();
    
    for _ in 0..repeat {
        for k in &random_keys[..] {
            let found = slist.find(*k);
            assert_eq!(keys[found.unwrap()], *k);
        }
    }
    let end_time_lookup = time::precise_time_s();
    println!("Lookup:    {} ops/s.", ((num_elements as f64) / (end_time_lookup - start_time_lookup)) as u64 ) ;
    
    let m = 1000000;
    let mut range_keys = Vec::<u32>::with_capacity(m);
    for _ in 0..m {
        range_keys.push(rng.next_u32() % ( num_elements as u32));
    }
    let r_size = (num_elements / 10) as u32;
    let start_time_range = time::precise_time_s();
    for k in &range_keys[..] {
        let found = slist.find_range(*k, *k + r_size);
        if found.is_some() {
            let found_range = found.unwrap();
            assert!(keys[found_range.start] >= *k && keys[found_range.end-1] <= (*k+r_size));
        }
    }
    let end_time_range = time::precise_time_s();
    println!("Range:     {} ops/s.", ((num_elements as f64) / (end_time_range - start_time_range)) as u64);
}
