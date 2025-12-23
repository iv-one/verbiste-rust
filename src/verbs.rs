use deunicode::deunicode;
use quick_xml::Reader;
use quick_xml::events::Event;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Verb {
    pub verb: String,
    pub template: String,
    pub aspirate_h: bool,
}

/// Normalize a string by removing accents and converting to lowercase
fn normalize_string(s: &str) -> String {
    deunicode(s).to_lowercase()
}

/// Index entry for fast accent-insensitive search
#[derive(Debug, Clone)]
struct NormalizedIndexEntry {
    normalized: String,
    index: usize,
}

/// Search index sorted by normalized verb form for fast binary search
#[derive(Debug)]
pub struct VerbSearchIndex {
    verbs: Vec<Verb>,
    normalized_index: Vec<NormalizedIndexEntry>,
}

impl VerbSearchIndex {
    /// Create a new search index from a vector of verbs
    pub fn new(mut verbs: Vec<Verb>) -> Self {
        // Sort verbs by original form (for exact matches)
        verbs.sort_by(|a, b| a.verb.cmp(&b.verb));

        // Build normalized index
        let mut normalized_index: Vec<NormalizedIndexEntry> = verbs
            .iter()
            .enumerate()
            .map(|(idx, verb)| NormalizedIndexEntry {
                normalized: normalize_string(&verb.verb),
                index: idx,
            })
            .collect();

        // Sort by normalized form for fast binary search
        normalized_index.sort_by(|a, b| a.normalized.cmp(&b.normalized));

        Self {
            verbs,
            normalized_index,
        }
    }

    /// Get the original verbs vector (sorted by original form)
    #[allow(dead_code)]
    pub fn verbs(&self) -> &Vec<Verb> {
        &self.verbs
    }

    /// Search for verbs matching the normalized query prefix
    /// Returns up to 20 results sorted by original verb name
    pub fn search(&self, query: &str) -> Vec<&Verb> {
        let query_normalized = normalize_string(query);
        let mut results = Vec::new();

        // Use binary search to find the first match
        let start_index = match self
            .normalized_index
            .binary_search_by(|entry| entry.normalized.cmp(&query_normalized))
        {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        // Find the first entry that starts with the query by scanning backwards
        let mut first_match = start_index;
        while first_match > 0
            && self.normalized_index[first_match - 1]
                .normalized
                .starts_with(&query_normalized)
        {
            first_match -= 1;
        }

        // Collect matches starting from first_match
        for entry in self.normalized_index.iter().skip(first_match) {
            if entry.normalized.starts_with(&query_normalized) {
                results.push(&self.verbs[entry.index]);
                if results.len() >= 20 {
                    break;
                }
            } else {
                // Since index is sorted by normalized form, we can stop here
                break;
            }
        }

        // Sort results by original verb name for consistent ordering
        results.sort_by(|a, b| a.verb.cmp(&b.verb));

        results
    }
}

pub fn load_all_verbs(xml_data: &str) -> Result<Vec<Verb>, Box<dyn std::error::Error>> {
    let mut xml_reader = Reader::from_str(xml_data);
    xml_reader.trim_text(true);

    let mut verbs = Vec::new();
    let mut buf = Vec::new();
    let mut current_verb: Option<String> = None;
    let mut current_template: Option<String> = None;
    let mut current_aspirate_h = false;
    let mut in_v = false;
    let mut in_i = false;
    let mut in_t = false;

    loop {
        match xml_reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"v" => {
                    in_v = true;
                    current_verb = None;
                    current_template = None;
                    current_aspirate_h = false;
                }
                b"i" => {
                    if in_v {
                        in_i = true;
                    }
                }
                b"t" => {
                    if in_v {
                        in_t = true;
                    }
                }
                b"aspirate-h" => {
                    if in_v {
                        current_aspirate_h = true;
                    }
                }
                _ => {}
            },
            Ok(Event::Empty(e)) => {
                if e.name().as_ref() == b"aspirate-h" && in_v {
                    current_aspirate_h = true;
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape()?.to_string();
                if in_i {
                    current_verb = Some(text);
                } else if in_t {
                    current_template = Some(text);
                }
            }
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"v" => {
                    if let (Some(verb), Some(template)) =
                        (current_verb.take(), current_template.take())
                    {
                        verbs.push(Verb {
                            verb: verb.trim().to_string(),
                            template: template.trim().to_string(),
                            aspirate_h: current_aspirate_h,
                        });
                    }
                    in_v = false;
                    current_aspirate_h = false;
                }
                b"i" => {
                    in_i = false;
                }
                b"t" => {
                    in_t = false;
                }
                _ => {}
            },
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    // Sort verbs by verb name for binary search
    verbs.sort_by(|a, b| a.verb.cmp(&b.verb));

    Ok(verbs)
}

/// Build a search index from loaded verbs for fast accent-insensitive search
pub fn build_search_index(verbs: Vec<Verb>) -> VerbSearchIndex {
    VerbSearchIndex::new(verbs)
}
