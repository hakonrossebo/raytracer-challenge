use crate::tuple::Tuple;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub struct Matrix {
    pub dimensions: usize,
    elements: Vec<f64>,
}

impl Matrix {
    pub fn from_vector(d: usize, e: &[f64]) -> Matrix {
        Matrix {
            dimensions: d,
            elements: e.to_vec(),
        }
    }

    pub fn at(&self, r: usize, c: usize) -> f64 {
        let pos = self.dimensions * r + c;
        self.elements[pos]
    }

    pub fn identity() -> Matrix {
        let vector = [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        Matrix::from_vector(4, &vector)
    }

    pub fn transpose(&self) -> Matrix {
        let mut new_vec: Vec<f64> = Vec::with_capacity(self.dimensions * self.dimensions);
        for r in 0..self.dimensions {
            for c in 0..self.dimensions {
                new_vec.push(self.at(c, r));
            }
        }
        Matrix::from_vector(self.dimensions, &new_vec)
    }

    pub fn determinant(&self) -> f64 {
        assert!(self.dimensions == 2);
        self.elements[0] * self.elements[3] - self.elements[1] * self.elements[2]
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut new_vec: Vec<f64> =
            Vec::with_capacity((self.dimensions - 1) * (self.dimensions - 1));
        for r in 0..self.dimensions {
            for c in 0..self.dimensions {
                if r != row && c != col {
                    new_vec.push(self.at(r, c));
                }
            }
        }
        Matrix::from_vector(self.dimensions - 1, &new_vec)
    }
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let m = self.minor(row, col);
        if (row + col) % 2 == 0 {
            m
        } else {
            -m
        }
    }
}
impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        let eps = 1e-6;
        let equal_elements = |a: &[f64], b: &[f64]| -> bool {
            for (x, y) in a.iter().zip(b.iter()) {
                if (x - y).abs() >= eps {
                    return false;
                }
            }
            true
        };
        self.dimensions == other.dimensions && equal_elements(&self.elements, &other.elements)
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Matrix {
        assert!(self.dimensions == other.dimensions);
        let dot = |row: usize, col: usize| -> f64 {
            (0..self.dimensions)
                .map(|n| self.at(row, n) * other.at(n, col))
                .sum()
        };

        let mut new_vec: Vec<f64> = Vec::new();
        for row in 0..self.dimensions {
            for col in 0..self.dimensions {
                new_vec.push(dot(row, col));
            }
        }
        Matrix {
            dimensions: self.dimensions,
            elements: new_vec,
        }
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, other: Tuple) -> Tuple {
        assert!(self.dimensions == 4);
        let dot = |row: usize| -> f64 {
            self.at(row, 0) * other.0
                + self.at(row, 1) * other.1
                + self.at(row, 2) * other.2
                + self.at(row, 3) * other.3
        };

        let mut new_vec: Vec<f64> = Vec::new();
        for row in 0..self.dimensions {
            new_vec.push(dot(row));
        }
        Tuple(new_vec[0], new_vec[1], new_vec[2], new_vec[3])
    }
}
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let v = vec![
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        ];
        let m = Matrix::from_vector(4, &v);
        assert_eq!(m.at(0, 0), 1.0);
        assert_eq!(m.at(0, 3), 4.0);
        assert_eq!(m.at(1, 0), 5.5);
        assert_eq!(m.at(1, 2), 7.5);
        assert_eq!(m.at(2, 2), 11.0);
        assert_eq!(m.at(3, 0), 13.5);
        assert_eq!(m.at(3, 2), 15.5);
    }
    #[test]
    fn constructing_and_inspecting_a_2x2_matrix() {
        let v = vec![-3.0, 5.0, 1.0, -2.0];
        let m = Matrix::from_vector(2, &v);
        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(0, 1), 5.0);
        assert_eq!(m.at(1, 0), 1.0);
        assert_eq!(m.at(1, 1), -2.0);
    }
    #[test]
    fn constructing_and_inspecting_a_3x3_matrix() {
        let v = vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0];
        let m = Matrix::from_vector(3, &v);
        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(1, 1), -2.0);
        assert_eq!(m.at(2, 2), 1.0);
    }
    #[test]
    fn matrix_equality_with_identical_matrices() {
        let v = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ];
        let m = Matrix::from_vector(4, &v);
        let m2 = Matrix::from_vector(4, &v);
        assert_eq!(m, m2);
    }
    #[test]
    fn matrix_equality_with_different_matrices() {
        let v1 = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ];
        let v2 = vec![
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        ];

        let m1 = Matrix::from_vector(4, &v1);
        let m2 = Matrix::from_vector(4, &v2);

        assert_ne!(m1, m2);
    }
    #[test]
    fn multiplying_two_matrices() {
        let avec = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ];
        let a = Matrix::from_vector(4, &avec);
        let bvec = vec![
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        ];
        let b = Matrix::from_vector(4, &bvec);
        let evec = vec![
            20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0,
            46.0, 42.0,
        ];
        let expected = Matrix::from_vector(4, &evec);

        assert_eq!(expected, a * b);
    }
    #[test]
    fn multiplying_matrix_by_tuple() {
        let avec = vec![
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ];
        let a = Matrix::from_vector(4, &avec);
        let b = Tuple(1.0, 2.0, 3.0, 1.0);
        let expected = Tuple(18.0, 24.0, 33.0, 1.0);
        let actual = a * b;
        assert_eq!(expected, actual);
    }
    #[test]
    fn multiplying_matrix_by_the_identity_matrix() {
        let avec = vec![
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
        ];
        let a = Matrix::from_vector(4, &avec);
        let expected = Matrix::from_vector(4, &avec);
        let actual = a * Matrix::identity();
        assert_eq!(expected, actual);
    }
    #[test]
    fn transposing_a_matrix() {
        let v1 = vec![
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        ];
        let a = Matrix::from_vector(4, &v1);
        let v2 = vec![
            0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
        ];
        let expected = Matrix::from_vector(4, &v2);
        let actual = a.transpose();

        assert_eq!(expected, actual);
    }
    #[test]
    fn transposing_the_identity_matrix() {
        let actual = Matrix::identity().transpose();
        assert_eq!(Matrix::identity(), actual);
    }
    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let v = vec![1.0, 5.0, -3.0, 2.0];
        let a = Matrix::from_vector(2, &v);
        assert_eq!(17.0, a.determinant());
    }
    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let va = vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0];
        let a = Matrix::from_vector(3, &va);
        let vasub = vec![-3.0, 2.0, 0.0, 6.0];
        let expected = Matrix::from_vector(2, &vasub);
        assert_eq!(expected, a.submatrix(0, 2));
    }
    #[test]
    fn a_submatrix_of_4x4_matrix_is_3x3_matrix() {
        let va = vec![
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
        ];
        let a = Matrix::from_vector(4, &va);

        let vasub = vec![-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0];
        let expected = Matrix::from_vector(3, &vasub);

        assert_eq!(expected, a.submatrix(2, 1));
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let va = vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0];
        let a = Matrix::from_vector(3, &va);
        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), a.minor(1, 0));
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let va = vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0];
        let a = Matrix::from_vector(3, &va);
        assert_eq!(-12.0, a.minor(0, 0));
        assert_eq!(-12.0, a.cofactor(0, 0));
        assert_eq!(25.0, a.minor(1, 0));
        assert_eq!(-25.0, a.cofactor(1, 0));
    }
}
