// check if one pair overlaps other pair 
// pairs look like 2-8, 3-7
// 2-8 -> 2345678 || 3-7 -> 34567 || 2-8 overlaps 3-7

use std::fs;

pub fn overlap(input: &str) -> String {
    let mut res = 0;
    
    for line in input.split("\n") {
        let v: Vec<&str> = line.split(",").collect();
        let nums1: Vec<&str> = v[0].split("-").collect();
        let nums2: Vec<&str> = v[1].split("-").collect();

        if nums1[0].parse::<u32>().unwrap() <= nums2[0].parse::<u32>().unwrap() && nums1[1].parse::<u32>().unwrap() >= nums2[1].parse::<u32>().unwrap() {
            res += 1;
        } else if nums1[0].parse::<u32>().unwrap() >= nums2[0].parse::<u32>().unwrap() && nums1[1].parse::<u32>().unwrap() <= nums2[1].parse::<u32>().unwrap() {
            res += 1;
        }
    }

    res.to_string()
}

pub fn overlap2(input: &str) -> String {
    let mut res = 0;
    
    for line in input.split("\n") {
        let v: Vec<&str> = line.split(",").collect();
        let pair1: Vec<&str> = v[0].split("-").collect();
        let pair2: Vec<&str> = v[1].split("-").collect();
        
        let mut nums1: Vec<String> = Vec::new();
        let mut nums2: Vec<String> = Vec::new();

        for i in pair1[0].parse::<u32>().unwrap()..pair1[1].parse::<u32>().unwrap()+1 {
            nums1.push(i.to_string());
        }

        for i in pair2[0].parse::<u32>().unwrap()..pair2[1].parse::<u32>().unwrap()+1 {
            nums2.push(i.to_string());
        }

        'inner:for num in nums1 {
            if nums2.contains(&num) {
                res += 1;
                break 'inner;
            }
        }

    }
    res.to_string()
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("{}", overlap2(&file));
}
