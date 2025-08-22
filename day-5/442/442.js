"use strict";
// 442. Find All Duplicates in an Array
/*
Given an integer array nums of length n where all the integers of nums are in the range [1, n] and each integer appears at most twice, return an array of all the integers that appears twice.

You must write an algorithm that runs in O(n) time and uses only constant auxiliary space, excluding the space needed to store the output

 

Example 1:

Input: nums = [4,3,2,7,8,2,3,1]
Output: [2,3]
Example 2:

Input: nums = [1,1,2]
Output: [1]
Example 3:

Input: nums = [1]
Output: []
 

Constraints:

n == nums.length
1 <= n <= 105
1 <= nums[i] <= n
Each element in nums appears once or twice.
*/
function findDuplicates(nums) {
    const sortedNums = [...nums].sort((a, b) => a - b);
    const duplicates = [];
    for (let i = 1; i < sortedNums.length; i++) {
        if (sortedNums[i] === sortedNums[i - 1]) {
            duplicates.push(sortedNums[i]);
        }
    }
    return duplicates;
}
console.log(findDuplicates([4, 3, 2, 7, 8, 2, 3, 1]));
