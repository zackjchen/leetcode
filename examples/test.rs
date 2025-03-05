use std::{collections::HashMap, vec};


fn main() {

    let mut matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9],];
    rotate(&mut matrix);
    println!("res={:?}", matrix);
}
///  
/// a[j][n-i-1] 应该放 a[i][j] ,用temp暂存 a[j][n-i-1]
/// a[n-i-1][n-j-1] 应该放 a[j][n-i-1] ,用temp暂存 a[n-i-1][n-j-1]
/// a[n-j-1][i]应该放 a[n-i-1][n-j-1] ,用temp暂存 a[n-j-1][i]
/// a[i][j]应该放 a[n-j-1][i] ,用temp暂存 a[i][j]
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for i in 0..n/2{
        for j in 0..(n+1)/2{
            let temp = matrix[i][j];
            matrix[i][j] = matrix[n-j-1][i];
            matrix[n-j-1][i] = matrix[n-i-1][n-j-1];
            matrix[n-i-1][n-j-1] = matrix[j][n-i-1];
            matrix[j][n-i-1] = temp;
        }
    }
}

