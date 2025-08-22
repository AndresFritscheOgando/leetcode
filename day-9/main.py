# Given an integer n, return any array containing n unique integers such that they add up to 0.

# Example 1:

# Input: n = 5
# Output: [-7,-1,1,3,4]
# Explanation: These arrays also are accepted [-5,-1,1,2,3] , [-3,-1,2,-2,4].

# Example 2:

# Input: n = 3
# Output: [-1,0,1]

# Example 3:

# Input: n = 1
# Output: [0]

 
# Constraints:
#     1 <= n <= 1000

def findTarget(nums,target):
    left = 0
    right = len(nums)

    for item in nums:
        if item == target:
            return item
        else:
            return -1




nums = [2,4,8,4,6, 9,1]
target = 8

print(findTarget(nums, target))