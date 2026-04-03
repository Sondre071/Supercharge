pub struct Section {
    pub title: String,
    pub content: String,
}

pub fn parse_sections(content: &str) -> Vec<Section> {
    let mut sections: Vec<Section> = Vec::new();
    let mut current_title: Option<String> = None;
    let mut current_lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            if let Some(t) = current_title.take() {
                sections.push(Section {
                    title: t,
                    content: current_lines.join("\n").trim().to_string(),
                });
            }
            current_title = Some(title.trim().to_string());
            current_lines = Vec::new();
        } else if current_title.is_some() {
            current_lines.push(line);
        }
    }

    if let Some(t) = current_title {
        sections.push(Section {
            title: t,
            content: current_lines.join("\n").trim().to_string(),
        });
    }

    sections
}
