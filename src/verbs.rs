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
