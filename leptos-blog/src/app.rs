use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_params,
    path,
    params::Params,
    StaticSegment,
};
use crate::{data::*, components::*};

#[cfg(feature = "ssr")]
use std::fs;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-blog.css"/>

        // sets the document title
        <Title text="Renan A Fonseca - Blog"/>

        // content for this blog
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=path!("/posts/:slug") view=PostPage/>
            </Routes>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let _posts = RwSignal::new(get_posts());
    let all_tags = RwSignal::new(get_all_tags());
    let current_filter = RwSignal::new("all".to_string());
    
    let filtered_posts = Memo::new(move |_| {
        let filter = current_filter.get();
        filter_posts_by_tag(&filter)
    });

    let filter_posts = move |tag: String| {
        current_filter.set(tag);
    };

    view! {
        <header>
            <div class="header-content">
                <div>
                    <h1>"Renan A Fonseca"</h1>
                    <p>"some random thoughts"</p>
                </div>
                <ThemeToggle/>
            </div>
        </header>
        
        <main>
            <section id="filters">
                <div class="tag-filters">
                    <button 
                        class="tag-filter"
                        class:active=move || current_filter.get() == "all"
                        on:click=move |_| filter_posts("all".to_string())
                    >
                        "All Posts"
                    </button>
                    <div id="tag-buttons">
                        {move || all_tags.get().into_iter().map(|tag| {
                            let tag_clone = tag.clone();
                            let filter_posts = filter_posts.clone();
                            view! {
                                <button 
                                    class="tag-filter"
                                    class:active=move || current_filter.get() == tag
                                    on:click=move |_| filter_posts(tag_clone.clone())
                                >
                                    {tag.clone()}
                                </button>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </section>
            
            <section id="posts">
                {move || {
                    let posts = filtered_posts.get();
                    posts.into_iter().map(|post| {
                        let filter_posts = filter_posts.clone();
                        view! {
                            <article class="post-card">
                                <h2 class="post-title">
                                    <a href=post.get_url()>{post.title.clone()}</a>
                                </h2>
                                <div class="post-tags">
                                    {post.tags.clone().into_iter().map(|tag| {
                                        let filter_posts = filter_posts.clone();
                                        let tag_clone = tag.clone();
                                        view! {
                                            <span 
                                                class="post-tag"
                                                on:click=move |_| filter_posts(tag_clone.clone())
                                            >
                                                {tag}
                                            </span>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                                <p class="post-preview">{post.preview}</p>
                            </article>
                        }
                    }).collect_view()
                }}
                
                <Show
                    when=move || filtered_posts.get().is_empty()
                    fallback=|| ()
                >
                    <p>"No posts found for the selected tag."</p>
                </Show>
            </section>
        </main>
        
        <footer>
            <p>"© 2024-2025 Renan Alves Fonseca"</p>
        </footer>
    }
}

#[derive(Params, PartialEq)]
struct PostParams {
    slug: Option<String>,
}

#[component]
fn PostPage() -> impl IntoView {
    let params = use_params::<PostParams>();
    
    let post = Memo::new(move |_| {
        params.with(|params| {
            match params {
                Ok(params) => {
                    match &params.slug {
                        Some(slug) => find_post_by_slug(slug),
                        None => None,
                    }
                },
                Err(_) => None,
            }
        })
    });

    view! {
        <header>
            <div class="header-content">
                <div>
                    <h1>"Renan A Fonseca"</h1>
                    <p>"some random thoughts"</p>
                </div>
                <ThemeToggle/>
            </div>
        </header>
        
        <main class="post-content">
            <a href="/" class="back-link">"← Back to posts"</a>
            
            {move || {
                match post.get() {
                    Some(post) => {
                        view! {
                            <article>
                                <header class="post-header">
                                    <h1>{post.title.clone()}</h1>
                                    <div class="post-tags">
                                        {post.tags.into_iter().map(|tag| {
                                            view! {
                                                <span class="post-tag">{tag}</span>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </header>
                                <div class="post-body">
                                    <PostContent slug=post.slug.clone() />
                                </div>
                            </article>
                        }.into_any()
                    },
                    None => {
                        view! {
                            <div>
                                <h1>"Post not found"</h1>
                                <p>"The requested post could not be found."</p>
                            </div>
                        }.into_any()
                    }
                }
            }}
        </main>
        
        <footer>
            <p>"© 2024-2025 Renan Alves Fonseca"</p>
        </footer>
    }
}

// Static route generation for all blog posts
#[cfg(feature = "ssr")]
pub async fn static_routes() -> Vec<String> {
    let posts = get_posts();
    let mut routes = vec!["/".to_string()];
    
    for post in posts {
        routes.push(post.get_url());
    }
    
    routes
}

#[component]
fn PostContent(slug: String) -> impl IntoView {
    let html_content = Resource::new_blocking(
        move || slug.clone(),
        move |slug_param| async move {
            #[cfg(feature = "ssr")]
            {
                let path = format!("posts-html/{}.html", slug_param);
                match fs::read_to_string(&path) {
                    Ok(content) => content,
                    Err(_) => "Post content not found".to_string(),
                }
            }
            #[cfg(not(feature = "ssr"))]
            {
                let _ = slug_param; // Use the parameter to avoid warning
                "Post content not available on client side".to_string()
            }
        },
    );

    view! {
        <Suspense fallback=move || view! { <p>"Loading post content..."</p> }>
            {move || {
                html_content.get().map(|content| {
                    view! {
                        <div inner_html=content></div>
                    }
                })
            }}
        </Suspense>
    }
}
