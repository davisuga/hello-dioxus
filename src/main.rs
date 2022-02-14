use dioxus::prelude::*;

// Compiling dioxus-desktop v0.1.6
// error: there is no argument named `mime`
//    --> /home/davi/.cargo/registry/src/github.com-1ecc6299db9ec823/dioxus-desktop-0.1.6/src/lib.rs:247:49
//     |
// 247 | ...                   let meta = format!("{mime}");
//     |                                           ^^^^^^

// error: could not compile `dioxus-desktop` due to previous error

fn main() {
    dioxus::desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        div { "Hello, world!" }
    ))
}