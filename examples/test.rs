use std::{collections::HashMap, vec};


fn main() {

    let mut matrix = vec![vec![1],vec![3]];
    let res = search_matrix(matrix,3);
    println!("res={:?}", res);
}

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    for nums in matrix {
        if nums.binary_search(&target).is_ok(){
            return true;
        }
    }
    false

}
