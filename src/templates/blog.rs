use std::fs::DirEntry;
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

impl BlogPost {
    pub fn build(file: DirEntry) -> Option<Self> {
        let path = file.path();
        let slug = path
            .file_stem()?
            .to_str()?
            .to_string();
        let title = slug.replace("-", " ");
        return Some(Self {title, slug})
    }
}
#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias="BlogIndexStateRx")]
#[rx(hsr_ignore)]
struct BlogIndexState {
    links: RxVec<BlogPost>
}

#[cfg(engine)]
impl BlogIndexState {
    pub fn from_dir(path: &str) -> Result<BlogIndexState, Box<dyn std::error::Error>> {
        let links: Vec<BlogPost> = std::fs::read_dir(path)?
            .into_iter()
            .map(|x| BlogPost::build(x.ok()?))
            .filter_map(|x| x)
            .collect();
        Ok(BlogIndexState { links: links.into() })
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> BlogIndexState {
    BlogIndexState::from_dir("posts").expect("Failed to build blog post list")
}

#[auto_scope]
fn blog_index<G: Html>(cx: Scope, BlogIndexStateRx {links}: &BlogIndexStateRx) -> View<G> {
    let posts = create_ref(cx, links.get());
    let posts = View::new_fragment(
        posts.iter().map(|link| {
            view! { cx,
                a(href=format!("/blog/{}", link.slug)) {
                    div(class="blog-list-item") {
                        h3(class="blog-list-item-title") { (link.title) }
                    }}
            }
        }).collect()
    );

    view! { cx,
        Layout(title="Blog") {
            div {
                p { "This is a WIP, and a place for experiments - it's probably not going to be a traditional blog, so expect some slightly mismatched content." }
                h2 { "Blog Posts" }
                div(class="blog-list-container") {
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