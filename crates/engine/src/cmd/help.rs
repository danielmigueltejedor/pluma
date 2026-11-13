//! Help → project links (Pluma's page on the Nodalia website and its GitHub repository). The
//! commands return the URL; the frontend opens it.

use serde_json::{Value, json};

use super::{CommandSpec, Result, always, cmd};

/// Pluma's page on the Nodalia website.
pub const WEBSITE_URL: &str = "https://getnodalia.com/pluma";

/// Pluma's source repository.
pub const GITHUB_URL: &str = "https://github.com/getnodalia/pluma";

/// This app's page on the website.
pub fn app_page_url() -> String {
    WEBSITE_URL.into()
}

/// This app's source repository.
pub fn github_url() -> String {
    GITHUB_URL.into()
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.website", "Pluma Website", ["Help"], None, "{} → {url} Pluma's page on the Nodalia website", always, |_, _| url(app_page_url())),
        cmd!(query "help.github", "Pluma on GitHub", ["Help"], None, "{} → {url} source code, issues and releases", always, |_, _| url(github_url())),
        cmd!(query "help.links", "Links", [], None, "{} → {website, github}", always, |_, _| {
            Ok(json!({ "website": app_page_url(), "github": github_url() }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.website", &json!({})).unwrap()["url"], "https://getnodalia.com/pluma");
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], "https://github.com/getnodalia/pluma");
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["website"], "https://getnodalia.com/pluma");
        assert!(s.execute("help.discord", &json!({})).is_err());
    }
}
