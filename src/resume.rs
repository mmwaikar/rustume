use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Resume {
    #[serde(default)]
    pub basics: Basics,
    #[serde(default)]
    pub work: Vec<WorkEntry>,
    #[serde(default)]
    pub skills: Vec<Skill>,
    #[serde(default)]
    pub projects: Vec<Project>,
    #[serde(default)]
    pub education: Vec<Education>,
    #[serde(default)]
    pub publications: Vec<Publication>,
    #[serde(default)]
    pub languages: Vec<Language>,
    #[serde(default)]
    pub interests: Vec<Interest>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Basics {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub location: Option<BasicsLocation>,
    #[serde(default)]
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct BasicsLocation {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub postal_code: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub region: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Profile {
    #[serde(default)]
    pub network: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Education {
    #[serde(default)]
    pub institution: String,
    #[serde(default)]
    pub area: String,
    #[serde(rename = "studyType", default)]
    pub study_type: String,
    #[serde(rename = "startDate", default)]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub courses: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Publication {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub publisher: String,
    #[serde(rename = "releaseDate", default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Language {
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub fluency: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Interest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Project {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct WorkEntry {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub position: String,
    #[serde(rename = "startDate", default)]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Location {
    Text(String),
    Structured {
        #[serde(rename = "countryCode", default)]
        country_code: Option<String>,
        #[serde(default)]
        country: Option<String>,
        #[serde(flatten)]
        _other: BTreeMap<String, serde_json::Value>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct Skill {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct YearMonth {
    pub year: i32,
    pub month: u32,
}

impl YearMonth {
    pub fn parse(value: &str) -> Result<Self, ResumeError> {
        let date = NaiveDate::parse_from_str(&format!("{value}-01"), "%Y-%m-%d")
            .map_err(|_| ResumeError::InvalidDate(value.to_owned()))?;
        Ok(Self {
            year: date.year(),
            month: date.month(),
        })
    }

    pub fn current() -> Self {
        let now = Local::now().date_naive();
        Self {
            year: now.year(),
            month: now.month(),
        }
    }
}

pub fn inclusive_months(start: YearMonth, end: YearMonth) -> u32 {
    if end < start {
        return 0;
    }
    ((end.year - start.year) * 12 + end.month as i32 - start.month as i32 + 1) as u32
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResumeError {
    Json(String),
    Io(String),
    InvalidDate(String),
}

impl fmt::Display for ResumeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => write!(formatter, "resume JSON could not be loaded: {message}"),
            Self::Io(message) => write!(formatter, "resume file could not be loaded: {message}"),
            Self::InvalidDate(value) => write!(formatter, "invalid resume date: {value}"),
        }
    }
}

impl std::error::Error for ResumeError {}

pub fn parse_resume(input: &str) -> Result<Resume, ResumeError> {
    let value: serde_json::Value =
        serde_json::from_str(input).map_err(|error| ResumeError::Json(error.to_string()))?;
    if !value.is_object() {
        return Err(ResumeError::Json(
            "resume document must be a JSON object".to_owned(),
        ));
    }
    serde_json::from_value(value).map_err(|error| ResumeError::Json(error.to_string()))
}

pub fn load_resume(path: impl AsRef<Path>) -> Result<Resume, ResumeError> {
    let input =
        std::fs::read_to_string(path).map_err(|error| ResumeError::Io(error.to_string()))?;
    parse_resume(&input)
}

pub fn work_duration_months(work: &WorkEntry, current: YearMonth) -> Result<u32, ResumeError> {
    let Some(start) = work.start_date.as_deref() else {
        return Ok(0);
    };
    let start = YearMonth::parse(normalize_date(start).as_str())?;
    let end = work
        .end_date
        .as_deref()
        .map(|value| YearMonth::parse(normalize_date(value).as_str()))
        .transpose()?
        .unwrap_or(current);
    Ok(inclusive_months(start, end))
}

pub fn company_months(
    resume: &Resume,
    current: YearMonth,
) -> Result<BTreeMap<String, u32>, ResumeError> {
    let mut totals = BTreeMap::new();
    for work in &resume.work {
        let name = if work.name.is_empty() {
            "Unknown company".to_owned()
        } else {
            work.name.clone()
        };
        *totals.entry(name).or_insert(0) += work_duration_months(work, current)?;
    }
    Ok(totals)
}

pub fn country_for_location(location: Option<&Location>) -> String {
    let candidate = match location {
        Some(Location::Text(value)) => value.rsplit(',').next().unwrap_or(value).trim(),
        Some(Location::Structured {
            country_code,
            country,
            ..
        }) => country_code
            .as_deref()
            .or(country.as_deref())
            .unwrap_or("")
            .trim(),
        None => "",
    };
    if candidate.is_empty() {
        return "Unknown".to_owned();
    }
    candidate.to_owned()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub group: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphPayload {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

fn graph_id(group: &str, label: &str) -> String {
    format!("{group}:{}", label.trim().to_lowercase().replace(' ', "-"))
}

pub fn geography_graph(resume: &Resume) -> GraphPayload {
    let mut nodes = BTreeMap::new();
    let mut edges = Vec::new();
    for work in &resume.work {
        let company = if work.name.is_empty() {
            "Unknown company"
        } else {
            &work.name
        };
        let country = country_for_location(work.location.as_ref());
        let company_id = graph_id("company", company);
        let country_id = graph_id("country", &country);
        nodes.entry(company_id.clone()).or_insert(GraphNode {
            id: company_id.clone(),
            label: company.to_owned(),
            group: "company".to_owned(),
        });
        nodes.entry(country_id.clone()).or_insert(GraphNode {
            id: country_id.clone(),
            label: country,
            group: "country".to_owned(),
        });
        edges.push(GraphEdge {
            id: format!("{company_id}->{country_id}"),
            source: company_id,
            target: country_id,
            label: None,
        });
    }
    GraphPayload {
        nodes: nodes.into_values().collect(),
        edges,
    }
}

pub fn skills_graph(resume: &Resume) -> GraphPayload {
    let mut nodes = BTreeMap::new();
    let mut keywords = Vec::new();
    for skill in &resume.skills {
        let skill_id = graph_id("skill", &skill.name);
        nodes.entry(skill_id).or_insert(GraphNode {
            id: graph_id("skill", &skill.name),
            label: skill.name.clone(),
            group: "skill".to_owned(),
        });
        keywords.push((
            skill.name.clone(),
            skill
                .keywords
                .iter()
                .map(|value| value.to_lowercase())
                .collect::<BTreeSet<_>>(),
        ));
    }
    let mut edges = Vec::new();
    for (index, (left, left_keywords)) in keywords.iter().enumerate() {
        for (right, right_keywords) in keywords.iter().skip(index + 1) {
            if left_keywords.intersection(right_keywords).next().is_some() {
                let source = graph_id("skill", left);
                let target = graph_id("skill", right);
                edges.push(GraphEdge {
                    id: format!("{source}->{target}"),
                    source,
                    target,
                    label: Some("shared keyword".to_owned()),
                });
            }
        }
    }
    GraphPayload {
        nodes: nodes.into_values().collect(),
        edges,
    }
}

fn normalize_date(value: &str) -> String {
    match value.len() {
        4 => format!("{value}-01"),
        _ => value[..7.min(value.len())].to_owned(),
    }
}

pub fn wordpress_publications(resume: &Resume) -> Vec<&Publication> {
    resume
        .publications
        .iter()
        .filter(|publication| publication.publisher.eq_ignore_ascii_case("wordpress"))
        .collect()
}

pub fn has_network_profile(resume: &Resume) -> bool {
    resume
        .basics
        .profiles
        .iter()
        .any(|profile| !profile.url.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Resume {
        parse_resume(r#"{
            "basics": {"name": "Ada Lovelace"},
            "work": [
                {"name": "Analytical Engines", "startDate": "2020-01", "endDate": "2020-03", "location": "London, United Kingdom"},
                {"name": "Analytical Engines", "startDate": "2021-01", "endDate": "2021-01"},
                {"name": "Difference Works", "startDate": "2020-01", "endDate": "2020-12", "location": {"countryCode": "US"}}
            ],
            "skills": [{"name": "Rust", "keywords": ["systems"]}, {"name": "GPUI", "keywords": ["systems"]}, {"name": "Writing"}]
        }"#).unwrap()
    }

    #[test]
    fn parses_valid_resume_and_rejects_non_object() {
        assert_eq!(sample().basics.name, "Ada Lovelace");
        assert!(parse_resume("[]").is_err());
        assert!(parse_resume("not json").is_err());
    }

    #[test]
    fn computes_inclusive_and_equal_months() {
        let current = YearMonth {
            year: 2022,
            month: 1,
        };
        assert_eq!(
            inclusive_months(
                YearMonth {
                    year: 2020,
                    month: 1
                },
                YearMonth {
                    year: 2020,
                    month: 3
                }
            ),
            3
        );
        let resume = sample();
        let totals = company_months(&resume, current).unwrap();
        assert_eq!(totals["Analytical Engines"], 4);
        assert_eq!(totals["Difference Works"], 12);
    }

    #[test]
    fn preserves_unknown_country_and_isolated_skill() {
        let resume = sample();
        let geography = geography_graph(&resume);
        assert!(geography.nodes.iter().any(|node| node.label == "Unknown"));
        let skills = skills_graph(&resume);
        assert!(skills.nodes.iter().any(|node| node.label == "Writing"));
        assert_eq!(skills.edges.len(), 1);
        assert!(skills.edges.iter().all(|edge| skills
            .nodes
            .iter()
            .any(|node| node.id == edge.source)
            && skills.nodes.iter().any(|node| node.id == edge.target)));
    }

    #[test]
    fn parses_actual_resume_content_sections() {
        let resume = parse_resume(include_str!("../assets/resume.json")).unwrap();
        assert_eq!(resume.basics.name, "Manoj Waikar");
        assert!(!resume.basics.profiles.is_empty());
        assert!(!resume.education.is_empty());
        assert!(!resume.publications.is_empty());
        assert!(!resume.languages.is_empty());
        assert!(!resume.interests.is_empty());
        assert!(!resume.projects.is_empty());
        assert_eq!(wordpress_publications(&resume).len(), 4);
        assert!(has_network_profile(&resume));
    }
}
