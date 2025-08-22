"use strict";
/*
Given an integer array nums, move all the even integers at the beginning
of the array followed by all the odd integers.

Return any array that satisfies this condition.

 

Example 1:

Input: nums = [3,1,2,4]
Output: [2,4,3,1]
Explanation: The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.

Example 2:

Input: nums = [0]
Output: [0]
 
*/
/*
Intuition:

Create 2 arrays, 1 for pair numbers and 1 for the odd numbers, then
we iterate for each number, if arr[i] is even, push it to the even array,
else push odd numbers to the odd array, create a variable sortedArray
to hold the concatenation of even and odd arrays, then return the sortedArray
*/
function sortArrayByParity(nums) {
    let pairArray = [];
    let oddArray = [];
    for (let i = 0; i < nums.length; i++) {
        if (nums[i] % 2 === 0) {
            pairArray.push(nums[i]);
        }
        else {
            oddArray.push(nums[i]);
        }
    }
    let sortedArray = pairArray.concat(oddArray);
    return sortedArray;
}
;
