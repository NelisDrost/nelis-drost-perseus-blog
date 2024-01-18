use perseus::prelude::*;
use sycamore::prelude::*;
use crate::components::layout::Layout;

fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Layout(title="About me") {
            p {
                "Hi, I'm Nelis, and this is my very much work in progress website.
                This site will eventually showcase some of my work, across several areas of interest, hopefully including some interactivity.
                In the mean time though, it is but a humble blog, rather lacking in both style and content."
            }
            p {
                "I'm building this site, from scratch, using " a(href="https://framesurge.sh/perseus/en-US/") { "Perseus" } ", and learning
                as I go, so it'll be rather rough around the edges for now. Please bear with me."
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About Page | Nelis Drost" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("about").view(about_page).head(head).build()
}

