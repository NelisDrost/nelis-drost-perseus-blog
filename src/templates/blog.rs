use perseus::prelude::*;
use sycamore::prelude::*;
use crate::components::layout::Layout;

fn blog_index<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Layout(title="Blog") {
            div(style="blog-index-div-style") {
                h2 { "Blog" }
                p { "This is the blog index page." }
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Blog | Nelis Drost" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("blog")
        .view(blog_index)
        .head(head)
        .build()
}