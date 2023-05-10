use std::mem;
use std::time::{Instant,Duration};
use boomphf::*;
use ahash::RandomState;

// from src/
mod bloom_wrap;
mod mphf;
use amq::{make_k, make_kprime};
use crate::bloom_wrap::{make_filter, query_filter};
use crate::mphf::{query_mphf, create_fpt, query_fpt};


fn main() {
    // task1: wrapper to build bloom filter on set of keys and queries
    // test with various false positive reads and sets of keys
    // measure observed fpr, no false neg
    // time spent querying
    // total bloom filter size
    
    for expected_num_items in [1000, 5000, 10000, 50000]{
        println!("----------------------");
        println!("expected_num_items {}", expected_num_items);
        println!("----------------------");
        let keys = make_k(expected_num_items);        // generate random string of size 31
        let mut all_keys: Vec<(Vec<String>, Vec<String>)> = Vec::new();
        for mix in [0.1, 0.25, 0.5,]{
            // make testing key set
            all_keys.push(make_kprime(keys.clone(), mix));  
        }   
        
        // test bloom filter
        println!("testing bloom filter");
        // iterate through various fpr powers
        for fpr in [7,8,10]{
            // iterate through various mixes of pos/neg keys
            let false_positive_rate:f32 = 1./f32::powi(2., fpr);
            let filter = make_filter(keys.clone(), expected_num_items, false_positive_rate);     
            let filter_size =  mem::size_of_val(&filter);  //return size in bytes
            println!("fpr {}, bloom filter size in bytes {} ", fpr, filter_size);
            
            // iterate through various mix of pos/neg k' and test fpr, fng, and time
            for keys_new in all_keys.iter(){
                // time querying
                let now = Instant::now();
                let results = query_filter(keys_new, &filter);
                let duration: Duration = now.elapsed();
                println!("false positive: {} false negative: {}", results.0 as f64 /expected_num_items as f64, results.1 as f64 /expected_num_items as f64);
                println!("{:?}", duration);
            }
        }

        // test mpfh
        // create mpfh
        let phf = Mphf::new(1.7, &keys);
        let mphf_size = mem::size_of_val(&phf);
        println!("**********");
        println!("testing mpfh");
        println!("mphf size: {}", mphf_size);
        // iterate through various mix of pos/neg k' and test fpr, fng, and time
        for keys_new in all_keys.iter(){
            // time querying
            let now = Instant::now();
            let results = query_mphf(keys_new, &phf);
            let duration: Duration = now.elapsed();
            println!("false positive: {} false negative: {}", results.0 as f64 /expected_num_items as f64, results.1 as f64 /expected_num_items as f64);
            println!("mphf query {:?}", duration);
        }

        // test fingerprint array
        println!("**********");
        println!("testing fingerprint array");
        let b: usize = 7; //  8 or 10
        let hash_builder = RandomState::with_seed(42);
        // create integer vector where i = Mphf(k) and A[i]=h(k)'s first b bits
        let fingerprint = create_fpt(&phf, &hash_builder, &keys, &b);
        // size of array
        println!("fpt size {}", mem::size_of_val(&fingerprint));
    
        for keys_new in all_keys.iter(){
            // tim querying
            let now = Instant::now();
            let results = query_fpt(keys_new, &fingerprint, &phf, &hash_builder, &b);
            let duration: Duration = now.elapsed();
            println!("false positive: {} false negative: {}", results.0 as f64 /expected_num_items as f64, results.1 as f64 /expected_num_items as f64);
            println!("fpt array query {:?}", duration);
        }
    }
}
