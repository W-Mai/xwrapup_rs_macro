use xwrapup_rs_macro::{ui};

static A: i32 = 20;

fn app(parent: i32) {
    ui! {
        :(
            parent: parent
        :)

        div (
            width: 100,
            height: 100 + A,
            color: "red"
        ) {
            text (content: "hello world") {
                picker (values: vec!["1", "2", "3"]) {

                }
            }
            button {}

            if a == "1" {
                input {

                }
            }
        }
    }
}

fn main() {
    let screen = 10;
    app(screen);

    return;
}
