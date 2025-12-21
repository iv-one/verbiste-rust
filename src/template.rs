use quick_xml::Reader;
use quick_xml::events::Event;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Template {
    pub name: String,
    pub infinitive: Infinitive,
    pub indicative: Indicative,
    pub conditional: Conditional,
    pub subjunctive: Subjunctive,
    pub imperative: Imperative,
    pub participle: Participle,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Infinitive {
    pub infinitive_present: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Indicative {
    pub present: Vec<Vec<String>>,
    pub imperfect: Vec<Vec<String>>,
    pub future: Vec<Vec<String>>,
    pub simple_past: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Conditional {
    pub present: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Subjunctive {
    pub present: Vec<Vec<String>>,
    pub imperfect: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Imperative {
    pub imperative_present: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Participle {
    pub present_participle: Vec<String>,
    pub past_participle: Vec<Vec<String>>,
}

pub fn load_all_templates(
    xml_data: &str,
) -> Result<HashMap<String, Template>, Box<dyn std::error::Error>> {
    let mut xml_reader = Reader::from_str(xml_data);
    xml_reader.trim_text(true);

    let mut templates = HashMap::new();
    let mut buf = Vec::new();

    // State tracking
    let mut current_template_name: Option<String> = None;
    let mut current_infinitive_present = Vec::new();
    let mut current_present = Vec::new();
    let mut current_imperfect = Vec::new();
    let mut current_future = Vec::new();
    let mut current_simple_past = Vec::new();
    let mut current_conditional_present = Vec::new();
    let mut current_subjunctive_present = Vec::new();
    let mut current_subjunctive_imperfect = Vec::new();
    let mut current_imperative_present = Vec::new();
    let mut current_present_participle = Vec::new();
    let mut current_past_participle = Vec::new();

    // Nested state
    let mut current_p_elements = Vec::new();
    let mut in_p = false;
    let mut in_i = false;

    // Section tracking
    let mut in_indicative = false;
    let mut in_conditional = false;
    let mut in_subjunctive = false;
    let mut current_section: Option<&str> = None;

    loop {
        match xml_reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"template" => {
                    for attr in e.attributes() {
                        let attr = attr?;
                        if attr.key.as_ref() == b"name" {
                            current_template_name = Some(String::from_utf8(attr.value.to_vec())?);
                        }
                    }
                }
                b"indicative" => {
                    in_indicative = true;
                }
                b"conditional" => {
                    in_conditional = true;
                }
                b"subjunctive" => {
                    in_subjunctive = true;
                }
                b"infinitive-present" => {
                    current_section = Some("infinitive-present");
                    current_infinitive_present.clear();
                }
                b"present" => {
                    if in_indicative {
                        current_section = Some("indicative-present");
                        current_present.clear();
                    } else if in_conditional {
                        current_section = Some("conditional-present");
                        current_conditional_present.clear();
                    } else if in_subjunctive {
                        current_section = Some("subjunctive-present");
                        current_subjunctive_present.clear();
                    }
                }
                b"imperfect" => {
                    if in_indicative {
                        current_section = Some("indicative-imperfect");
                        current_imperfect.clear();
                    } else if in_subjunctive {
                        current_section = Some("subjunctive-imperfect");
                        current_subjunctive_imperfect.clear();
                    }
                }
                b"future" => {
                    current_section = Some("indicative-future");
                    current_future.clear();
                }
                b"simple-past" => {
                    current_section = Some("indicative-simple-past");
                    current_simple_past.clear();
                }
                b"imperative-present" => {
                    current_section = Some("imperative-present");
                    current_imperative_present.clear();
                }
                b"present-participle" => {
                    current_section = Some("present-participle");
                    current_present_participle.clear();
                }
                b"past-participle" => {
                    current_section = Some("past-participle");
                    current_past_participle.clear();
                }
                b"p" => {
                    in_p = true;
                    current_p_elements.clear();
                }
                b"i" => {
                    in_i = true;
                }
                _ => {}
            },
            Ok(Event::Text(e)) => {
                if in_i && in_p {
                    let text = e.unescape()?.to_string();
                    current_p_elements.push(text.trim().to_string());
                }
            }
            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"template" => {
                        if let Some(name) = current_template_name.take() {
                            templates.insert(
                                name.clone(),
                                Template {
                                    name: name.clone(),
                                    infinitive: Infinitive {
                                        infinitive_present: current_infinitive_present.clone(),
                                    },
                                    indicative: Indicative {
                                        present: current_present.clone(),
                                        imperfect: current_imperfect.clone(),
                                        future: current_future.clone(),
                                        simple_past: current_simple_past.clone(),
                                    },
                                    conditional: Conditional {
                                        present: current_conditional_present.clone(),
                                    },
                                    subjunctive: Subjunctive {
                                        present: current_subjunctive_present.clone(),
                                        imperfect: current_subjunctive_imperfect.clone(),
                                    },
                                    imperative: Imperative {
                                        imperative_present: current_imperative_present.clone(),
                                    },
                                    participle: Participle {
                                        present_participle: current_present_participle.clone(),
                                        past_participle: current_past_participle.clone(),
                                    },
                                },
                            );
                        }
                        // Reset all state
                        current_infinitive_present.clear();
                        current_present.clear();
                        current_imperfect.clear();
                        current_future.clear();
                        current_simple_past.clear();
                        current_conditional_present.clear();
                        current_subjunctive_present.clear();
                        current_subjunctive_imperfect.clear();
                        current_imperative_present.clear();
                        current_present_participle.clear();
                        current_past_participle.clear();
                        in_indicative = false;
                        in_conditional = false;
                        in_subjunctive = false;
                        current_section = None;
                    }
                    b"indicative" => {
                        in_indicative = false;
                    }
                    b"conditional" => {
                        in_conditional = false;
                    }
                    b"subjunctive" => {
                        in_subjunctive = false;
                    }
                    b"p" => {
                        in_p = false;
                        if !current_p_elements.is_empty() {
                            match current_section {
                                Some("infinitive-present") => {
                                    current_infinitive_present.extend(current_p_elements.clone());
                                }
                                Some("indicative-present") => {
                                    current_present.push(current_p_elements.clone());
                                }
                                Some("indicative-imperfect") => {
                                    current_imperfect.push(current_p_elements.clone());
                                }
                                Some("indicative-future") => {
                                    current_future.push(current_p_elements.clone());
                                }
                                Some("indicative-simple-past") => {
                                    current_simple_past.push(current_p_elements.clone());
                                }
                                Some("conditional-present") => {
                                    current_conditional_present.push(current_p_elements.clone());
                                }
                                Some("subjunctive-present") => {
                                    current_subjunctive_present.push(current_p_elements.clone());
                                }
                                Some("subjunctive-imperfect") => {
                                    current_subjunctive_imperfect.push(current_p_elements.clone());
                                }
                                Some("imperative-present") => {
                                    current_imperative_present.push(current_p_elements.clone());
                                }
                                Some("present-participle") => {
                                    current_present_participle.extend(current_p_elements.clone());
                                }
                                Some("past-participle") => {
                                    current_past_participle.push(current_p_elements.clone());
                                }
                                _ => {}
                            }
                        }
                        current_p_elements.clear();
                    }
                    b"i" => {
                        in_i = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(templates)
}
