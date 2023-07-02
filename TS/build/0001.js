"use strict";
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function (nums, target) {
    for (var i = 0; i < nums.length; i++) {
        for (var j = nums.length - 1; j > i; j--) {
            if (nums[i] + nums[j] === target) {
                return [i, j];
            }
        }
    }
};
console.log(twoSum([2, 7, 11, 15], 9));
