use perseus::prelude::*;
use perseus::state::rx_collections::RxVec;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use crate::components::layout::Layout;


#[derive(Serialize, Deserialize, Clone)]
struct BlogPost {
    title: String,
    slug: String
}

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias="BlogIndexStateRx")]
#[rx(hsr_ignore)]
struct BlogIndexState {
    links: RxVec<BlogPost>
}


#[cfg(engine)]
impl BlogIndexState {
    pub fn from_dir() -> Self {
        let mut links = Vec::new();
        // Load all files in ./posts directory
        for file in std::fs::read_dir("posts").unwrap() {
            let file = file.unwrap();
            let path = file.path();
            let slug = path.file_stem().unwrap().to_str().unwrap().to_string();
            let title = slug.replace("-", " ");
            links.push(BlogPost { title, slug });
        }
        BlogIndexState { links: links.into() }
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> BlogIndexState {
    BlogIndexState::from_dir()
}

#[auto_scope]
fn blog_index<G: Html>(cx: Scope, BlogIndexStateRx {links}: &BlogIndexStateRx) -> View<G> {
    let posts = create_ref(cx, links.get());
    let posts = View::new_fragment(
        posts.iter().map(|link| {
            view! { cx,
                a(href=format!("/blog/{}", link.slug)) { (link.title) }
                br {}
            }
        }).collect()
    );

    view! { cx,
        Layout(title="Blog") {
            div {
                p { "This is a WIP, and a place for experiments - it's probably not going to be a traditional blog, so expect some slightly mismatched content." }
                div(class="blog-list-container") {
                    h2 { "Blog Posts" }
                    (posts)
                }
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
        .view_with_state(blog_index)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}