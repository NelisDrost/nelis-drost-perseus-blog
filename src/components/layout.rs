use sycamore::prelude::*;

#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps {title, children}: LayoutProps<'a, G>,
) -> View<G> {
    let children = children.call(cx);

    view! { cx,
        html {
            div {
                div(class = "header-div", role = "banner") {
                    header(class = "site-title") {
                        img(class = "site-logo", src = ".perseus/static/Recamen_Avatar.png", alt="logo")
                        span { (title.to_string()) }
                    }
                    nav(class = "nav-div") {
                        ul(class= "nav-links") {
                            li(class= "nav-link-item") { a(class= "nav-link-text", href = "/") { "Home" } }
                            li(class= "nav-link-item") { a(class= "nav-link-text", href = "/about") { "About" } }
                            li(class= "nav-link-item") { a(class= "nav-link-text", href = "/blog") { "Blog" } }
                        }
                    }
                }
                main(class="container") {
                    // h1(class="heading") { (title.to_string()) }
                    (children)
                }
            }
            footer(class="footer") {
                div(class="footer-div") {
                    p { "© 2023 Nelis Drost" }
                }
            }
        }
    }
}

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    pub title: &'a str,
    pub children: Children<'a, G>
}