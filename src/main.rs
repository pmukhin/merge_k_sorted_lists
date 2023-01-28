#![feature(test)]

use linkedlist::LNode;

mod qsort;
mod linkedlist;
mod dict;

fn main() {
    let mut arr = vec![0,9,8,7,6,5,4,3,2,1];
    let last_idx = arr.len()-1;
    qsort::qsort(&mut arr, 0, last_idx);

    let int_list = LNode::from(&arr);

    let stringified_list = int_list.map(|v| { 
        let mut s = v.to_string();
        s.push_str("_asstr");
        s
    });

    println!("tree: {:?}", stringified_list);
    println!("tree: {:?}", stringified_list.clone());
}
