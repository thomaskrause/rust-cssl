extern crate cssl;

fn main() {
    let vals = [2, 1, 3, 10, 0];
    let slist = cssl::skiplist::SkipList::new(3, 2, &vals);

    let r = slist.find(2);
    assert_eq!(true, r.is_some());
    println!("{}", r.unwrap());
}