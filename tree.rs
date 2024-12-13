fn trap(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }

    let mut left = 0;
    let mut right = height.len() - 1;
    let mut left_max = 0;
    let mut right_max = 0;
    let mut water = 0;

    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                water += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                water += right_max - height[right];
            }
            right -= 1;
        }
    }

    water
}

// Example usage:
fn main() {
    let heights = vec![4, 0, 0, 4];
    println!("Trapped water: {}", trap(heights)); // Should print 6
}
