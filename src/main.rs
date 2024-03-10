use ndarray::{arr2, Axis};

fn main() {
    // 创建一个二维矩阵
    let matrix = arr2(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);

    // 计算矩阵的秩
    let rank = matrix.rank(Axis(0)); // 按行计算秩
    println!("Matrix:\n{:?}", matrix);
    println!("Rank: {}", rank);
}


