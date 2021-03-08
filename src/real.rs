macro_rules! define_real_type {
    ($real:ident, $datatype:ty) => {
        #[derive(Clone, Copy, PartialEq, PartialOrd)]
        pub struct $real($datatype);

        impl ::std::fmt::Debug for $real {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl ::std::fmt::Display for $real {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl ::std::ops::Neg for $real {
            type Output = Self;
            fn neg(self) -> Self {
                $real(-self.0)
            }
        }

        impl ::std::ops::Add for $real {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                $real(self.0 + other.0)
            }
        }

        impl ::std::ops::AddAssign for $real {
            fn add_assign(&mut self, other: Self) {
                *self = Self(self.0 + other.0)
            }
        }

        impl ::std::ops::Sub for $real {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                $real(self.0 - other.0)
            }
        }

        impl ::std::ops::SubAssign for $real {
            fn sub_assign(&mut self, other: Self) {
                *self = Self(self.0 - other.0)
            }
        }

        impl ::std::ops::Mul for $real {
            type Output = Self;
            fn mul(self, other: Self) -> Self {
                $real(self.0 * other.0)
            }
        }

        impl ::std::ops::MulAssign for $real {
            fn mul_assign(&mut self, other: Self) {
                *self = Self(self.0 * other.0)
            }
        }

        impl ::std::ops::Div for $real {
            type Output = Self;
            fn div(self, other: Self) -> Self {
                $real(self.0 / other.0)
            }
        }

        impl ::std::ops::DivAssign for $real {
            fn div_assign(&mut self, other: Self) {
                *self = Self(self.0 / other.0)
            }
        }

        impl ::std::ops::Rem for $real {
            type Output = Self;
            fn rem(self, other: Self) -> Self {
                $real(self.0 % other.0)
            }
        }

        impl ::std::ops::RemAssign for $real {
            fn rem_assign(&mut self, other: Self) {
                *self = Self(self.0 % other.0)
            }
        }

        impl ::std::default::Default for $real {
            fn default() -> Self {
                $real(0.0)
            }
        }

        impl ::std::cmp::Eq for $real {}

        impl ::std::cmp::Ord for $real {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        impl ::std::convert::From<$datatype> for $real {
            fn from(src: $datatype) -> $real {
                $real(src)
            }
        }

        impl ::std::convert::From<$real> for $datatype {
            fn from(src: $real) -> $datatype {
                src.0
            }
        }
    };
}

define_real_type!(Real32, f32);
define_real_type!(Real64, f64);
