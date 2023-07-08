use xwrapup_rs_macro::{ui};

fn main() {
    ui!(
        test (a: "1", b: 1 + 2, d: 5) {
            test2 (b: "3") {
                test3 (d: "hhh") {

                }
            }
            test4 {

            }

            if (a == "1") {
                test5 {

                }
            }
        }
    );
}
