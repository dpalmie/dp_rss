use chrono::{DateTime, FixedOffset, NaiveDate};
use rss::{Item, ItemBuilder};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Post {
    pub title: String,
    pub slug: Option<String>,
    pub publish_date: String,
    pub last_edit_date: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub content: String,
    pub filename: String,
}

impl Post {
    pub fn from_file(file_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let filename = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        Self::parse_content(&content, filename)
    }

    pub fn parse_content(content: &str, filename: String) -> Result<Self, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = content.splitn(2, "---").collect();
        
        if parts.len() != 2 {
            return Err("Post must contain '---' separator between headers and content".into());
        }

        let headers_section = parts[0].trim();
        let content_section = parts[1].trim();

        let mut headers = HashMap::new();
        
        for line in headers_section.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(
                    key.trim().to_lowercase(),
                    value.trim().to_string(),
                );
            }
        }

        let title = headers
            .get("title")
            .ok_or("Missing required 'Title' field")?
            .clone();

        let publish_date = headers
            .get("publish date")
            .ok_or("Missing required 'Publish Date' field")?
            .clone();

        Ok(Post {
            title,
            slug: headers.get("slug").cloned(),
            publish_date,
            last_edit_date: headers.get("last edit date").cloned(),
            author: headers.get("author").cloned(),
            category: headers.get("category").cloned(),
            content: content_section.to_string(),
            filename,
        })
    }

    pub fn to_rss_item(&self, base_url: &str) -> Result<Item, Box<dyn std::error::Error>> {
        let default_slug = self.filename.replace(".txt", "");
        let slug = self.slug.as_ref().unwrap_or(&default_slug);
        let link = format!("{}/posts/{}", base_url.trim_end_matches('/'), slug);
        
        let mut item_builder = ItemBuilder::default();
        
        item_builder.title(Some(self.title.clone()));
        item_builder.description(Some(self.content.clone()));
        item_builder.link(Some(link.clone()));
        
        item_builder.guid(Some(rss::Guid {
            value: link,
            permalink: true,
        }));

        if let Ok(date) = self.parse_date(&self.publish_date) {
            item_builder.pub_date(Some(date.to_rfc2822()));
        }

        if let Some(author) = &self.author {
            item_builder.author(Some(author.clone()));
        }

        if let Some(category) = &self.category {
            item_builder.categories(vec![rss::Category {
                name: category.clone(),
                domain: None,
            }]);
        }

        Ok(item_builder.build())
    }

    fn parse_date(&self, date_str: &str) -> Result<DateTime<FixedOffset>, Box<dyn std::error::Error>> {
        let formats = [
            "%d %B %Y",       // "03 July 2025"
            "%B %d, %Y",      // "July 03, 2025"
            "%Y-%m-%d",       // "2025-07-03"
            "%m/%d/%Y",       // "07/03/2025"
        ];

        for format in &formats {
            if let Ok(naive_date) = NaiveDate::parse_from_str(date_str, format) {
                let datetime = naive_date.and_hms_opt(12, 0, 0).unwrap(); // Default to noon
                return Ok(DateTime::from_naive_utc_and_offset(datetime, FixedOffset::east_opt(0).unwrap()));
            }
        }

        Err(format!("Unable to parse date: {}", date_str).into())
    }
}

pub async fn load_posts(items_dir: &str) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
    let mut posts = Vec::new();
    let items_path = Path::new(items_dir);

    if !items_path.exists() {
        return Err(format!("Items directory '{}' does not exist", items_dir).into());
    }

    let entries = fs::read_dir(items_path)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            match Post::from_file(&path) {
                Ok(post) => posts.push(post),
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                }
            }
        }
    }

    posts.sort_by(|a, b| {
        let date_a = a.parse_date(&a.publish_date).unwrap_or_else(|_| {
            DateTime::from_naive_utc_and_offset(
                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                FixedOffset::east_opt(0).unwrap()
            )
        });
        let date_b = b.parse_date(&b.publish_date).unwrap_or_else(|_| {
            DateTime::from_naive_utc_and_offset(
                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                FixedOffset::east_opt(0).unwrap()
            )
        });
        date_b.cmp(&date_a)
    });

    Ok(posts)
} 