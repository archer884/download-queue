use error::Result;
use url::Url;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Download {
    host: String,
    url: String,
}

impl Download {
    pub fn from_url(s: &str) -> Result<Self> {
        let s = s.trim();
        let url = Url::parse(s)?;
        let host = url.host_str()
            .ok_or("Unable to resolve host")?
            .trim_left_matches("www.")
            .to_owned();

        Ok(Download {
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

#[cfg(test)]
mod tests {
    use download::Download;

    #[test]
    fn it_works() {
        let url = "http://www.contoso.com/corporate-event-video";
        let result = Download::from_url(url).expect("Failed to read url");
        assert_eq!("contoso.com", result.host());
        assert_eq!(url, result.url());
    }
}
