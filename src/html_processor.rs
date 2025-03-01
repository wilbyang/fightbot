use scraper::{Html, Selector};
use std::sync::Arc;
use crate::id_mapping::IdMapping;
use regex::Regex;
use lazy_static::lazy_static;

pub struct HtmlProcessor {
    id_mapping: Arc<IdMapping>,
}

impl HtmlProcessor {
    pub fn new(id_mapping: Arc<IdMapping>) -> Self {
        Self { id_mapping }
    }

    pub fn process(&self, content: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let html_str = String::from_utf8_lossy(content);
        let document = Html::parse_document(&html_str);
        let mut processed_html = html_str.to_string();

        // Process HTML elements with IDs
        let elements_with_id = Selector::parse("[id]").unwrap();
        for element in document.select(&elements_with_id) {
            if let Some(id) = element.value().attr("id") {
                let new_id = self.id_mapping.get_or_create_mapping(id);
                processed_html = processed_html.replace(&format!("id=\"{}\"", id), 
                                                      &format!("id=\"{}\"", new_id));
                
                // Update JavaScript references
                processed_html = self.update_js_references(&processed_html, id, &new_id);
                
                // Update CSS references
                processed_html = self.update_css_references(&processed_html, id, &new_id);
            }
        }

        Ok(processed_html.into_bytes())
    }

    fn update_js_references(&self, content: &str, old_id: &str, new_id: &str) -> String {
        lazy_static! {
            static ref JS_ID_PATTERNS: Vec<Regex> = vec![
                Regex::new(r#"getElementById\(['"](.*?)['"]"#).unwrap(),
                Regex::new(r#"querySelector\(['"]#(.*?)['"]"#).unwrap(),
            ];
        }

        let mut result = content.to_string();
        for pattern in JS_ID_PATTERNS.iter() {
            result = pattern.replace_all(&result, |caps: &regex::Captures| {
                let captured_id = &caps[1];
                if captured_id == old_id {
                    if caps[0].starts_with("getElementById") {
                        format!("getElementById('{}'", new_id)
                    } else {
                        format!("querySelector('#{}'", new_id)
                    }
                } else {
                    caps[0].to_string()
                }
            }).to_string();
        }
        result
    }

    fn update_css_references(&self, content: &str, old_id: &str, new_id: &str) -> String {
        lazy_static! {
            static ref CSS_ID_PATTERN: Regex = Regex::new(r#"#([a-zA-Z][\w-]*)"#).unwrap();
        }

        CSS_ID_PATTERN.replace_all(content, |caps: &regex::Captures| {
            let captured_id = &caps[1];
            if captured_id == old_id {
                format!("#{}", new_id)
            } else {
                caps[0].to_string()
            }
        }).to_string()
    }
} 