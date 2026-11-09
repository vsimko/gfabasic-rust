//! Checked floating-point field operations from the Amiga GFA BASIC 3.5 addendum.
//!
//! The addendum documents MAT CLR and MAT SET for previously dimensioned
//! one- or two-dimensional floating-point fields, and MAT ONE for square
//! floating-point matrices (Chapter 13, printed pp. 13-2–13-3).
//!
//! This module provides the storage-independent operation semantics. It does
//! not define the original interpreter's array layout, numeric representation,
//! or runtime error codes. Shape errors below are Rust API validation errors.

use std::error::Error;
use std::fmt;

/// A checked mutable view over a row-major floating-point field.
pub struct MatrixMut<'a> {
    rows: usize,
    columns: usize,
    elements: &'a mut [f64],
}

/// Errors detected while constructing or operating on a matrix view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixShapeError {
    /// The requested row/column product cannot be represented by `usize`.
    DimensionOverflow,
    /// The backing slice does not contain exactly rows × columns elements.
    ElementCountMismatch { expected: usize, actual: usize },
    /// MAT ONE is defined by the source for square matrices only.
    IdentityRequiresSquare { rows: usize, columns: usize },
}

impl fmt::Display for MatrixShapeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionOverflow => write!(formatter, "matrix dimensions overflow usize"),
            Self::ElementCountMismatch { expected, actual } => write!(
                formatter,
                "matrix requires {expected} elements, but storage has {actual}"
            ),
            Self::IdentityRequiresSquare { rows, columns } => write!(
                formatter,
                "identity matrix requires a square shape, got {rows}x{columns}"
            ),
        }
    }
}

impl Error for MatrixShapeError {}

impl<'a> MatrixMut<'a> {
    /// Create a checked row-major view over a previously allocated field.
    ///
    /// The backing slice must contain exactly `rows * columns` elements.
    /// Zero-sized dimensions are accepted when their product matches the
    /// supplied storage; the Amiga source does not specify allocation rules
    /// for such shapes.
    pub fn new(
        rows: usize,
        columns: usize,
        elements: &'a mut [f64],
    ) -> Result<Self, MatrixShapeError> {
        let expected = rows
            .checked_mul(columns)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        if elements.len() != expected {
            return Err(MatrixShapeError::ElementCountMismatch {
                expected,
                actual: elements.len(),
            });
        }
        Ok(Self {
            rows,
            columns,
            elements,
        })
    }

    /// Number of rows in the view.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Number of columns in the view.
    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Set every element to zero (Amiga 3.5 `MAT CLR`).
    pub fn clear(&mut self) {
        self.elements.fill(0.0);
    }

    /// Set every element to the supplied scalar (Amiga 3.5 `MAT SET`).
    pub fn set(&mut self, value: f64) {
        self.elements.fill(value);
    }

    /// Replace the field with an identity matrix (Amiga 3.5 `MAT ONE`).
    ///
    /// The operation rejects non-square shapes before modifying storage.
    pub fn identity(&mut self) -> Result<(), MatrixShapeError> {
        if self.rows != self.columns {
            return Err(MatrixShapeError::IdentityRequiresSquare {
                rows: self.rows,
                columns: self.columns,
            });
        }

        self.elements.fill(0.0);
        for diagonal in 0..self.rows {
            self.elements[diagonal * self.columns + diagonal] = 1.0;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{MatrixMut, MatrixShapeError};

    #[test]
    fn clear_zeros_a_row_vector() {
        let mut values = [1.0, -2.0, 3.5];
        let mut matrix = MatrixMut::new(1, 3, &mut values).unwrap();

        matrix.clear();

        assert_eq!(values, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn set_fills_a_two_dimensional_field() {
        let mut values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut matrix = MatrixMut::new(2, 3, &mut values).unwrap();

        matrix.set(-2.5);

        assert_eq!(values, [-2.5; 6]);
    }

    #[test]
    fn identity_sets_only_the_diagonal() {
        let mut values = [9.0; 9];
        let mut matrix = MatrixMut::new(3, 3, &mut values).unwrap();

        matrix.identity().unwrap();

        assert_eq!(values, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn identity_accepts_a_one_by_one_matrix() {
        let mut values = [7.0];
        let mut matrix = MatrixMut::new(1, 1, &mut values).unwrap();

        matrix.identity().unwrap();

        assert_eq!(values, [1.0]);
    }

    #[test]
    fn identity_rejects_non_square_shape_without_mutating_storage() {
        let mut values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut matrix = MatrixMut::new(2, 3, &mut values).unwrap();

        assert_eq!(
            matrix.identity(),
            Err(MatrixShapeError::IdentityRequiresSquare {
                rows: 2,
                columns: 3
            })
        );
        assert_eq!(values, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn constructor_rejects_wrong_storage_length() {
        let mut values = [0.0; 5];

        assert_eq!(
            MatrixMut::new(2, 3, &mut values).err(),
            Some(MatrixShapeError::ElementCountMismatch {
                expected: 6,
                actual: 5
            })
        );
    }

    #[test]
    fn constructor_detects_dimension_product_overflow() {
        let mut values = [];

        assert_eq!(
            MatrixMut::new(usize::MAX, 2, &mut values).err(),
            Some(MatrixShapeError::DimensionOverflow)
        );
    }

    #[test]
    fn zero_by_zero_identity_is_empty() {
        let mut values = [];
        let mut matrix = MatrixMut::new(0, 0, &mut values).unwrap();

        assert_eq!((matrix.rows(), matrix.columns()), (0, 0));
        matrix.identity().unwrap();
        assert!(values.is_empty());
    }
}
