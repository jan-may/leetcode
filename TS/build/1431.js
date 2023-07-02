"use strict";
/**
 * @param {number[]} candies
 * @param {number} extraCandies
 * @return {boolean[]}
 */
var kidsWithCandies = function (candies, extraCandies) {
    var max = Math.max.apply(Math, candies);
    return candies.map(function (candy) { return candy + extraCandies >= max; });
};
console.log(kidsWithCandies([2, 3, 5, 1, 3], 3));
