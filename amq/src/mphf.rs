use boomphf::*;

use ahash::RandomState;

pub fn query_mphf(keys_new: &(Vec<String>, Vec<String>), phf: &Mphf<String>, expected_num_items: &u32) -> (i32, i32){
    // query mphf on known mismatches and known matches
    let mut fpr = 0;
    for s in keys_new.0.iter(){
        let hash = phf.try_hash(s);
        if hash!=None{
            if hash.unwrap() < *expected_num_items as u64{
                fpr+=1;
            }
        }    
    }
    let mut fng = 0;
    for s in keys_new.1.iter(){
        if phf.try_hash(s)==None{
            fng+=1;
        }
    }
    return (fpr, fng)
}

pub fn create_fpt(phf: &Mphf<String>, hash_builder: &RandomState, keys:  &Vec<String>, b: &usize) -> Vec<i32> {
    // create integer vector where i = Mphf(k) and A[i]=h(k)'s first b bits
    let mut fingerprint: Vec<i32> = vec![0; keys.len()];
    for key in keys.iter(){
        fingerprint[phf.hash(key) as usize] = get_bbits(hash_builder, key, b);  
    }
    return fingerprint
}

pub fn query_fpt(keys_new: &(Vec<String>, Vec<String>), fingerprint: &Vec<i32>, phf: &Mphf<String>, hash_builder: &RandomState, b: &usize, expected_num_items: &u32) -> (i32, i32){
    let mut fpr = 0;
    for s in keys_new.0.iter(){
        let mpfh_tmp =  phf.try_hash(s);
        if mpfh_tmp!=None{
            let mpfh_val = mpfh_tmp.unwrap();
            if mpfh_val< *expected_num_items as u64{
                // check if it matches the fingerprint array, then definitely false pos
                if fingerprint[mpfh_val as usize]==get_bbits(&hash_builder, s, &b) as i32 {
                    fpr+=1;
                }   
            }
        }
        
    }
    //check false neg
    let mut fng = 0;
    for s in keys_new.1.iter(){
        // if it's not in the hash or it is but fingerprint doesn't match
        let mpfh_tmp =  phf.try_hash(s);
        if mpfh_tmp!=None{
            let mpfh_val = mpfh_tmp.unwrap();
            if mpfh_val < *expected_num_items as u64{
                fng+=1;
            } else if fingerprint[mpfh_val as usize] != get_bbits(&hash_builder, s, &b) {
                fng+=1;
            }
        }
        
    }
    return (fpr, fng)
}

fn get_bbits(hash_builder:&RandomState, key:&String, b:&usize) -> i32{
    let hash_val = hash_builder.hash_one(key);
    let binval = format!("{:b}", hash_val).to_string();
    return binval[binval.len()-b..].to_string().parse::<i32>().unwrap(); 
}
