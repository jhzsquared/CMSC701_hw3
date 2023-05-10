use rand::{distributions::Alphanumeric, Rng};

pub fn make_k(n: u32) -> Vec<String>{
     // generate n random alphanumeric strings of size 31
    let mut keys: Vec<String> = Vec::new();
    for _n in 0..n{
        keys.push(rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(31)
                        .map(char::from)
                        .collect());
    }
    return keys;
}

pub fn make_kprime(keys: Vec<String>, mix: f32) -> (Vec<String>, Vec<String>){
    // create new list of keys that are unique, include some mix % of old keys
    let mut keys_new: Vec<String> = Vec::new();
    let overlap_size = (mix*keys.len() as f32).floor() as u32;
    let mut count = 0;
    // make more keys
    while count < keys.len() as u32 - overlap_size {
        let s: String = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(31)
                        .map(char::from)
                        .collect();
        if !keys.contains(&s){
            //make sure it's not in the original key set before addings
            keys_new.push(s);
            count +=1;
        }
    }  
    // add some of og keys values keys[0..(mix*expected_num_items as f32).floor() as i32];
    let slice: Vec<String> = keys[0..overlap_size as usize].to_vec();
    return (keys_new, slice); 
}