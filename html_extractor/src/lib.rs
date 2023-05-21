use std::io::Read;

use downloader::{basic_hash, download_file};
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Name, Predicate};



/// Matches if the inner Predicate `B` is a direct sibling of `A`
/// eg: `<ul><li>a</li><span>b</span><li>c</li></ul>`
/// Sibling(Name("li"), Name("span")) would return the <span> b
/// because it's immediately prior sibling is a <li>
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Sibling<A, B>(pub A, pub B);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Nothing;

impl Predicate for Nothing {
    fn matches(&self, _node: &Node) -> bool {
        false
    }
}

/// check if A matches the node, and
/// adds another check if A has a direct child that matches
/// predicate B.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HasChild<A, B>(pub A, pub B);

impl<A: Predicate, B: Predicate> Predicate for HasChild<A, B> {
    fn matches(&self, node: &Node) -> bool {
        if !self.0.matches(node) {
            return false;
        }
        for child in node.children() {
            if self.1.matches(&child) {
                return true;
            }
        }
        false
    }
} 

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextEquals<A: AsRef<str>>(pub A);

impl<A: AsRef<str>> Predicate for TextEquals<A> {
    fn matches(&self, node: &Node) -> bool {
        node.text() == self.0.as_ref()
    }
}

pub struct RefPredicate<'a>(&'a dyn Predicate);

impl<'a> Predicate for RefPredicate<'a> {
    fn matches(&self, node: &Node) -> bool {
        self.0.matches(node)
    }
}

pub struct BoxPred(Box<dyn Predicate>);

impl BoxPred {
    pub fn new<T: Predicate + 'static>(t: T) -> Self {
        Self(Box::new(t))
    }
}

impl Predicate for BoxPred {
    fn matches(&self, node: &Node) -> bool {
        self.0.matches(node)
    }
}

impl<A: Predicate, B: Predicate> Predicate for Sibling<A, B> {
    fn matches(&self, node: &Node) -> bool {
        if let Some(parent) = node.parent() {
            let mut last_child: Option<Node> = None;
            for child in parent.children() {
                if child.name().is_none() { continue; }
                if let Some(last_child2) = last_child {
                    if &child == node {
                        if self.0.matches(&last_child2) && self.1.matches(node) {
                            return true;
                        }
                    }
                }
                last_child = Some(child);
            }
            false
        } else {
            false
        }
    }
}

pub fn get_documentation_of_service(doc: &Document) -> Option<String> {
    let mut main_body = doc.find(
        Attr("id", "main-col-body")
    );
    let mut out = "".to_string();
    let main_body = main_body.next()?;
    for node in main_body.children() {
        if let Some(name) = node.name() {
            if name == "h2" { break }
            if name == "p" {
                let out_line = node.text();
                let out_line = out_line.trim().replace("\n", " ");
                let out_line = out_line.replace("  ", " ");
                out.push_str(&out_line);
                out.push('\n');
            }
        }
    }
    Some(out)
}

pub fn get_documentation_for_prop(doc: &Document, prop_id: &str) -> Option<String> {
    let mut dd_element = doc.find(
        Sibling(
            HasChild(Name("dt"), Attr("id", prop_id)),
            Name("dd")
        )
    );
    let elm = match dd_element.next() {
        Some(e) => e,
        None => {
            println!("Failed to find dt with child id={prop_id}");
            return None;
        }
    };
    let mut out = "".to_string();
    for p in elm.children() {
        let out_line = p.text();
        let out_line = out_line.trim().replace("\n", " ");
        let out_line = out_line.replace("  ", " ");
        out.push_str(&out_line);
        out.push('\n');
    }
    Some(out)
}

/// check if we have the rest of the filename cached already
/// if so, read it from cache. if not, download and cache it, and return the html file as a string.
pub fn download_and_cache(download_url: &str) -> String {
    let cached_name = basic_hash(download_url.as_bytes());
    let out_dir = "cfn_html";
    let _ = std::fs::create_dir(out_dir);
    let file_name = format!("{out_dir}/{}.html", cached_name);
    match std::fs::File::open(&file_name) {
        Ok(mut f) => {
            // if we're able to read it then we're good.
            // but if we fail, then we'll try to make a request.
            let mut out = "".to_string();
            if let Ok(_) = f.read_to_string(&mut out) {
                return out;
            }
        }
        Err(_) => {
            // we assume it doesnt exist, so we just make a request instead.
        }
    }
    // we dont have it cached so just try to download it
    let contents = match download_file(download_url) {
        Ok(d) => d,
        Err(e) => panic!("Failed to download {download_url}\n{:?}", e),
    };
    let s = String::from_utf8_lossy(&contents).to_string();
    if let Err(e) = std::fs::write(&file_name, &s) {
        panic!("Failed to write {download_url} to file {file_name}\n{:?}", e);
    }
    s
}

#[derive(Default)]
pub struct HtmlCache {
    pub from_url: Option<String>,
    pub contents: Option<Document>,
}

/// extract a specific doc comment via an id.
pub fn extract_documentation_specific(specific: &str, doc: &Document) -> Option<String> {
    // if the specific part is empty, that means we should extract just the top part.
    if specific.is_empty() {
        return get_documentation_of_service(doc);
    }
    get_documentation_for_prop(doc, specific)
}

pub fn extract_documentation(html_cache: &mut HtmlCache, link: &String) -> String {
    // static mut HTML_CACHE: HtmlCache = HtmlCache { from_url: None, contents: None };

    // given a link like https://{something}.html#specific
    // remove the #specific part
    let (download_url, specific) = if let Some((generic, specific)) = link.split_once("#") {
        (generic, specific)
    } else {
        (link.as_str(), "")
    };

    // check if we have the html object already cached in memory:
    if let (Some(url), Some(contents)) = (&html_cache.from_url, &html_cache.contents) {
        if url != download_url {
            html_cache.from_url = None;
            html_cache.contents = None;
        } else {
            match extract_documentation_specific(specific, contents) {
                Some(s) => return s,
                None => {
                    return format!("Failed to resolve {}", link)
                    // println!("From cached url {}", url);
                    // panic!("Failed to extract doc comment for {link}");
                }
            }
        }
    }
    // if we dont have it, or the cache is invalid, then we fetch it, extract
    // the specific part we want, and then cache these contents for future requests.
    let html_string = download_and_cache(download_url);
    let doc = Document::from(html_string.as_str());
    let out_s = extract_documentation_specific(specific, &doc);
    html_cache.from_url = Some(download_url.to_string());
    html_cache.contents = Some(doc);
    match out_s {
        Some(s) => return s,
        None => {
            return format!("Failed to resolve {}", link);
        }
    }
}
