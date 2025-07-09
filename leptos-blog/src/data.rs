use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub slug: String,
    pub tags: Vec<String>,
    pub category: String,
    pub layout: String,
    pub html_content: String,
    pub original_filename: String,
    pub preview: String,
}

impl Post {
    pub fn get_url(&self) -> String {
        format!("/posts/{}", self.slug)
    }
}

// Include the generated posts module
include!(concat!(env!("OUT_DIR"), "/posts.rs"));

pub fn get_all_tags() -> Vec<String> {
    let mut tags: Vec<String> = get_posts()
        .iter()
        .flat_map(|post| post.tags.clone())
        .collect();
    
    tags.sort();
    tags.dedup();
    tags
}

pub fn find_post_by_slug(slug: &str) -> Option<Post> {
    get_posts().into_iter().find(|post| post.slug == slug)
}

pub fn filter_posts_by_tag(tag: &str) -> Vec<Post> {
    if tag == "all" {
        get_posts()
    } else {
        get_posts()
            .into_iter()
            .filter(|post| post.tags.contains(&tag.to_string()))
            .collect()
    }
}