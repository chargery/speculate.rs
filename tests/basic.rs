#![feature(phase)]

#[phase(plugin)]
extern crate speculate;

speculate! {
    before {
        let mut i = 0u;
    }

    after {
        assert_eq!(i, 5);
    }

    it "works at level 1!" {
        assert_eq!(i, 0);
        i = 5;
    }

    describe "something" {
        before {
            assert_eq!(i, 0);
            i = 1;
        }

        after {
            assert_eq!(i, 4);
            i = 5;
        }

        describe "nested" {
            before {
                assert_eq!(i, 1);
                i = 2;
            }

            it "works at level 3!" {
                assert_eq!(i, 2);
                i = 3;
            }

            after {
                assert_eq!(i, 3);
                i = 4;
            }
        }
    }
}
