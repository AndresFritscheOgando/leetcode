"use strict";
function findDuplicate(nums) {
    nums.sort((a, b) => a - b);
    let duplicateNumber = 0;
    let left = 0;
    let right = nums.length - 1;
    while (left < right) {
        if (nums[left] === nums[right]) {
            duplicateNumber += nums[left];
            return duplicateNumber;
        }
        else {
            left++;
            right--;
        }
    }
    return duplicateNumber;
}
