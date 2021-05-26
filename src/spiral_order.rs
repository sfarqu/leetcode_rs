/**
https://leetcode.com/problems/spiral-matrix/
Given an m x n matrix, return all elements of the matrix in spiral order.
Constraints:
    * m == matrix.length
    * n == matrix[i].length
    * 1 <= m, n <= 10
    * -100 <= matrix[i][j] <= 100
*/
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut max_x = matrix[0].len() - 1;
    let mut max_y = matrix.len() - 1;
    let mut min_x = 0;
    let mut min_y = 0;

    let mut return_value = Vec::with_capacity(max_x * max_y);

    while min_x <= max_x && min_y <= max_y {
        for col in min_x..=max_x {
            return_value.push(matrix[min_y][col]);
        }
        for row in min_y+1..=max_y {
            return_value.push(matrix[row][max_x]);
        }

        if min_x >= max_x || min_y >= max_y { break;}

        for col in (min_x+1..max_x).rev() {
            return_value.push(matrix[max_y][col]);
        }

        for row in (min_y+1..=max_y).rev() {
            return_value.push(matrix[row][min_x]);
        }
        min_x += 1;
        min_y += 1;
        max_x -= 1;
        max_y -= 1;
    }
    return_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let test_data = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        assert_eq!(spiral_order(test_data),vec![1,2,3,6,9,8,7,4,5]);
    }

    #[test]
    fn test_single_element() {
        let test_data = vec![vec![1]];
        assert_eq!(spiral_order(test_data),vec![1]);
    }

    #[test]
    fn test_single_row() {
        let test_data = vec![vec![1,2,3,4]];
        assert_eq!(spiral_order(test_data),vec![1,2,3,4]);
    }

    #[test]
    fn test_wider_matrix() {
        let test_data = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]];
        assert_eq!(spiral_order(test_data),vec![1,2,3,4,8,12,11,10,9,5,6,7]);
    }

    #[test]
    fn test_longer_matrix() {
        let test_data = vec![vec![1,2,3]
                                         ,vec![4,5,6]
                                         ,vec![7,8,9]
                                         ,vec![10,11,12]
                                         ,vec![13,14,15]];
        assert_eq!(spiral_order(test_data),vec![1,2,3,6,9,12,15,14,13,10,7,4,5,8,11]);
    }

    #[test]
    fn test_larger_matrix() {
        let test_data = vec![vec![1,2,3,4,5]
                             ,vec![6,7,8,9,10]
                             ,vec![11,12,13,14,15]
                             ,vec![16,17,18,19,20]
                             ,vec![21,22,23,24,25]];
        assert_eq!(spiral_order(test_data),
                   vec![1,2,3,4,5,10,15,20,25,24,23,22,21,16,11,6,7,8,9,14,19,18,17,12,13]);
    }
}