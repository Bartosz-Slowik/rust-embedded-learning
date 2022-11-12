use std::clone;
use std::ops::Add;
use std::ops::Mul;

struct Matrix<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

trait Zero {
    fn zero() -> Self;
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
}

impl<T: Clone + Zero> Matrix<T> {
    pub fn zeros(width: usize, height: usize) -> Self {
        Matrix {
            data: (vec![Zero::zero(); width*height]),
            width: (width),
            height: (height),
        }
    }
}

impl<T> Matrix<T> {
    pub fn from_arrays<const n: usize, const m: usize>(arr: [[T; m]; n]) {
        let mut data: Vec<T> = Vec::with_capacity(n * m);
        for x in arr.into_iter().map(|x| x.into_iter()).flatten() {
            data.push(x)
        }
    }
    // constructor that fills matrix with zeros

    // get reference to single value of matrix
    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.data[i * j]
    }
    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.data[i * j]
    }
    // get a slice of a row at index i
    pub fn row(&self, i: usize) -> &[T] {
        let j = &self.width;
        &self.data[i * j..i * j + j]
    }
    pub fn row_mut(&mut self, i: usize) -> &mut [T] {
        let j = self.width;
        &mut self.data[i * j..i * j + j]
    }
}
impl<T> Matrix<T>
where
    T: Clone + Copy + Zero + Add<Output = T>,


{
    pub fn add(&self, other: &Self) -> Self {
        let mut result = Matrix::zeros(self.height, self.width);
        for i in 0..self.width * self.height {
            result.data[i] = self.data[i] + other.data[i];
        }
        result
    }
}
impl<T> Matrix<T>
where
    T: Clone + Copy + Zero  + Mul<Output = T>,
{
    pub fn mul(&self, other: &Self) -> Self {
        let mut result = Matrix::zeros(self.height, self.width);
        for i in 0..self.width * self.height {
            result.data[i] = self.data[i] * other.data[i];
        }
        result
    }
}
fn main() {
    let mut matrix: Matrix<f64> = Matrix::zeros(4, 4);

    println!("get!");
    println!("{}", Matrix::get(&matrix, 2, 2));
    println!("get mut");
    println!("{}", Matrix::get_mut(&mut matrix, 2, 2));

    let mymat1 = Matrix {
        data: vec![3.0; 4 * 4],
        width: 2,
        height: 4,
    };
    let mymat2 = Matrix {
        data: vec![2.0; 4 * 4],
        width: 4,
        height: 2,
    };
    Matrix::from_arrays([[0., 0., 0.], [1., 1., 1.]]);

    // dbg!(Matrix::add(&mymat1,&mymat2).data);
    dbg!(Matrix::add(&mymat1, &mymat2).data);
}
