


fn main() {

    let matrix = vec![vec![-1,3]];
    let res = search_matrix(matrix,-1);
    println!("res={:?}", res);
}

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut x = 0;
    let mut y = matrix[0].len() - 1;
    while x < matrix.len() && y >= 0 && matrix[x][y as usize] != target {
        println!("x={},y={}",x,y);
        println!("matrix[x][y]={}",matrix[x][y as usize]);
        if matrix[x][y as usize] > target {
            if y == 0 {
                return false;
            }
            y -= 1;
        } else {
            if x == matrix.len() - 1 {
                return false;
            }
            x += 1;
        }
    }
    true
}
