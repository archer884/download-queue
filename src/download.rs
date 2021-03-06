use std::hash::{Hash, Hasher};

use url::Url;

#[derive(Debug, Eq)]
pub struct Download {
    pub idx: usize,
    pub host: String,
    pub url: String,
}

impl Download {
    pub fn with_index(idx: usize, s: &str) -> crate::Result<Self> {
        let s = s.trim();
        let url = Url::parse(s)?;
        let host = url
            .host_str()
            .ok_or("Unable to resolve host")?
            .trim_start_matches("www.")
            .to_owned();

        Ok(Download {
            idx: idx + 1, // Enumeration starts at 0, lines start at 1.
            host,
            url: s.to_owned(),
        })
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

impl Hash for Download {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.url.hash(hasher);
    }
}

impl PartialEq for Download {
    fn eq(&self, rhs: &Self) -> bool {
        self.url == rhs.url
    }
}
