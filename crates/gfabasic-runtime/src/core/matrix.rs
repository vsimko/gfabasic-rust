//! Checked floating-point field operations from the Amiga GFA BASIC 3.5 addendum.
//!
//! The addendum documents MAT CLR and MAT SET for previously dimensioned
//! one- or two-dimensional floating-point fields, MAT ONE for square
//! floating-point matrices (printed pp. 13-2–13-3), and MAT ADD/MAT SUB/MAT MUL
//! scalar forms for element-wise arithmetic (printed pp. 13-15–13-27).
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
    /// The source storage does not match its declared shape.
    SourceElementCountMismatch { expected: usize, actual: usize },
    /// Element-wise arithmetic requires identical source and destination shapes.
    ElementwiseShapeMismatch {
        destination_rows: usize,
        destination_columns: usize,
        source_rows: usize,
        source_columns: usize,
    },
    /// MAT TRANS requires destination dimensions to reverse the source shape.
    TransposeShapeMismatch {
        destination_rows: usize,
        destination_columns: usize,
        source_rows: usize,
        source_columns: usize,
    },
    /// The in-place form of MAT TRANS requires a square matrix.
    TransposeRequiresSquare { rows: usize, columns: usize },
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
            Self::SourceElementCountMismatch { expected, actual } => write!(
                formatter,
                "matrix source requires {expected} elements, but storage has {actual}"
            ),
            Self::ElementwiseShapeMismatch {
                destination_rows,
                destination_columns,
                source_rows,
                source_columns,
            } => write!(
                formatter,
                "element-wise operation requires {destination_rows}x{destination_columns} source, got {source_rows}x{source_columns}"
            ),
            Self::TransposeShapeMismatch {
                destination_rows,
                destination_columns,
                source_rows,
                source_columns,
            } => write!(
                formatter,
                "transpose destination must be {source_columns}x{source_rows}, got {destination_rows}x{destination_columns}"
            ),
            Self::TransposeRequiresSquare { rows, columns } => write!(
                formatter,
                "in-place transpose requires a square shape, got {rows}x{columns}"
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

    /// Add a same-shaped floating-point field element-wise (Amiga 3.5 `MAT ADD a(),b()`).
    ///
    /// The source shape and storage length are validated before any element changes.
    pub fn add_from(
        &mut self,
        source: &[f64],
        source_rows: usize,
        source_columns: usize,
    ) -> Result<(), MatrixShapeError> {
        self.apply_elementwise(source, source_rows, source_columns, |left, right| left + right)
    }

    /// Subtract a same-shaped floating-point field element-wise (Amiga 3.5 `MAT SUB a(),b()`).
    pub fn subtract_from(
        &mut self,
        source: &[f64],
        source_rows: usize,
        source_columns: usize,
    ) -> Result<(), MatrixShapeError> {
        self.apply_elementwise(source, source_rows, source_columns, |left, right| left - right)
    }

    /// Add a scalar to every element (Amiga 3.5 `MAT ADD a(),x`).
    pub fn add_scalar(&mut self, value: f64) {
        self.elements.iter_mut().for_each(|element| *element += value);
    }

    /// Subtract a scalar from every element (Amiga 3.5 `MAT SUB a(),x`).
    pub fn subtract_scalar(&mut self, value: f64) {
        self.elements.iter_mut().for_each(|element| *element -= value);
    }

    /// Multiply every element by a scalar (Amiga 3.5 `MAT MUL a(),x`).
    pub fn multiply_scalar(&mut self, value: f64) {
        self.elements.iter_mut().for_each(|element| *element *= value);
    }

    fn apply_elementwise(
        &mut self,
        source: &[f64],
        source_rows: usize,
        source_columns: usize,
        operation: fn(f64, f64) -> f64,
    ) -> Result<(), MatrixShapeError> {
        let expected_source_len = source_rows
            .checked_mul(source_columns)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        if source.len() != expected_source_len {
            return Err(MatrixShapeError::SourceElementCountMismatch {
                expected: expected_source_len,
                actual: source.len(),
            });
        }
        if self.rows != source_rows || self.columns != source_columns {
            return Err(MatrixShapeError::ElementwiseShapeMismatch {
                destination_rows: self.rows,
                destination_columns: self.columns,
                source_rows,
                source_columns,
            });
        }

        for (destination, source) in self.elements.iter_mut().zip(source) {
            *destination = operation(*destination, *source);
        }
        Ok(())
    }

    /// Copy the transpose of a separate row-major source field into this field
    /// (Amiga 3.5 `MAT TRANS destination()=source()`).
    ///
    /// If the source is `m × n`, this destination must be `n × m`.
    /// Source shape and destination shape are checked before any write occurs.
    pub fn transpose_from(
        &mut self,
        source: &[f64],
        source_rows: usize,
        source_columns: usize,
    ) -> Result<(), MatrixShapeError> {
        let expected_source_len = source_rows
            .checked_mul(source_columns)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        if source.len() != expected_source_len {
            return Err(MatrixShapeError::SourceElementCountMismatch {
                expected: expected_source_len,
                actual: source.len(),
            });
        }
        if self.rows != source_columns || self.columns != source_rows {
            return Err(MatrixShapeError::TransposeShapeMismatch {
                destination_rows: self.rows,
                destination_columns: self.columns,
                source_rows,
                source_columns,
            });
        }

        for destination_row in 0..self.rows {
            for destination_column in 0..self.columns {
                self.elements[destination_row * self.columns + destination_column] =
                    source[destination_column * source_columns + destination_row];
            }
        }
        Ok(())
    }

    /// Transpose a square field in place (Amiga 3.5 `MAT TRANS matrix()`).
    ///
    /// Non-square shapes are rejected before any element is modified.
    pub fn transpose_in_place(&mut self) -> Result<(), MatrixShapeError> {
        if self.rows != self.columns {
            return Err(MatrixShapeError::TransposeRequiresSquare {
                rows: self.rows,
                columns: self.columns,
            });
        }

        for row in 0..self.rows {
            for column in (row + 1)..self.columns {
                self.elements
                    .swap(row * self.columns + column, column * self.columns + row);
            }
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
    fn add_from_combines_same_shaped_fields_element_wise() {
        let source = [10.0, 20.0, 30.0, 40.0];
        let mut destination = [1.0, 2.0, 3.0, 4.0];
        let mut matrix = MatrixMut::new(2, 2, &mut destination).unwrap();

        matrix.add_from(&source, 2, 2).unwrap();

        assert_eq!(destination, [11.0, 22.0, 33.0, 44.0]);
    }

    #[test]
    fn subtract_from_combines_same_shaped_fields_element_wise() {
        let source = [10.0, 20.0, 30.0, 40.0];
        let mut destination = [11.0, 22.0, 33.0, 44.0];
        let mut matrix = MatrixMut::new(2, 2, &mut destination).unwrap();

        matrix.subtract_from(&source, 2, 2).unwrap();

        assert_eq!(destination, [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn elementwise_operations_reject_shape_mismatch_without_mutation() {
        let source = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut destination = [9.0; 6];
        let mut matrix = MatrixMut::new(2, 3, &mut destination).unwrap();

        assert_eq!(
            matrix.add_from(&source, 3, 2),
            Err(MatrixShapeError::ElementwiseShapeMismatch {
                destination_rows: 2,
                destination_columns: 3,
                source_rows: 3,
                source_columns: 2,
            })
        );
        assert_eq!(destination, [9.0; 6]);
    }

    #[test]
    fn elementwise_operations_reject_wrong_source_length_without_mutation() {
        let source = [1.0, 2.0, 3.0];
        let mut destination = [9.0; 4];
        let mut matrix = MatrixMut::new(2, 2, &mut destination).unwrap();

        assert_eq!(
            matrix.subtract_from(&source, 2, 2),
            Err(MatrixShapeError::SourceElementCountMismatch {
                expected: 4,
                actual: 3,
            })
        );
        assert_eq!(destination, [9.0; 4]);
    }

    #[test]
    fn scalar_arithmetic_updates_every_element() {
        let mut values = [1.0, 2.0, 3.0];
        let mut matrix = MatrixMut::new(1, 3, &mut values).unwrap();

        matrix.add_scalar(2.0);
        matrix.subtract_scalar(1.0);
        matrix.multiply_scalar(3.0);

        assert_eq!(values, [6.0, 9.0, 12.0]);
    }

    #[test]
    fn transpose_copies_a_rectangular_source_into_reversed_shape() {
        let source = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut destination = [0.0; 6];
        let mut matrix = MatrixMut::new(3, 2, &mut destination).unwrap();

        matrix.transpose_from(&source, 2, 3).unwrap();

        assert_eq!(destination, [1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    }

    #[test]
    fn transpose_rejects_incompatible_destination_without_mutation() {
        let source = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut destination = [9.0; 6];
        let mut matrix = MatrixMut::new(2, 3, &mut destination).unwrap();

        assert_eq!(
            matrix.transpose_from(&source, 2, 3),
            Err(MatrixShapeError::TransposeShapeMismatch {
                destination_rows: 2,
                destination_columns: 3,
                source_rows: 2,
                source_columns: 3,
            })
        );
        assert_eq!(destination, [9.0; 6]);
    }

    #[test]
    fn transpose_rejects_incorrect_source_storage_length() {
        let source = [1.0, 2.0, 3.0, 4.0, 5.0];
        let mut destination = [0.0; 6];
        let mut matrix = MatrixMut::new(3, 2, &mut destination).unwrap();

        assert_eq!(
            matrix.transpose_from(&source, 2, 3),
            Err(MatrixShapeError::SourceElementCountMismatch {
                expected: 6,
                actual: 5,
            })
        );
        assert_eq!(destination, [0.0; 6]);
    }

    #[test]
    fn transpose_in_place_swaps_only_off_diagonal_elements() {
        let mut values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let mut matrix = MatrixMut::new(3, 3, &mut values).unwrap();

        matrix.transpose_in_place().unwrap();

        assert_eq!(values, [1.0, 4.0, 7.0, 2.0, 5.0, 8.0, 3.0, 6.0, 9.0]);
    }

    #[test]
    fn transpose_in_place_rejects_non_square_shape_without_mutation() {
        let mut values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut matrix = MatrixMut::new(2, 3, &mut values).unwrap();

        assert_eq!(
            matrix.transpose_in_place(),
            Err(MatrixShapeError::TransposeRequiresSquare {
                rows: 2,
                columns: 3,
            })
        );
        assert_eq!(values, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
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
