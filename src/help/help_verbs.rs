use {
    crate::{
        app::AppContext,
        pattern::*,
        verb::*,
    },
};

/// what should be shown for a verb in the help screen, after
/// filtering
pub struct MatchingVerbRow<'v> {
    name: Option<String>,
    shortcut: Option<String>,
    pub verb: &'v Verb,
}

impl MatchingVerbRow<'_> {
    /// the name in markdown (with matching chars in bold if
    /// some filtering occured)
    pub fn name(&self) -> &str {
        // there should be a better way to write this
        self.name.as_deref().unwrap_or_else(|| match self.verb.names.get(0) {
            Some(s) => &s.as_str(),
            _ => " ",
        })
    }
    pub fn shortcut(&self) -> &str {
        // there should be a better way to write this
        self.shortcut.as_deref().unwrap_or_else(|| match self.verb.names.get(1) {
            Some(s) => &s.as_str(),
            _ => " ",
        })
    }
}

/// return the rows of the verbs table in help, taking the current filter
/// into account
pub fn matching_verb_rows<'v>(
    pat: &Pattern,
    con: &'v AppContext,
) -> Vec<MatchingVerbRow<'v>> {
    let mut rows = Vec::new();
    for verb in &con.verb_store.verbs {
        let mut name = None;
        let mut shortcut = None;
        if pat.is_some() {
            let mut ok = false;
            name = verb.names.get(0)
                .and_then(|s|
                    pat.search_string(s).map(|nm| {
                        ok = true;
                        nm.wrap(s, "**", "**")
                    })
                );
            shortcut = verb.names.get(1)
                .and_then(|s|
                    pat.search_string(s).map(|nm| {
                        ok = true;
                        nm.wrap(s, "**", "**")
                    })
                );
            if !ok {
                continue;
            }
        }
        rows.push(MatchingVerbRow {
            name,
            shortcut,
            verb,
        });
    }
    rows
}
