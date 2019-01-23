
include!("macros.rs");

#[derive(Copy, Clone)]
pub struct Matrix4<T> {
    m_Matrix : [[T; 4]; 4],
}

use std::ops:: { Add, Mul,  Sub, Div };

impl_add!(self, rhs, Matrix4<T>, {
    Matrix4::<T>
    {
        m_Matrix[0] [0] = self.m_Matrix[0][0] + rhs.m_Matrix[0][0],
    }
});