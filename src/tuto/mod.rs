mod macors {
    macro_rules! one_plus {
        (1 + 2) => {
            3
        };
        (1 + 3) => {
            3
        };
        (1+ $n: literal) => {
            1 + $n
        };
    }
    pub trait Nono {
        fn foo() -> u32 {
            5
        }
    }
    /**
     * $() for repition
     * + for one or more
     * */
    macro_rules! impl_for_one_or_more {
            ($t: ty, $($tt: ty, )+) => {

                $(
                    impl Nono for $tt {}
                )+
            };
        }

    macro_rules! r#try {
        ($v: expr) => {
            match $v {
                Ok(x) => x,
                Err(x) => x,
            }
        };
    }

    macro_rules! print_zero_or_more_lines {
            ($($l: literal, )*) => {
                $(
                    println!("{}", $l);
                )*
            };
        }

    macro_rules! add_zero_or_one {
            ($n: expr  $(, )* $(, $a: literal )?) => {{
                let mut n = $n;

                $(
                    n += $a;
                )?
                n
                }};
        }

    macro_rules! closing {
        () => {};
    }

    #[cfg(test)]
    pub mod macros_test {
        use crate::tuto::macors::Nono;

        #[test]

        fn test_kinds_closing() {
            closing!();
            closing![];
            closing! {};
        }

        #[test]
        fn test_rep0() {
            impl_for_one_or_more!(Nono, u32,);
            assert_eq!(5, u32::foo());
        }

        #[test]
        fn test_rep1() {
            print_zero_or_more_lines!("anas", "jaidi",);
        }
        #[test]

        fn test_rep2() {
            assert_eq!(5, add_zero_or_one!(5));
        }
        #[test]
        fn test_dummy0() {
            assert_eq!(3, one_plus!(1 + 2));
        }
        #[test]
        fn test_dummy1() {
            assert_eq!(3, one_plus!(1 + 3));
        }
        #[test]
        fn test_dummy2() {
            assert_eq!(13, one_plus!(1 + 12));
        }
        #[test]
        fn test_try0() {
            assert_eq!(5, r#try!(Ok(5)));
        }
        #[test]
        fn test_try1() {
            assert_eq!(5, r#try!(Err(5)));
        }
    }
}
