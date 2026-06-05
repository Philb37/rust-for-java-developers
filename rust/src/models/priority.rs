pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

pub fn test() {
    let t = Priority::Low;
    let s: String = t.into();
}

impl Into<String> for Priority {
    fn into(self) -> String {
        match self {
            Priority::Low => "LOW".to_string(),
            Priority::Medium => "MEDIUM".to_string(),
            Priority::High => "HIGH".to_string(),
            Priority::Critical => "CRITICAL".to_string(),
        }
    }
}