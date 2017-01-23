extern crate cssl;

fn main() {
    let vals = [0, 1, 2, 3, 10, 20, 23, 24, 25, 26, 40, 400, 421, 422, 423];
    let slist = cssl::skiplist::SkipList::new(3, 2, &vals[0..vals.len()]);


    let r = slist.find_range(0,423);
    assert_eq!(true, r.is_some());
    let v = r.unwrap();
    println!("{}..{}", v.start, v.end);
    println!("--> {}..{}", vals[v.start], vals[v.end]);
}
