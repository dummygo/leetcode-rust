impl Solution {
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
        } else if num[left] <= num[mid] {
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