use http::Uri;
use http::uri::Scheme;

pub fn open(url: &str) -> std::io::Result<()> {
    open(String::from(path))
}

pub fn open(url: String) -> std::io::Result<()> {
    open(Path::new(path))
}

pub fn open(url: Path) -> std::io::Result<()> {
    let request = Request::builder()
      .uri("https://www.rust-lang.org/")
      .header("User-Agent", "awesome/1.0")
      .body(())
      .unwrap();
}
