extern crate cssl;

fn main() {
    let vals = [2, 1, 3, 10, 0];
    let slist = cssl::skiplist::SkipList::new(9, 5, &vals);
    let sorted = slist.get_nodes();
    println!("{}", sorted[0]);
}