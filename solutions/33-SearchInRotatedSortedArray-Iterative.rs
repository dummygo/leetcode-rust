/*
The solution leverages a modified form of Binary Search.

Motivation:
If we partition the array at any random element, one part of such an array must be sorted.

Consider an array being bound by two indices: LEFT and RIGHT.
LEFT is the index of the left-most element of the array.
RIGHT is the right-most element of the array.

Picking any random element MID in the range [LEFT, RIGHT] can tell us following:

1. If MID is equal to the target value, MID is our answer.
2. If element at LEFT is less than element at MID, the array in the range [LEFT, MID) is sorted. 
    (a) If so and target falls in the elements in this range, it is a simple case of classic binary search.
    (b) If not, the target can be in the remaining part of the array bound by [MID, RIGHT]
3. If (2) is false, the array in the range (MID, RIGHT] must be sorted.
    (a) Check if target falls in the array in index bound by range (MID, RIGHT]. Again, simple binary search.
    (b) If not, target can be in the left half of the remaining array bound by [LEFT, MID)


Complexity:
Since we're eliminating one-half of the array at each step as the algorithm proceeds, it is O(log n)

NOTE: THE ARRAY CONTAINING ALL UNIQUE ELEMENTS IS AN IMPORTANT PRE-CONDITION FOR THIS ALGORITHM. 

*/
impl Solution {

   // This is an iterative version.  
   pub fn search(num: Vec<i32>, target: i32) -> i32 {
    
    if num.len() == 0 {
        return -1;
    }

    let mut left:usize = 0;
    let mut right:usize = num.len() - 1;
    
    while left <= right {
        let mid = left + (right - left) / 2;
        if num[mid] == target {
            return mid as i32;
        } else if num[left] <= num[mid] {                           // Equals comparison is necessary for the case where mid converges to become equal to left
            if target < num[mid] && target >= num[left] {
                if mid == 0 { break; } else { right = mid - 1; }
            } else {
                left = mid + 1;
            }
        } else {
            if target > num[mid] && num[right] >= target {         
                left = mid + 1;
            } else {
                if mid == 0 { break; } else { right = mid - 1; } 
            }
        } 
    }
    return -1;
  }

}