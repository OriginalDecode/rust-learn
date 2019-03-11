
include!("macros.rs");

#[derive(Copy, Clone)]
pub struct Matrix4<T> {
    matrix : [T; 16],
}


use std::ops:: { Add, Mul,  Sub, Div };

impl_add!(self, rhs, Matrix4<T>, {
    Matrix4::<T>
    {
        matrix : [
         self.matrix[0] + rhs.matrix[0],
         self.matrix[1] + rhs.matrix[1],
         self.matrix[2] + rhs.matrix[2],
         self.matrix[3] + rhs.matrix[3],
         self.matrix[4] + rhs.matrix[4],
         self.matrix[5] + rhs.matrix[5],
         self.matrix[6] + rhs.matrix[6],
         self.matrix[7] + rhs.matrix[7],
         self.matrix[8] + rhs.matrix[8],
         self.matrix[9] + rhs.matrix[9],
         self.matrix[10] + rhs.matrix[10],
         self.matrix[11] + rhs.matrix[11],
         self.matrix[12] + rhs.matrix[12],
         self.matrix[13] + rhs.matrix[13],
         self.matrix[14] + rhs.matrix[14],
         self.matrix[15] + rhs.matrix[15],
        ]
    }
});

// impl_sub!(self, rhs, Matrix4<T>, {
//     Matrix4::<T>
//     {
//         matrix[0][0] : self.matrix[0][0] - rhs.matrix[0][0],
//     }
// });

// impl_div!(self, rhs, Matrix4<T>, {
//     Matrix4::<T>
//     {
//         matrix[0][0] : self.matrix[0][0] / rhs.matrix[0][0],
//     }
// });

// impl_mul!(self, rhs, Matrix4<T>, {
//     Matrix4::<T>
//     {
//         matrix[0][0] : self.matrix[0][0] * rhs.matrix[0][0],
//     }
// });
