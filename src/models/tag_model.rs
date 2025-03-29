use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};

#[derive(Debug, FromRow, Serialize, Deserialize, Clone, Decode)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

/// TagResponse is used for API responses where we don't want to expose the ID
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagResponse {
    pub name: String,
    pub slug: String,
}

impl From<Tag> for TagResponse {
    fn from(tag: Tag) -> Self {
        Self {
            name: tag.name,
            slug: tag.slug,
        }
    }
}

impl Tag {
    /// Creates a new tag with automatically generated slug from the name
    pub fn new(name: String) -> Self {
        let slug = Tag::slugify(&name);
        Self { 
            id: 0, // Default value, will be set by the database
            name, 
            slug 
        }
    }

    /// Converts a string to a slug format:
    /// - Converts to lowercase
    /// - Removes special characters
    /// - Replaces spaces with hyphens
    pub fn slugify(input: &str) -> String {
        let lowercase = input.to_lowercase();
        
        // Replace spaces with hyphens and filter out special characters
        lowercase
            .chars()
            .map(|c| match c {
                ' ' => '-',
                'a'..='z' | '0'..='9' | '-' => c,
                _ => ' ',  // temporarily replace other chars with spaces
            })
            .collect::<String>()
            .split_whitespace() // split by temporary spaces to remove them
            .collect::<Vec<&str>>()
            .join("-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(Tag::slugify("Jazz"), "jazz");
        assert_eq!(Tag::slugify("Rock & Roll"), "rock-roll");
        assert_eq!(Tag::slugify("Hip-Hop"), "hip-hop");
        assert_eq!(Tag::slugify("70's Music"), "70s-music");
        assert_eq!(Tag::slugify("R&B / Soul"), "rb-soul");
        assert_eq!(Tag::slugify("   Multiple   Spaces   "), "multiple-spaces");
    }
}