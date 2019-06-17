/*
This approach uses memoization to solve the problem. Memoization is just a means to record a value already
calculated earlier which may be used later again. It is a classic approach to avoid recomputations in overlapping problems.

How is it an overlapping problem?
Consider this. The only way you can reach a step N is by taking two steps from step N -2 or one step from step N -1.
Simply put, if f(N) is the number of ways you can reach step N,

--------------------------
f(N) = f(N-1) + f(N-2)
--------------------------

Now, take the example of f(5).

f(5) = f(3) + f(4)                     (i)
              f(4) = f(3) + f(2)       (ii)

Notice that we require f(3) be calculated twice (once in each step). If we can somehow record the value of f(3) obtained in step (i),
we can reuse the same in step (ii). 

NOTICE THE RESEMBLANCE OF THE FUNCTION f(N) WITH THE FIBONACCI SEQUENCE.

Complexity Analysis:

Due to memoization, all we have to do is to compute f(N) for each value of N in the range (1, N). 
Time Complexity: O(N)

We're storing each result in a hashmap.
Space Complexity: O(N) (for hash of key) + O(N) (for value) = O(2N) = O(N) 

We're not considering the recursion stack as part of the space complexity. If we do, it will add another O(N) to it which will still be linear.
*/

use std::collections::HashMap;

impl Solution {

    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo:HashMap<u32, u32> = HashMap::new();
        return Solution::climb_stairs_recursive(n as u32, &mut memo) as i32;
    }

    fn climb_stairs_recursive(n: u32, memo: &mut HashMap<u32, u32>) -> u32 {
        if n <= 1 {
            return 1;
        } else if memo.contains_key(&n) {
            return memo[&n];
        }
        let result = Solution::climb_stairs_recursive(n - 1, memo) + Solution::climb_stairs_recursive(n - 2, memo);
        memo.insert(n, result);
        return result;
    }

}
