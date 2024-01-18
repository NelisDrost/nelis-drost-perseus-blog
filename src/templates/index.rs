use perseus::prelude::*;
use sycamore::prelude::*;
use crate::components::layout::Layout;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Layout(title="Index") {
            div(style="welcome-div-style") {
                h2 { "Welcome!" }
                p { "Welcome to this website, my very much work-in-progress corner of the internet." }
                br {}
                p { "I'm Nelis, a " span {"Test"} "and this site will eventually house some of my work."}
                img(style="width: 200px; height: 200px", src=".perseus/static/Recamen_Avatar.png")
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Welcome | Nelis Drost" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .view(index_page)
        .head(head)
        .build()
}

