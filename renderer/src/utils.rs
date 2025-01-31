use std::slice::{from_raw_parts, from_raw_parts_mut};

use cgmath::{Matrix, SquareMatrix};
use ldraw::{Matrix3, Matrix4};

pub(crate) fn cast_as_bytes<T>(input: &[T]) -> &[u8] {
    unsafe { from_raw_parts(input.as_ptr() as *const u8, input.len() * 4) }
}

pub fn cast_as_bytes_mut<T>(input: &mut [T]) -> &mut [u8] {
    unsafe { from_raw_parts_mut(input.as_mut_ptr() as *mut u8, input.len() * 4) }
}

fn truncate_matrix4(m: &Matrix4) -> Matrix3 {
    Matrix3::new(
        m[0][0], m[0][1], m[0][2], m[1][0], m[1][1], m[1][2], m[2][0], m[2][1], m[2][2],
    )
}

pub(crate) fn derive_normal_matrix(m: &Matrix4) -> Matrix3 {
    truncate_matrix4(m)
        .invert()
        .unwrap_or_else(Matrix3::identity)
        .transpose()
}
