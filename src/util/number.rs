use num_traits::Zero;
use rustfft::num_complex::Complex;

pub fn to_complex<T: Zero>(number: T) -> Complex<T> {
    Complex::new(number, T::zero())
}
