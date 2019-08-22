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

}
