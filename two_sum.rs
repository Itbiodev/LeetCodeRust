struct Solution {}

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices: Vec<i32> = Vec::new();
        // y = n - x
        let mut hashtable = std::collections::HashMap::new();

        for (i, num) in nums.iter().enumerate() { // for x in nums 
            let key = target - num;
            if hashtable.contains_key(&key) { // if y is on the table
                indices.push(*hashtable.get(&key).unwrap());
                indices.push(i as i32);
            }
            hashtable.insert(num, i as i32);
        }
        indices
    }
}
fn main() {
    println!("{:?}", Solution::two_sum(vec![3,2,4], 6));
    println!("{:?}", Solution::two_sum(vec![2,7,11,15], 9));
}