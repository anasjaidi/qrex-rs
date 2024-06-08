#![allow(unused)]
mod query_builder;

mod tuto {
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
            assert_eq!(3, one_plus!(1 + 12));
        }
    }
}
