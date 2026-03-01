//! Image search adapter for finding stock images.
//!
//! Provides integration with Unsplash API for searching free-to-use stock images.
//! Images from Unsplash are publicly accessible and can be used directly with
//! Google Docs/Slides insert-image commands.

use serde::{Deserialize, Serialize};
use std::env;
use tracing::{error, info};

/// Unsplash API client for searching stock images.
#[derive(Debug, Clone)]
pub struct UnsplashClient {
    access_key: String,
    client: reqwest::blocking::Client,
}

/// A single image result from Unsplash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResult {
    /// Unique image ID
    pub id: String,
    /// Description or alt text
    pub description: Option<String>,
    /// Alternative description
    pub alt_description: Option<String>,
    /// Image dimensions
    pub width: u32,
    pub height: u32,
    /// URLs for different sizes
    pub urls: ImageUrls,
    /// User who uploaded the image
    pub user: ImageUser,
    /// Links for attribution
    pub links: ImageLinks,
}

/// URLs for different image sizes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrls {
    /// Raw image URL (full resolution)
    pub raw: String,
    /// Full size image
    pub full: String,
    /// Regular size (~1080px width)
    pub regular: String,
    /// Small size (~400px width)
    pub small: String,
    /// Thumbnail (~200px width)
    pub thumb: String,
    /// Small S3 URL (very small)
    pub small_s3: Option<String>,
}

/// User who uploaded the image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUser {
    pub id: String,
    pub username: String,
    pub name: String,
    pub portfolio_url: Option<String>,
}

/// Links for the image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageLinks {
    /// Self link
    #[serde(rename = "self")]
    pub self_link: String,
    /// HTML page link
    pub html: String,
    /// Download link
    pub download: String,
    /// Download location (for tracking)
    pub download_location: String,
}

/// Search response from Unsplash API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub total: u32,
    pub total_pages: u32,
    pub results: Vec<ImageResult>,
}

impl UnsplashClient {
    /// Create a new Unsplash client from environment variable.
    pub fn from_env() -> Result<Self, String> {
        let access_key = env::var("UNSPLASH_ACCESS_KEY")
            .map_err(|_| "UNSPLASH_ACCESS_KEY environment variable not set".to_string())?;

        Ok(Self {
            access_key,
            client: reqwest::blocking::Client::new(),
        })
    }

    /// Create a new Unsplash client with a specific access key.
    pub fn new(access_key: String) -> Self {
        Self {
            access_key,
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Search for images by query.
    ///
    /// # Arguments
    /// * `query` - Search query (e.g., "mountain landscape", "office meeting")
    /// * `per_page` - Number of results (1-30, default 10)
    /// * `orientation` - Optional: "landscape", "portrait", or "squarish"
    pub fn search_images(
        &self,
        query: &str,
        per_page: Option<u32>,
        orientation: Option<&str>,
    ) -> Result<SearchResponse, String> {
        let per_page = per_page.unwrap_or(10).min(30);

        let mut url = format!(
            "https://api.unsplash.com/search/photos?query={}&per_page={}",
            urlencoding::encode(query),
            per_page
        );

        if let Some(orient) = orientation {
            url.push_str(&format!("&orientation={}", orient));
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Client-ID {}", self.access_key))
            .header("Accept-Version", "v1")
            .send()
            .map_err(|e| format!("Failed to search images: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Unsplash API error: {} - {}", status, body);
            return Err(format!("Unsplash API error: {} - {}", status, body));
        }

        let search_response: SearchResponse = response
            .json()
            .map_err(|e| format!("Failed to parse Unsplash response: {}", e))?;

        info!(
            "Found {} images for query '{}'",
            search_response.total, query
        );

        Ok(search_response)
    }

    /// Get a random image matching a query.
    ///
    /// # Arguments
    /// * `query` - Search query
    /// * `orientation` - Optional: "landscape", "portrait", or "squarish"
    pub fn get_random_image(
        &self,
        query: &str,
        orientation: Option<&str>,
    ) -> Result<ImageResult, String> {
        let mut url = format!(
            "https://api.unsplash.com/photos/random?query={}",
            urlencoding::encode(query)
        );

        if let Some(orient) = orientation {
            url.push_str(&format!("&orientation={}", orient));
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Client-ID {}", self.access_key))
            .header("Accept-Version", "v1")
            .send()
            .map_err(|e| format!("Failed to get random image: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Unsplash API error: {} - {}", status, body);
            return Err(format!("Unsplash API error: {} - {}", status, body));
        }

        let image: ImageResult = response
            .json()
            .map_err(|e| format!("Failed to parse Unsplash response: {}", e))?;

        info!("Got random image for query '{}': {}", query, image.id);

        Ok(image)
    }

    /// Trigger a download event for attribution tracking.
    /// Call this when you actually use an image to comply with Unsplash guidelines.
    pub fn track_download(&self, image: &ImageResult) -> Result<(), String> {
        let response = self
            .client
            .get(&image.links.download_location)
            .header("Authorization", format!("Client-ID {}", self.access_key))
            .header("Accept-Version", "v1")
            .send()
            .map_err(|e| format!("Failed to track download: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Failed to track download: {} - {}", status, body);
            // Don't fail the whole operation for tracking failure
        }

        Ok(())
    }
}

impl ImageResult {
    /// Get the best URL for inserting into Google Docs/Slides.
    ///
    /// # Arguments
    /// * `max_width` - Maximum width in pixels (will return appropriate size)
    pub fn get_url_for_size(&self, max_width: Option<u32>) -> &str {
        let max_width = max_width.unwrap_or(800);

        if max_width <= 200 {
            &self.urls.thumb
        } else if max_width <= 400 {
            &self.urls.small
        } else if max_width <= 1080 {
            &self.urls.regular
        } else {
            &self.urls.full
        }
    }

    /// Get a formatted attribution string.
    pub fn get_attribution(&self) -> String {
        format!(
            "Photo by {} on Unsplash ({})",
            self.user.name, self.links.html
        )
    }

    /// Get the description or alt description.
    pub fn get_description(&self) -> String {
        self.description
            .clone()
            .or_else(|| self.alt_description.clone())
            .unwrap_or_else(|| "Image from Unsplash".to_string())
    }

    /// Get aspect ratio (width / height).
    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    /// Calculate dimensions to fit within a box while maintaining aspect ratio.
    ///
    /// # Arguments
    /// * `max_width` - Maximum width in points
    /// * `max_height` - Maximum height in points
    ///
    /// # Returns
    /// (width, height) in points
    pub fn fit_dimensions(&self, max_width: f64, max_height: f64) -> (f64, f64) {
        let aspect = self.aspect_ratio();

        // Try fitting to width first
        let w = max_width;
        let h = w / aspect;

        if h <= max_height {
            (w, h)
        } else {
            // Fit to height instead
            let h = max_height;
            let w = h * aspect;
            (w, h)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit_dimensions() {
        let image = ImageResult {
            id: "test".to_string(),
            description: None,
            alt_description: None,
            width: 1920,
            height: 1080,
            urls: ImageUrls {
                raw: "".to_string(),
                full: "".to_string(),
                regular: "".to_string(),
                small: "".to_string(),
                thumb: "".to_string(),
                small_s3: None,
            },
            user: ImageUser {
                id: "".to_string(),
                username: "".to_string(),
                name: "".to_string(),
                portfolio_url: None,
            },
            links: ImageLinks {
                self_link: "".to_string(),
                html: "".to_string(),
                download: "".to_string(),
                download_location: "".to_string(),
            },
        };

        // 16:9 image fitting in 300x200 box
        let (w, h) = image.fit_dimensions(300.0, 200.0);
        assert!((w - 300.0).abs() < 0.01);
        assert!((h - 168.75).abs() < 0.01); // 300 / (16/9)

        // 16:9 image fitting in 100x200 box (height limited)
        let (w, h) = image.fit_dimensions(100.0, 200.0);
        assert!((w - 100.0).abs() < 0.01);
        assert!((h - 56.25).abs() < 0.01);
    }
}
