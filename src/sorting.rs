pub mod insertion;
pub mod quick;
pub mod radix;
pub mod shell;

pub use insertion::insertion;
pub use quick::quick;
pub use radix::radix;
pub use shell::shell;

#[cfg(test)]
mod tests {
    use crate::data::ArrayWrapper;

    use super::*;
    const N: usize = 1000;

    macro_rules! check {
        ($name:ident, $sort_f:ident) => {
            check!($name, $sort_f, usize);
        };
        ($name:ident, $sort_f:ident, $type:ty) => {
            #[test]
            fn $name() {
                let mut array = Vec::<$type>::with_capacity(N);
                for _ in 0..N {
                    array.push(rand::random());
                }
                let mut array = ArrayWrapper::new(array);

                $sort_f(&mut array);
                let mut prev_opt: Option<$type> = None;
                for elm in array.into_inner().0 {
                    if let Some(prev) = prev_opt {
                        assert!(prev <= elm, "prev({}) <= elm({}) failed.", prev, elm);
                    }
                    prev_opt = Some(elm);
                }
            }
        };
    }

    check!(quick_test, quick);
    check!(insertion_test, insertion);
    check!(shell_test, shell);
    check!(radix_test, radix, u8);
}
