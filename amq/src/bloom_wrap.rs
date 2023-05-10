extern crate bloom;
use bloom::{BloomFilter};


pub fn make_filter(keys: Vec<String>, expected_num_items: u32, false_positive_rate: f32) -> BloomFilter{
    // initiate a bloom filter with given keys, size, and fpr
    let mut filter = BloomFilter::with_rate(false_positive_rate,expected_num_items);
    // insert into bloom filter
    for s in keys.iter() {
        filter.insert(&s);
    } 
    return filter
}

pub fn query_filter(keys_new: &(Vec<String>, Vec<String>), filter: &BloomFilter) -> (i32, i32) {
    // query bloom filter given new set of keys and some og keys and return fpr and fng
    let mut fpr = 0;
    for s in keys_new.0.iter() { //new key set
        if filter.contains(&s){
            fpr+=1;
        }
    }
    let mut fng = 0;
    for s in keys_new.1.iter() {
        // check no false negatives, breaks everything if there is
        if !filter.contains(&s){
            fng +=1
        }
    }
    return (fpr, fng)
}