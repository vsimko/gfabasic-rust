//! Checked floating-point field operations from the Amiga GFA BASIC 3.5 addendum.
//!
//! The addendum documents MAT CLR and MAT SET for previously dimensioned
//! one- or two-dimensional floating-point fields, MAT ONE for square
//! floating-point matrices (printed pp. 13-2–13-3), and MAT ADD/MAT SUB/MAT MUL
//! scalar forms for element-wise arithmetic and matrix multiplication
//! (printed pp. 13-15–13-22), and row-/column-wise normalization
//! (printed pp. 13-31–13-33).
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
    ElementCountMismatch {
        expected: usize,
        actual: usize,
    },
    /// MAT INV is defined by the source for square matrices only.
    InverseRequiresSquare {
        rows: usize,
        columns: usize,
    },
    /// MAT INV destination must have the same shape as its source.
    InverseDestinationShapeMismatch {
        destination_rows: usize,
        destination_columns: usize,
        source_rows: usize,
        source_columns: usize,
    },
    /// The source matrix is singular at the reported pivot.
    SingularMatrix {
        pivot: usize,
    },
    /// MAT DET is defined by the source for square matrices only.
    DeterminantRequiresSquare {
        rows: usize,
        columns: usize,
    },
    /// A requested square submatrix lies outside the matrix field.
    SubmatrixOutOfBounds {
        start_row: usize,
        start_column: usize,
        size: usize,
        rows: usize,
        columns: usize,
    },
    /// MAT RANG is defined by the source for square matrices only.
    RankRequiresSquare {
        rows: usize,
        columns: usize,
    },
    /// The requested rank tolerance must be finite and non-negative.
    InvalidRankTolerance,
    /// MAT ONE is defined by the source for square matrices only.
    IdentityRequiresSquare {
        rows: usize,
        columns: usize,
    },
    /// The source storage does not match its declared shape.
    SourceElementCountMismatch {
        expected: usize,
        actual: usize,
    },
    /// Element-wise arithmetic requires identical source and destination shapes.
    ElementwiseShapeMismatch {
        destination_rows: usize,
        destination_columns: usize,
        source_rows: usize,
        source_columns: usize,
    },
    /// The vector–matrix–vector product requires matching vector/matrix axes.
    BilinearShapeMismatch {
        left_vector_length: usize,
        matrix_rows: usize,
        matrix_columns: usize,
        right_vector_length: usize,
    },
    /// Vector dot products require vectors with equal element counts.
    VectorLengthMismatch {
        left: usize,
        right: usize,
    },
    /// Matrix multiplication requires the left column count to equal right row count.
    ProductInnerDimensionMismatch {
        left_columns: usize,
        right_rows: usize,
    },
    /// Matrix multiplication destination must have the product's shape.
    ProductDestinationShapeMismatch {
        destination_rows: usize,
        destination_columns: usize,
        expected_rows: usize,
        expected_columns: usize,
    },
    /// MAT NORM cannot normalize a zero-length row or column.
    ZeroNormRow {
        row: usize,
    },
    ZeroNormColumn {
        column: usize,
    },
    /// MAT NORM requires finite row/column norms in this Rust implementation.
    NonFiniteNormRow {
        row: usize,
    },
    NonFiniteNormColumn {
        column: usize,
    },
    /// MAT TRANS requires destination dimensions to reverse the source shape.
    TransposeShapeMismatch {
        destination_rows: usize,
        destination_columns: usize,
        source_rows: usize,
        source_columns: usize,
    },
    /// The in-place form of MAT TRANS requires a square matrix.
    TransposeRequiresSquare {
        rows: usize,
        columns: usize,
    },
}

impl fmt::Display for MatrixShapeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionOverflow => write!(formatter, "matrix dimensions overflow usize"),
            Self::ElementCountMismatch { expected, actual } => write!(
                formatter,
                "matrix requires {expected} elements, but storage has {actual}"
            ),
            Self::InverseRequiresSquare { rows, columns } => write!(
                formatter,
                "matrix inverse requires a square source, got {rows}x{columns}"
            ),
            Self::InverseDestinationShapeMismatch {
                destination_rows,
                destination_columns,
                source_rows,
                source_columns,
            } => write!(
                formatter,
                "matrix inverse destination must be {source_rows}x{source_columns}, got {destination_rows}x{destination_columns}"
            ),
            Self::SingularMatrix { pivot } => {
                write!(formatter, "matrix is singular at pivot {pivot}")
            }
            Self::DeterminantRequiresSquare { rows, columns } => write!(
                formatter,
                "determinant requires a square matrix, got {rows}x{columns}"
            ),
            Self::SubmatrixOutOfBounds {
                start_row,
                start_column,
                size,
                rows,
                columns,
            } => write!(
                formatter,
                "submatrix at ({start_row}, {start_column}) with size {size} exceeds {rows}x{columns} matrix"
            ),
            Self::RankRequiresSquare { rows, columns } => write!(
                formatter,
                "matrix rank requires a square matrix, got {rows}x{columns}"
            ),
            Self::InvalidRankTolerance => write!(
                formatter,
                "matrix rank tolerance must be finite and non-negative"
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
            Self::BilinearShapeMismatch {
                left_vector_length,
                matrix_rows,
                matrix_columns,
                right_vector_length,
            } => write!(
                formatter,
                "vector-matrix-vector product requires vector lengths {matrix_rows} and {matrix_columns}, got {left_vector_length} and {right_vector_length}"
            ),
            Self::VectorLengthMismatch { left, right } => write!(
                formatter,
                "vector dot product requires equal lengths, got {left} and {right}"
            ),
            Self::ProductInnerDimensionMismatch {
                left_columns,
                right_rows,
            } => write!(
                formatter,
                "matrix product inner dimensions differ: {left_columns} and {right_rows}"
            ),
            Self::ProductDestinationShapeMismatch {
                destination_rows,
                destination_columns,
                expected_rows,
                expected_columns,
            } => write!(
                formatter,
                "matrix product destination must be {expected_rows}x{expected_columns}, got {destination_rows}x{destination_columns}"
            ),
            Self::ZeroNormRow { row } => write!(formatter, "cannot normalize zero-norm row {row}"),
            Self::ZeroNormColumn { column } => {
                write!(formatter, "cannot normalize zero-norm column {column}")
            }
            Self::NonFiniteNormRow { row } => {
                write!(formatter, "row {row} has a non-finite norm")
            }
            Self::NonFiniteNormColumn { column } => {
                write!(formatter, "column {column} has a non-finite norm")
            }
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

    /// Write the inverse of a square source matrix (Amiga 3.5 `MAT INV destination()=source()`).
    ///
    /// The destination must have the same shape as the source. Gauss–Jordan
    /// elimination with partial pivoting is performed on private copies, and
    /// the destination is updated only after a complete inverse is available.
    /// An exactly zero pivot reports a singular matrix. Numerical algorithm
    /// and singularity handling are Rust implementation choices.
    pub fn inverse_from(
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
        if source_rows != source_columns {
            return Err(MatrixShapeError::InverseRequiresSquare {
                rows: source_rows,
                columns: source_columns,
            });
        }
        if self.rows != source_rows || self.columns != source_columns {
            return Err(MatrixShapeError::InverseDestinationShapeMismatch {
                destination_rows: self.rows,
                destination_columns: self.columns,
                source_rows,
                source_columns,
            });
        }

        let size = source_rows;
        let mut work = source.to_vec();
        let mut inverse = vec![0.0; expected_source_len];
        for diagonal in 0..size {
            inverse[diagonal * size + diagonal] = 1.0;
        }

        for pivot_column in 0..size {
            let pivot_row = (pivot_column..size)
                .max_by(|left, right| {
                    work[*left * size + pivot_column]
                        .abs()
                        .total_cmp(&work[*right * size + pivot_column].abs())
                })
                .expect("a pivot row exists for each non-empty square matrix");
            let pivot = work[pivot_row * size + pivot_column];
            if pivot == 0.0 {
                return Err(MatrixShapeError::SingularMatrix {
                    pivot: pivot_column,
                });
            }
            if pivot_row != pivot_column {
                for column in 0..size {
                    work.swap(pivot_column * size + column, pivot_row * size + column);
                    inverse.swap(pivot_column * size + column, pivot_row * size + column);
                }
            }

            let pivot_value = work[pivot_column * size + pivot_column];
            for column in 0..size {
                work[pivot_column * size + column] /= pivot_value;
                inverse[pivot_column * size + column] /= pivot_value;
            }

            for row in 0..size {
                if row == pivot_column {
                    continue;
                }
                let factor = work[row * size + pivot_column];
                for column in 0..size {
                    work[row * size + column] -= factor * work[pivot_column * size + column];
                    inverse[row * size + column] -= factor * inverse[pivot_column * size + column];
                }
            }
        }

        self.elements.copy_from_slice(&inverse);
        Ok(())
    }

    /// Calculate the rank of a square matrix (Amiga 3.5 `MAT RANG x=a()`).
    ///
    /// The default absolute pivot tolerance is f64::EPSILON × n × max_abs,
    /// where n is the matrix order and max_abs is its largest absolute
    /// element. Use rank_with_tolerance when a caller needs another policy.
    pub fn rank(&self) -> Result<usize, MatrixShapeError> {
        if self.rows != self.columns {
            return Err(MatrixShapeError::RankRequiresSquare {
                rows: self.rows,
                columns: self.columns,
            });
        }
        let tolerance = rank_default_tolerance(self.elements, self.rows);
        Ok(rank_of_square(self.elements, self.rows, tolerance))
    }

    /// Calculate matrix rank using an explicit absolute pivot tolerance.
    ///
    /// This makes the floating-point threshold a caller-controlled Rust API
    /// choice rather than attributing one to the original interpreter.
    pub fn rank_with_tolerance(&self, tolerance: f64) -> Result<usize, MatrixShapeError> {
        if self.rows != self.columns {
            return Err(MatrixShapeError::RankRequiresSquare {
                rows: self.rows,
                columns: self.columns,
            });
        }
        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(MatrixShapeError::InvalidRankTolerance);
        }
        Ok(rank_of_square(self.elements, self.rows, tolerance))
    }

    /// Calculate the rank of a square part matrix (Amiga 3.5 `MAT RANG x=a([i,j]),n`).
    ///
    /// The starting offsets are zero-based relative to this Rust matrix view.
    /// The same scale-relative default tolerance as rank is used.
    pub fn rank_submatrix(
        &self,
        start_row: usize,
        start_column: usize,
        size: usize,
    ) -> Result<usize, MatrixShapeError> {
        let submatrix = self.copy_square_submatrix(start_row, start_column, size)?;
        let tolerance = rank_default_tolerance(&submatrix, size);
        Ok(rank_of_square(&submatrix, size, tolerance))
    }

    /// Calculate a square part-matrix rank using an explicit absolute tolerance.
    pub fn rank_submatrix_with_tolerance(
        &self,
        start_row: usize,
        start_column: usize,
        size: usize,
        tolerance: f64,
    ) -> Result<usize, MatrixShapeError> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(MatrixShapeError::InvalidRankTolerance);
        }
        let submatrix = self.copy_square_submatrix(start_row, start_column, size)?;
        Ok(rank_of_square(&submatrix, size, tolerance))
    }

    fn copy_square_submatrix(
        &self,
        start_row: usize,
        start_column: usize,
        size: usize,
    ) -> Result<Vec<f64>, MatrixShapeError> {
        let end_row = start_row
            .checked_add(size)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        let end_column = start_column
            .checked_add(size)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        if end_row > self.rows || end_column > self.columns {
            return Err(MatrixShapeError::SubmatrixOutOfBounds {
                start_row,
                start_column,
                size,
                rows: self.rows,
                columns: self.columns,
            });
        }

        let capacity = size
            .checked_mul(size)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        let mut submatrix = Vec::with_capacity(capacity);
        for row in start_row..end_row {
            let start = row * self.columns + start_column;
            submatrix.extend_from_slice(&self.elements[start..start + size]);
        }
        Ok(submatrix)
    }

    /// Calculate the determinant of a square field (Amiga 3.5 `MAT DET x=a()`).
    ///
    /// Uses Gaussian elimination with partial pivoting on a private copy, so
    /// the matrix storage is never modified. A zero pivot yields determinant
    /// zero. Floating-point rounding and singular-edge behavior are Rust
    /// implementation details, not claims about the original interpreter.
    pub fn determinant(&self) -> Result<f64, MatrixShapeError> {
        if self.rows != self.columns {
            return Err(MatrixShapeError::DeterminantRequiresSquare {
                rows: self.rows,
                columns: self.columns,
            });
        }

        Ok(determinant_of_square(self.elements, self.rows))
    }

    /// Calculate a square submatrix determinant (Amiga 3.5 `MAT DET x=a([i,j]),n`).
    ///
    /// Offsets are zero-based relative to this Rust matrix view. Both offsets
    /// and the requested square extent are checked before reading any elements.
    pub fn determinant_submatrix(
        &self,
        start_row: usize,
        start_column: usize,
        size: usize,
    ) -> Result<f64, MatrixShapeError> {
        let end_row = start_row
            .checked_add(size)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        let end_column = start_column
            .checked_add(size)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        if end_row > self.rows || end_column > self.columns {
            return Err(MatrixShapeError::SubmatrixOutOfBounds {
                start_row,
                start_column,
                size,
                rows: self.rows,
                columns: self.columns,
            });
        }

        let mut submatrix = Vec::with_capacity(
            size.checked_mul(size)
                .ok_or(MatrixShapeError::DimensionOverflow)?,
        );
        for row in start_row..end_row {
            let start = row * self.columns + start_column;
            submatrix.extend_from_slice(&self.elements[start..start + size]);
        }
        Ok(determinant_of_square(&submatrix, size))
    }

    /// Copy a rectangular region from a source field (Amiga 3.5 MAT CPY).
    ///
    /// Source and destination offsets are zero-based. The requested rectangle
    /// is clipped to the elements available in both fields, matching the
    /// addendum's rule that only corresponding valid indices are copied.
    /// The source remains unchanged.
    #[allow(clippy::too_many_arguments)]
    pub fn copy_region_from(
        &mut self,
        source: &[f64],
        source_rows: usize,
        source_columns: usize,
        destination_row: usize,
        destination_column: usize,
        source_row: usize,
        source_column: usize,
        height: usize,
        width: usize,
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
        destination_row
            .checked_add(height)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        destination_column
            .checked_add(width)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        source_row
            .checked_add(height)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        source_column
            .checked_add(width)
            .ok_or(MatrixShapeError::DimensionOverflow)?;

        let rows_to_copy = height
            .min(self.rows.saturating_sub(destination_row))
            .min(source_rows.saturating_sub(source_row));
        let columns_to_copy = width
            .min(self.columns.saturating_sub(destination_column))
            .min(source_columns.saturating_sub(source_column));
        if rows_to_copy == 0 || columns_to_copy == 0 {
            return Ok(());
        }

        for row in 0..rows_to_copy {
            let source_start = (source_row + row) * source_columns + source_column;
            let destination_start = (destination_row + row) * self.columns + destination_column;
            self.elements[destination_start..destination_start + columns_to_copy]
                .copy_from_slice(&source[source_start..source_start + columns_to_copy]);
        }
        Ok(())
    }

    /// Copy a source rectangle while transposing its rows and columns (Amiga 3.5 MAT XCPY).
    ///
    /// The source rectangle has height × width elements; the destination
    /// receives the transposed width × height region. Out-of-range portions
    /// are clipped to corresponding valid indices. The source remains unchanged.
    #[allow(clippy::too_many_arguments)]
    pub fn transpose_copy_region_from(
        &mut self,
        source: &[f64],
        source_rows: usize,
        source_columns: usize,
        destination_row: usize,
        destination_column: usize,
        source_row: usize,
        source_column: usize,
        height: usize,
        width: usize,
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
        destination_row
            .checked_add(width)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        destination_column
            .checked_add(height)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        source_row
            .checked_add(height)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        source_column
            .checked_add(width)
            .ok_or(MatrixShapeError::DimensionOverflow)?;

        let rows_to_copy = height
            .min(source_rows.saturating_sub(source_row))
            .min(self.columns.saturating_sub(destination_column));
        let columns_to_copy = width
            .min(source_columns.saturating_sub(source_column))
            .min(self.rows.saturating_sub(destination_row));

        for row in 0..rows_to_copy {
            for column in 0..columns_to_copy {
                let source_index = (source_row + row) * source_columns + source_column + column;
                let destination_index =
                    (destination_row + column) * self.columns + destination_column + row;
                self.elements[destination_index] = source[source_index];
            }
        }
        Ok(())
    }

    /// Copy all corresponding indices from a source field (Amiga 3.5 MAT CPY).
    ///
    /// When shapes differ, only their common top-left rectangle is copied.
    pub fn copy_overlapping_from(
        &mut self,
        source: &[f64],
        source_rows: usize,
        source_columns: usize,
    ) -> Result<(), MatrixShapeError> {
        self.copy_region_from(
            source,
            source_rows,
            source_columns,
            0,
            0,
            0,
            0,
            self.rows.min(source_rows),
            self.columns.min(source_columns),
        )
    }

    /// Add a same-shaped floating-point field element-wise (Amiga 3.5 `MAT ADD a(),b()`).
    ///
    /// The receiver is the destination and is updated in place.
    ///
    /// The source shape and storage length are validated before any element changes.
    pub fn add_from(
        &mut self,
        source: &[f64],
        source_rows: usize,
        source_columns: usize,
    ) -> Result<(), MatrixShapeError> {
        self.apply_elementwise(source, source_rows, source_columns, |left, right| {
            left + right
        })
    }

    /// Subtract a same-shaped floating-point field element-wise (Amiga 3.5 `MAT SUB a(),b()`).
    ///
    /// The receiver is the destination and is updated in place.
    pub fn subtract_from(
        &mut self,
        source: &[f64],
        source_rows: usize,
        source_columns: usize,
    ) -> Result<(), MatrixShapeError> {
        self.apply_elementwise(source, source_rows, source_columns, |left, right| {
            left - right
        })
    }

    /// Add a scalar to every element (Amiga 3.5 `MAT ADD a(),x`).
    pub fn add_scalar(&mut self, value: f64) {
        self.elements
            .iter_mut()
            .for_each(|element| *element += value);
    }

    /// Subtract a scalar from every element (Amiga 3.5 `MAT SUB a(),x`).
    pub fn subtract_scalar(&mut self, value: f64) {
        self.elements
            .iter_mut()
            .for_each(|element| *element -= value);
    }

    /// Multiply every element by a scalar (Amiga 3.5 `MAT MUL a(),x`).
    pub fn multiply_scalar(&mut self, value: f64) {
        self.elements
            .iter_mut()
            .for_each(|element| *element *= value);
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

    /// Multiply two matrices using row-by-column products (Amiga 3.5 `MAT MUL a()=b()*c()`).
    ///
    /// The left matrix is `left_rows × left_columns`, the right matrix is
    /// `right_rows × right_columns`, and the receiver must be
    /// `left_rows × right_columns`. All shapes and source lengths are checked
    /// before the destination is modified.
    pub fn multiply_from(
        &mut self,
        left: &[f64],
        left_rows: usize,
        left_columns: usize,
        right: &[f64],
        right_rows: usize,
        right_columns: usize,
    ) -> Result<(), MatrixShapeError> {
        let expected_left = left_rows
            .checked_mul(left_columns)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        if left.len() != expected_left {
            return Err(MatrixShapeError::SourceElementCountMismatch {
                expected: expected_left,
                actual: left.len(),
            });
        }
        let expected_right = right_rows
            .checked_mul(right_columns)
            .ok_or(MatrixShapeError::DimensionOverflow)?;
        if right.len() != expected_right {
            return Err(MatrixShapeError::SourceElementCountMismatch {
                expected: expected_right,
                actual: right.len(),
            });
        }
        if left_columns != right_rows {
            return Err(MatrixShapeError::ProductInnerDimensionMismatch {
                left_columns,
                right_rows,
            });
        }
        if self.rows != left_rows || self.columns != right_columns {
            return Err(MatrixShapeError::ProductDestinationShapeMismatch {
                destination_rows: self.rows,
                destination_columns: self.columns,
                expected_rows: left_rows,
                expected_columns: right_columns,
            });
        }

        let mut result = vec![0.0; self.elements.len()];
        for row in 0..left_rows {
            for column in 0..right_columns {
                let mut sum = 0.0;
                for inner in 0..left_columns {
                    sum += left[row * left_columns + inner] * right[inner * right_columns + column];
                }
                result[row * right_columns + column] = sum;
            }
        }
        self.elements.copy_from_slice(&result);
        Ok(())
    }

    /// Write the dyadic (outer) product of two vectors (Amiga 3.5 `MAT MUL a()=b()*c()`).
    ///
    /// A vector of length m multiplied dyadically by a vector of length n
    /// produces an m × n matrix. Destination shape is checked before writing.
    pub fn outer_product_from(
        &mut self,
        left: &[f64],
        right: &[f64],
    ) -> Result<(), MatrixShapeError> {
        let expected_rows = left.len();
        let expected_columns = right.len();
        if self.rows != expected_rows || self.columns != expected_columns {
            return Err(MatrixShapeError::ProductDestinationShapeMismatch {
                destination_rows: self.rows,
                destination_columns: self.columns,
                expected_rows,
                expected_columns,
            });
        }

        for (row, left_value) in left.iter().enumerate() {
            for (column, right_value) in right.iter().enumerate() {
                self.elements[row * self.columns + column] = left_value * right_value;
            }
        }
        Ok(())
    }

    /// Normalize each row to unit Euclidean length (Amiga 3.5 `MAT NORM a(),0`).
    ///
    /// Norms are checked before mutation. A zero or non-finite norm is reported
    /// as a Rust-side validation error; the original interpreter's edge-case
    /// behavior is not specified by the cited addendum.
    pub fn normalize_rows(&mut self) -> Result<(), MatrixShapeError> {
        let mut norms = Vec::with_capacity(self.rows);
        for row in 0..self.rows {
            let start = row * self.columns;
            let norm = self.elements[start..start + self.columns]
                .iter()
                .fold(0.0_f64, |acc, value| acc.hypot(*value));
            if !norm.is_finite() {
                return Err(MatrixShapeError::NonFiniteNormRow { row });
            }
            if norm == 0.0 {
                return Err(MatrixShapeError::ZeroNormRow { row });
            }
            norms.push(norm);
        }

        for (row, norm) in norms.into_iter().enumerate() {
            let start = row * self.columns;
            for element in &mut self.elements[start..start + self.columns] {
                *element /= norm;
            }
        }
        Ok(())
    }

    /// Normalize each column to unit Euclidean length (Amiga 3.5 `MAT NORM a(),1`).
    ///
    /// Norms are checked before mutation. A zero or non-finite norm is reported
    /// as a Rust-side validation error; the original interpreter's edge-case
    /// behavior is not specified by the cited addendum.
    pub fn normalize_columns(&mut self) -> Result<(), MatrixShapeError> {
        let mut norms = Vec::with_capacity(self.columns);
        for column in 0..self.columns {
            let norm = (0..self.rows).fold(0.0_f64, |acc, row| {
                acc.hypot(self.elements[row * self.columns + column])
            });
            if !norm.is_finite() {
                return Err(MatrixShapeError::NonFiniteNormColumn { column });
            }
            if norm == 0.0 {
                return Err(MatrixShapeError::ZeroNormColumn { column });
            }
            norms.push(norm);
        }

        for (column, norm) in norms.into_iter().enumerate() {
            for row in 0..self.rows {
                self.elements[row * self.columns + column] /= norm;
            }
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

fn rank_default_tolerance(elements: &[f64], size: usize) -> f64 {
    let max_abs = elements
        .iter()
        .fold(0.0_f64, |maximum, value| maximum.max(value.abs()));
    f64::EPSILON * size as f64 * max_abs
}

fn rank_of_square(elements: &[f64], size: usize, tolerance: f64) -> usize {
    let mut work = elements.to_vec();
    let mut pivot_row = 0;

    for pivot_column in 0..size {
        if pivot_row == size {
            break;
        }
        let best_row = (pivot_row..size)
            .max_by(|left, right| {
                work[*left * size + pivot_column]
                    .abs()
                    .total_cmp(&work[*right * size + pivot_column].abs())
            })
            .expect("a candidate pivot row exists while rows remain");
        let pivot_magnitude = work[best_row * size + pivot_column].abs();
        if pivot_magnitude <= tolerance {
            continue;
        }
        if best_row != pivot_row {
            for column in 0..size {
                work.swap(pivot_row * size + column, best_row * size + column);
            }
        }

        let pivot = work[pivot_row * size + pivot_column];
        for row in (pivot_row + 1)..size {
            let factor = work[row * size + pivot_column] / pivot;
            for column in (pivot_column + 1)..size {
                work[row * size + column] -= factor * work[pivot_row * size + column];
            }
            work[row * size + pivot_column] = 0.0;
        }
        pivot_row += 1;
    }
    pivot_row
}

fn determinant_of_square(elements: &[f64], size: usize) -> f64 {
    let mut work = elements.to_vec();
    let mut determinant = 1.0;
    for pivot_column in 0..size {
        let pivot_row = (pivot_column..size)
            .max_by(|left, right| {
                work[*left * size + pivot_column]
                    .abs()
                    .total_cmp(&work[*right * size + pivot_column].abs())
            })
            .expect("a pivot row exists for each non-empty square matrix");
        let pivot = work[pivot_row * size + pivot_column];
        if pivot == 0.0 {
            return 0.0;
        }
        if pivot_row != pivot_column {
            for column in 0..size {
                work.swap(pivot_column * size + column, pivot_row * size + column);
            }
            determinant = -determinant;
        }
        let pivot = work[pivot_column * size + pivot_column];
        determinant *= pivot;

        for row in (pivot_column + 1)..size {
            let factor = work[row * size + pivot_column] / pivot;
            for column in (pivot_column + 1)..size {
                work[row * size + column] -= factor * work[pivot_column * size + column];
            }
            work[row * size + pivot_column] = 0.0;
        }
    }
    determinant
}

/// Calculate the vector–matrix–vector scalar product
/// (Amiga 3.5 `MAT MUL x=a()*b()*c()`).
///
/// The left vector length must equal the matrix row count and the right vector
/// length must equal its column count. The matrix storage length and both
/// vector dimensions are checked before calculation.
pub fn vector_matrix_vector_product(
    left: &[f64],
    matrix: &[f64],
    matrix_rows: usize,
    matrix_columns: usize,
    right: &[f64],
) -> Result<f64, MatrixShapeError> {
    let expected_matrix_len = matrix_rows
        .checked_mul(matrix_columns)
        .ok_or(MatrixShapeError::DimensionOverflow)?;
    if matrix.len() != expected_matrix_len {
        return Err(MatrixShapeError::SourceElementCountMismatch {
            expected: expected_matrix_len,
            actual: matrix.len(),
        });
    }
    if left.len() != matrix_rows || right.len() != matrix_columns {
        return Err(MatrixShapeError::BilinearShapeMismatch {
            left_vector_length: left.len(),
            matrix_rows,
            matrix_columns,
            right_vector_length: right.len(),
        });
    }

    let mut result = 0.0;
    for (row, left_value) in left.iter().enumerate() {
        for (column, right_value) in right.iter().enumerate() {
            result += left_value * matrix[row * matrix_columns + column] * right_value;
        }
    }
    Ok(result)
}

/// Calculate the scalar (dot) product of two equal-length vectors
/// (Amiga 3.5 `MAT MUL x=a()*b()`).
///
/// A length mismatch is reported as a Rust API validation error.
pub fn dot_product(left: &[f64], right: &[f64]) -> Result<f64, MatrixShapeError> {
    if left.len() != right.len() {
        return Err(MatrixShapeError::VectorLengthMismatch {
            left: left.len(),
            right: right.len(),
        });
    }
    Ok(left.iter().zip(right).map(|(a, b)| a * b).sum())
}

#[cfg(test)]
mod tests {
    use super::{MatrixMut, MatrixShapeError, dot_product, vector_matrix_vector_product};

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
    fn inverse_from_calculates_inverse_with_pivoting() {
        let source = [0.0, 2.0, 1.0, 3.0];
        let mut destination = [0.0; 4];
        let mut matrix = MatrixMut::new(2, 2, &mut destination).unwrap();

        matrix.inverse_from(&source, 2, 2).unwrap();

        let expected = [-1.5, 1.0, 0.5, 0.0];
        for (actual, expected) in destination.iter().zip(expected) {
            assert!((actual - expected).abs() < 1e-12);
        }
        assert_eq!(source, [0.0, 2.0, 1.0, 3.0]);
    }

    #[test]
    fn inverse_from_rejects_singular_matrix_without_mutation() {
        let source = [1.0, 2.0, 2.0, 4.0];
        let mut destination = [9.0; 4];
        let mut matrix = MatrixMut::new(2, 2, &mut destination).unwrap();

        assert_eq!(
            matrix.inverse_from(&source, 2, 2),
            Err(MatrixShapeError::SingularMatrix { pivot: 1 })
        );
        assert_eq!(destination, [9.0; 4]);
    }

    #[test]
    fn inverse_from_rejects_non_square_source() {
        let source = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut destination = [9.0; 6];
        let mut matrix = MatrixMut::new(2, 3, &mut destination).unwrap();

        assert_eq!(
            matrix.inverse_from(&source, 2, 3),
            Err(MatrixShapeError::InverseRequiresSquare {
                rows: 2,
                columns: 3,
            })
        );
        assert_eq!(destination, [9.0; 6]);
    }

    #[test]
    fn rank_calculates_full_and_deficient_matrix_rank() {
        let mut full_rank = [1.0, 2.0, 3.0, 4.0];
        let matrix = MatrixMut::new(2, 2, &mut full_rank).unwrap();
        assert_eq!(matrix.rank(), Ok(2));

        let mut deficient = [1.0, 2.0, 2.0, 4.0];
        let matrix = MatrixMut::new(2, 2, &mut deficient).unwrap();
        assert_eq!(matrix.rank(), Ok(1));
    }

    #[test]
    fn rank_matches_amiga_35_addendum_example() {
        let mut values = [
            [1.0, 2.0, 3.0, 4.0, 5.0],
            [3.2, 4.0, -5.0, 2.4, 5.1],
            [-2.0, 4.0, -5.0, 2.4, 5.1],
            [5.0, -2.3, 4.0, 5.6, 12.2],
            [4.1, 5.2, 16.7, 18.4, 19.1],
        ]
        .concat();
        let matrix = MatrixMut::new(5, 5, &mut values).unwrap();

        assert_eq!(matrix.rank(), Ok(5));
        // The manual's MAT BASE 1 offset (1,2) maps to Rust offset (0,1).
        assert_eq!(matrix.rank_submatrix(0, 1, 3), Ok(2));
    }

    #[test]
    fn rank_submatrix_calculates_selected_square_region() {
        let mut values = [1.0, 2.0, 3.0, 0.0, 4.0, 5.0, 1.0, 0.0, 6.0];
        let matrix = MatrixMut::new(3, 3, &mut values).unwrap();

        assert_eq!(matrix.rank_submatrix(1, 1, 2), Ok(2));
    }

    #[test]
    fn rank_rejects_non_square_matrix_and_invalid_tolerance() {
        let mut values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let matrix = MatrixMut::new(2, 3, &mut values).unwrap();

        assert_eq!(
            matrix.rank(),
            Err(MatrixShapeError::RankRequiresSquare {
                rows: 2,
                columns: 3,
            })
        );
    }

    #[test]
    fn rank_rejects_invalid_explicit_tolerance() {
        let mut values = [1.0, 0.0, 0.0, 1.0];
        let matrix = MatrixMut::new(2, 2, &mut values).unwrap();

        assert_eq!(
            matrix.rank_with_tolerance(f64::NAN),
            Err(MatrixShapeError::InvalidRankTolerance)
        );
        assert_eq!(
            matrix.rank_with_tolerance(-1.0),
            Err(MatrixShapeError::InvalidRankTolerance)
        );
    }

    #[test]
    fn determinant_calculates_square_matrix_with_row_swapping() {
        let mut values = [0.0, 2.0, 1.0, 3.0];
        let matrix = MatrixMut::new(2, 2, &mut values).unwrap();

        assert_eq!(matrix.determinant(), Ok(-2.0));
        assert_eq!(values, [0.0, 2.0, 1.0, 3.0]);
    }

    #[test]
    fn determinant_submatrix_calculates_selected_square_region() {
        let mut values = [1.0, 2.0, 3.0, 0.0, 4.0, 5.0, 1.0, 0.0, 6.0];
        let matrix = MatrixMut::new(3, 3, &mut values).unwrap();

        assert_eq!(matrix.determinant_submatrix(1, 1, 2), Ok(24.0));
    }

    #[test]
    fn determinant_submatrix_rejects_out_of_bounds_region() {
        let mut values = [1.0, 2.0, 3.0, 4.0];
        let matrix = MatrixMut::new(2, 2, &mut values).unwrap();

        assert_eq!(
            matrix.determinant_submatrix(1, 1, 2),
            Err(MatrixShapeError::SubmatrixOutOfBounds {
                start_row: 1,
                start_column: 1,
                size: 2,
                rows: 2,
                columns: 2,
            })
        );
    }

    #[test]
    fn determinant_returns_zero_for_singular_matrix() {
        let mut values = [1.0, 2.0, 2.0, 4.0];
        let matrix = MatrixMut::new(2, 2, &mut values).unwrap();

        assert_eq!(matrix.determinant(), Ok(0.0));
    }

    #[test]
    fn determinant_rejects_non_square_matrix() {
        let mut values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let matrix = MatrixMut::new(2, 3, &mut values).unwrap();

        assert_eq!(
            matrix.determinant(),
            Err(MatrixShapeError::DeterminantRequiresSquare {
                rows: 2,
                columns: 3,
            })
        );
    }

    #[test]
    fn copy_region_copies_requested_rectangle_between_offsets() {
        let source = [
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 3.0, 4.0, 5.0],
            [3.0, 4.0, 5.0, 6.0],
            [4.0, 5.0, 6.0, 7.0],
        ]
        .concat();
        let mut destination = [1.0; 25];
        let mut matrix = MatrixMut::new(5, 5, &mut destination).unwrap();

        matrix
            .copy_region_from(&source, 4, 4, 1, 1, 1, 1, 3, 3)
            .unwrap();

        assert_eq!(
            destination,
            [
                [1.0, 1.0, 1.0, 1.0, 1.0],
                [1.0, 3.0, 4.0, 5.0, 1.0],
                [1.0, 4.0, 5.0, 6.0, 1.0],
                [1.0, 5.0, 6.0, 7.0, 1.0],
                [1.0, 1.0, 1.0, 1.0, 1.0],
            ]
            .concat()
        );
    }

    #[test]
    fn copy_overlapping_copies_common_indices_only() {
        let source = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut destination = [9.0; 6];
        let mut matrix = MatrixMut::new(3, 2, &mut destination).unwrap();

        matrix.copy_overlapping_from(&source, 2, 3).unwrap();

        assert_eq!(destination, [1.0, 2.0, 4.0, 5.0, 9.0, 9.0]);
    }

    #[test]
    fn transpose_copy_region_copies_transposed_rectangle() {
        let source = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut destination = [9.0; 9];
        let mut matrix = MatrixMut::new(3, 3, &mut destination).unwrap();

        matrix
            .transpose_copy_region_from(&source, 2, 3, 0, 0, 0, 0, 2, 3)
            .unwrap();

        assert_eq!(destination, [1.0, 4.0, 9.0, 2.0, 5.0, 9.0, 3.0, 6.0, 9.0]);
        assert_eq!(source, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
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
    fn multiply_from_computes_rectangular_row_by_column_product() {
        let left = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let right = [1.0, 2.0, -3.0, 4.0, 5.0, -1.0];
        let mut destination = [0.0; 4];
        let mut matrix = MatrixMut::new(2, 2, &mut destination).unwrap();

        matrix.multiply_from(&left, 2, 3, &right, 3, 2).unwrap();

        assert_eq!(destination, [3.0, 5.0, 3.0, 5.0]);
    }

    #[test]
    fn multiply_from_rejects_incompatible_inner_dimensions_without_mutation() {
        let left = [1.0; 6];
        let right = [1.0; 4];
        let mut destination = [9.0; 4];
        let mut matrix = MatrixMut::new(2, 2, &mut destination).unwrap();

        assert_eq!(
            matrix.multiply_from(&left, 2, 3, &right, 2, 2),
            Err(MatrixShapeError::ProductInnerDimensionMismatch {
                left_columns: 3,
                right_rows: 2,
            })
        );
        assert_eq!(destination, [9.0; 4]);
    }

    #[test]
    fn multiply_from_rejects_wrong_destination_shape_without_mutation() {
        let left = [1.0; 6];
        let right = [1.0; 6];
        let mut destination = [9.0; 6];
        let mut matrix = MatrixMut::new(2, 3, &mut destination).unwrap();

        assert_eq!(
            matrix.multiply_from(&left, 2, 3, &right, 3, 2),
            Err(MatrixShapeError::ProductDestinationShapeMismatch {
                destination_rows: 2,
                destination_columns: 3,
                expected_rows: 2,
                expected_columns: 2,
            })
        );
        assert_eq!(destination, [9.0; 6]);
    }

    #[test]
    fn outer_product_computes_vector_dyadic_product() {
        let left = [1.0, 2.0, -3.0];
        let right = [4.0, 5.0];
        let mut destination = [0.0; 6];
        let mut matrix = MatrixMut::new(3, 2, &mut destination).unwrap();

        matrix.outer_product_from(&left, &right).unwrap();

        assert_eq!(destination, [4.0, 5.0, 8.0, 10.0, -12.0, -15.0]);
    }

    #[test]
    fn outer_product_rejects_wrong_destination_shape_without_mutation() {
        let left = [1.0, 2.0];
        let right = [3.0, 4.0, 5.0];
        let mut destination = [9.0; 4];
        let mut matrix = MatrixMut::new(2, 2, &mut destination).unwrap();

        assert_eq!(
            matrix.outer_product_from(&left, &right),
            Err(MatrixShapeError::ProductDestinationShapeMismatch {
                destination_rows: 2,
                destination_columns: 2,
                expected_rows: 2,
                expected_columns: 3,
            })
        );
        assert_eq!(destination, [9.0; 4]);
    }

    #[test]
    fn dot_product_computes_scalar_product_of_equal_length_vectors() {
        assert_eq!(dot_product(&[1.0, 2.0, -3.0], &[4.0, 5.0, -1.0]), Ok(17.0));
    }

    #[test]
    fn dot_product_rejects_different_vector_lengths() {
        assert_eq!(
            dot_product(&[1.0, 2.0], &[3.0]),
            Err(MatrixShapeError::VectorLengthMismatch { left: 2, right: 1 })
        );
    }

    #[test]
    fn vector_matrix_vector_product_computes_documented_scalar_form() {
        let left = [1.0, 2.0];
        let matrix = [1.0; 6];
        let right = [-3.0, 4.0, 5.0];

        assert_eq!(
            vector_matrix_vector_product(&left, &matrix, 2, 3, &right),
            Ok(18.0)
        );
    }

    #[test]
    fn vector_matrix_vector_product_rejects_incompatible_vectors() {
        let left = [1.0];
        let matrix = [1.0; 6];
        let right = [3.0, 4.0, 5.0];

        assert_eq!(
            vector_matrix_vector_product(&left, &matrix, 2, 3, &right),
            Err(MatrixShapeError::BilinearShapeMismatch {
                left_vector_length: 1,
                matrix_rows: 2,
                matrix_columns: 3,
                right_vector_length: 3,
            })
        );
    }

    #[test]
    fn normalize_rows_scales_each_row_to_unit_length() {
        let mut values = [3.0, 4.0, 0.0, 2.0];
        let mut matrix = MatrixMut::new(2, 2, &mut values).unwrap();

        matrix.normalize_rows().unwrap();

        assert_eq!(values, [0.6, 0.8, 0.0, 1.0]);
    }

    #[test]
    fn normalize_columns_scales_each_column_to_unit_length() {
        let mut values = [3.0, 0.0, 4.0, 5.0];
        let mut matrix = MatrixMut::new(2, 2, &mut values).unwrap();

        matrix.normalize_columns().unwrap();

        assert_eq!(values, [0.6, 0.0, 0.8, 1.0]);
    }

    #[test]
    fn normalize_rows_rejects_zero_norm_without_partial_mutation() {
        let mut values = [3.0, 4.0, 0.0, 0.0];
        let mut matrix = MatrixMut::new(2, 2, &mut values).unwrap();

        assert_eq!(
            matrix.normalize_rows(),
            Err(MatrixShapeError::ZeroNormRow { row: 1 })
        );
        assert_eq!(values, [3.0, 4.0, 0.0, 0.0]);
    }

    #[test]
    fn normalize_columns_rejects_zero_norm_without_partial_mutation() {
        let mut values = [3.0, 0.0, 4.0, 0.0];
        let mut matrix = MatrixMut::new(2, 2, &mut values).unwrap();

        assert_eq!(
            matrix.normalize_columns(),
            Err(MatrixShapeError::ZeroNormColumn { column: 1 })
        );
        assert_eq!(values, [3.0, 0.0, 4.0, 0.0]);
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
