use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    iter::Sum,
    vec,
};

fn main() {
    let fruits = vec![
        "apple".to_string(),
        "banana".to_string(),
        "apple".to_string(),
        "pear".to_string(),
        "banana".to_string(),
        "apple".to_string(),
    ];
    let fruits_ref = vec!["apple", "banana", "apple", "pear", "banana", "apple"];

    let nums: Vec<i32> = vec![12, 1, 20, 15, 3];
    let nums1: Vec<i32> = vec![2, 3, 2];
    let nums2: Vec<i32> = vec![1, 2];

    // two_sum(vec![4, 7, 1, 3, 2], 5);
    // most_frequent_word(fruits);
    // smaller_numbers_than_current(nums);
    // iterator();
    // even_nums(nums);
    // println!("{:#?}", string_lengths(fruits));
    // println!("{:#?}", sum_greater_than_10(nums));
    // uppercase(fruits);
    // divisible_three_five(nums);
    // find_intersection_values(nums1, nums2);

    let mut my_hash_set: HashSet<Vec<i32>> = HashSet::new();
    println!("{:#?}", my_hash_set);
}

// Given an array of integers nums and an integer target, return the first pair of indices (i, j) such that:

// nums[i] + nums[j] == target
// i < j
// among all valid pairs, j is minimized
// if multiple pairs have the same j, minimize i

// Return (-1, -1) if no such pair exists.

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn two_sum(nums: Vec<i32>, target: i32) -> HashMap<i32, i32> {
    let mut result = HashMap::new();

    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] + nums[j] == target && i < j {
                result.insert(i as i32, j as i32);

                println!("{:#?}", result);
                return result;
            }
        }
    }

    result
}
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 2. Most Frequent Word
// Problem

// Given a vector of lowercase strings words, return the word that appears most frequently.

// If multiple words have the same highest frequency, return the lexicographically smallest one.

// Example
// Input:
// ["apple", "banana", "apple", "pear", "banana", "apple"]

// Output:
// "apple"
// Constraints
// 1 <= words.len() <= 10^5

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// pub fn most_frequent_word(words: Vec<String>) -> String {
//     let mut result = String::new();
//     let mut counter: HashMap<String, i32> = HashMap::new();
//     let mut frequency = 0;

//     for i in 0..words.len() {
//         let keys = words.t(i).unwrap().to_string();

//         counter.insert(keys, frequency);

//     }

//     result
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 1365. How Many Numbers Are Smaller Than the Current Number
// Easy
// Topics
// premium lock icon
// Companies
// Hint
// Given the array nums, for each nums[i] find out how many numbers in the array are smaller than it. That is, for each nums[i] you have to count the number of valid j's such that j != i and nums[j] < nums[i].

// Return the answer in an array.

// Example 1:

// Input: nums = [8,1,2,2,3]
// Output: [4,0,1,1,3]
// Explanation:
// For nums[0]=8 there exist four smaller numbers than it (1, 2, 2 and 3).
// For nums[1]=1 does not exist any smaller number than it.
// For nums[2]=2 there exist one smaller number than it (1).
// For nums[3]=2 there exist one smaller number than it (1).
// For nums[4]=3 there exist three smaller numbers than it (1, 2 and 2).
// Example 2:

// Input: nums = [6,5,4,8]
// Output: [2,1,0,3]
// Example 3:

// Input: nums = [7,7,7,7]
// Output: [0,0,0,0]

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut counter = 0;

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if j != i && nums[j] < nums[i] {
                counter += 1;
            }
        }
        result.push(counter);
        counter = 0;
    }

    result
}

// pub fn iterator() {
//     let fruits_list = vec!["strawberry", "blueberry", "mango", "orange", "apple"];

//     let mut fruit_iter = fruits_list.iter();

//     let item_01 = fruit_iter.next().unwrap();

//     println!("{:#?}", item_01)
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// Given Vec<i32>, create a new vector containing only the even numbers.

pub fn even_nums(nums: Vec<i32>) {
    let even: Vec<&i32> = nums.iter().filter(|e| *e % 2 == 0).collect();

    println!("{:#?}", even)
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// Given Vec<String>, return a vector of the lengths of each string.

pub fn string_lengths(fruits: Vec<String>) -> Vec<usize> {
    fruits.iter().map(|fruit| fruit.len()).collect()
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn sum_greater_than_10(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().filter(|num| **num > 10).sum();

    if sum == 0 {
        println!("none of the nums were greater than 0")
    }

    sum
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// Given Vec<&str>, create a Vec<String> where every word is uppercase.
fn uppercase(fruits: Vec<String>) -> Vec<String> {
    let uppercase_fruits: Vec<String> = fruits.iter().map(|fruit| fruit.to_uppercase()).collect();

    println!("{:#?}", uppercase_fruits);
    uppercase_fruits
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// Given Vec<i32>, find the first number divisible by both 3 and 5.

fn divisible_three_five(nums: Vec<i32>) -> i32 {
    let magic_number = nums.iter().find(|&&e| e % 3 == 0 || e % 5 == 0).unwrap();

    println!("{:#?}", magic_number);
    *magic_number
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//     2956. Find Common Elements Between Two Arrays
// Easy
// Topics
// premium lock icon
// Companies
// Hint
// You are given two integer arrays nums1 and nums2 of sizes n and m, respectively. Calculate the following values:

// answer1 : the number of indices i such that nums1[i] exists in nums2.
// answer2 : the number of indices i such that nums2[i] exists in nums1.
// Return [answer1,answer2].

// Example 1:

// Input: nums1 = [2,3,2], nums2 = [1,2]

// Output: [2,1]

// Explanation:

// Example 2:

// Input: nums1 = [4,3,2,3,1], nums2 = [2,2,5,2,3,6]

// Output: [3,4]

// Explanation:

// The elements at indices 1, 2, and 3 in nums1 exist in nums2 as well. So answer1 is 3.

// The elements at indices 0, 1, 3, and 4 in nums2 exist in nums1. So answer2 is 4.

// Example 3:

// Input: nums1 = [3,4,2,3], nums2 = [1,5]

// Output: [0,0]

// Explanation:

// No numbers are common between nums1 and nums2, so answer is [0,0].

pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for i in 0..nums1.len() {
        for j in 0..nums2.len() {
            if nums1[i] == nums2[j] && !result.contains(&nums1[i]) {
                result.push(nums1[i]);
            }
        }
    }

    result
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
