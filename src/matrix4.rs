
include!("macros.rs");

#[derive(Copy, Clone, PartialEq)]
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


impl_sub!(self, rhs, Matrix4<T>, {
    Matrix4::<T>
    {
        matrix : [
         self.matrix[0] - rhs.matrix[0],
         self.matrix[1] - rhs.matrix[1],
         self.matrix[2] - rhs.matrix[2],
         self.matrix[3] - rhs.matrix[3],
         self.matrix[4] - rhs.matrix[4],
         self.matrix[5] - rhs.matrix[5],
         self.matrix[6] - rhs.matrix[6],
         self.matrix[7] - rhs.matrix[7],
         self.matrix[8] - rhs.matrix[8],
         self.matrix[9] - rhs.matrix[9],
         self.matrix[10] - rhs.matrix[10],
         self.matrix[11] - rhs.matrix[11],
         self.matrix[12] - rhs.matrix[12],
         self.matrix[13] - rhs.matrix[13],
         self.matrix[14] - rhs.matrix[14],
         self.matrix[15] - rhs.matrix[15],
        ]
    }
});

impl<T> Mul<Matrix4<T>> for Matrix4<T> 
    where T: Copy + Mul<T, Output=T>, 
    <T as Mul>::Output: Add<Output=T>
{
	type Output = Matrix4<T>;
	fn mul(self, rhs : Matrix4<T> ) -> Matrix4<T> {
	    Matrix4::<T>
        {
            matrix : [
                (self.matrix[0] * rhs.matrix[0]) + (self.matrix[1] * rhs.matrix[4]) + (self.matrix[2] * rhs.matrix[8]) + (self.matrix[3] * rhs.matrix[12]), //0
                (self.matrix[0] * rhs.matrix[1]) + (self.matrix[1] * rhs.matrix[5]) + (self.matrix[2] * rhs.matrix[9]) + (self.matrix[3] * rhs.matrix[13]), //1
                (self.matrix[0] * rhs.matrix[2]) + (self.matrix[1] * rhs.matrix[6]) + (self.matrix[2] * rhs.matrix[10]) + (self.matrix[3] * rhs.matrix[14]), //2
                (self.matrix[0] * rhs.matrix[3]) + (self.matrix[1] * rhs.matrix[7]) + (self.matrix[2] * rhs.matrix[11]) + (self.matrix[3] * rhs.matrix[15]), //3
                
                (self.matrix[4] * rhs.matrix[0]) + (self.matrix[5] * rhs.matrix[4]) + (self.matrix[6] * rhs.matrix[8]) + (self.matrix[7] * rhs.matrix[12]), //4
                (self.matrix[4] * rhs.matrix[1]) + (self.matrix[5] * rhs.matrix[5]) + (self.matrix[6] * rhs.matrix[9]) + (self.matrix[7] * rhs.matrix[13]), //5
                (self.matrix[4] * rhs.matrix[2]) + (self.matrix[5] * rhs.matrix[6]) + (self.matrix[6] * rhs.matrix[10]) + (self.matrix[7] * rhs.matrix[14]), //6
                (self.matrix[4] * rhs.matrix[3]) + (self.matrix[5] * rhs.matrix[7]) + (self.matrix[6] * rhs.matrix[11]) + (self.matrix[7] * rhs.matrix[15]), //7
                
                (self.matrix[8] * rhs.matrix[0]) + (self.matrix[9] * rhs.matrix[4]) + (self.matrix[10] * rhs.matrix[8]) + (self.matrix[11] * rhs.matrix[12]), //8
                (self.matrix[8] * rhs.matrix[1]) + (self.matrix[9] * rhs.matrix[5]) + (self.matrix[10] * rhs.matrix[9]) + (self.matrix[11] * rhs.matrix[13]), //9
                (self.matrix[8] * rhs.matrix[2]) + (self.matrix[9] * rhs.matrix[6]) + (self.matrix[10] * rhs.matrix[10]) + (self.matrix[11] * rhs.matrix[14]), //10
                (self.matrix[8] * rhs.matrix[3]) + (self.matrix[9] * rhs.matrix[7]) + (self.matrix[10] * rhs.matrix[11]) + (self.matrix[11] * rhs.matrix[15]), //11

                (self.matrix[12] * rhs.matrix[0]) + (self.matrix[13] * rhs.matrix[4]) + (self.matrix[14] * rhs.matrix[8]) + (self.matrix[15] * rhs.matrix[12]), //12
                (self.matrix[12] * rhs.matrix[1]) + (self.matrix[13] * rhs.matrix[5]) + (self.matrix[14] * rhs.matrix[9]) + (self.matrix[15] * rhs.matrix[13]), //13
                (self.matrix[12] * rhs.matrix[2]) + (self.matrix[13] * rhs.matrix[6]) + (self.matrix[14] * rhs.matrix[10]) + (self.matrix[15] * rhs.matrix[14]), //14
                (self.matrix[12] * rhs.matrix[3]) + (self.matrix[13] * rhs.matrix[7]) + (self.matrix[14] * rhs.matrix[11]) + (self.matrix[15] * rhs.matrix[15]), //15
            ]
        }
    }
}


impl_div!(self, rhs, Matrix4<T>, {
    Matrix4::<T>
    {
        matrix : [
         self.matrix[0] / rhs.matrix[0],
         self.matrix[1] / rhs.matrix[1],
         self.matrix[2] / rhs.matrix[2],
         self.matrix[3] / rhs.matrix[3],
         self.matrix[4] / rhs.matrix[4],
         self.matrix[5] / rhs.matrix[5],
         self.matrix[6] / rhs.matrix[6],
         self.matrix[7] / rhs.matrix[7],
         self.matrix[8] / rhs.matrix[8],
         self.matrix[9] / rhs.matrix[9],
         self.matrix[10] / rhs.matrix[10],
         self.matrix[11] / rhs.matrix[11],
         self.matrix[12] / rhs.matrix[12],
         self.matrix[13] / rhs.matrix[13],
         self.matrix[14] / rhs.matrix[14],
         self.matrix[15] / rhs.matrix[15],
        ]
    }
});