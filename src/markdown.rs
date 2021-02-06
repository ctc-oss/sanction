use crate::grype::Match;
use crossterm::style::Color::*;
use termimad::*;

static HEADER: &str = r#"
|:--:|:--------:|---------|-----|-----|-----|
| OK | category | package | cve | fix | ref |
|:--:|:--------:|---------|-----|-----|-----|"#;

pub fn pretty_table(matches: Vec<&Match>, allowlist: Vec<String>) -> String {
    let full = markdown_table(matches, allowlist);

    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(Green);
    skin.strikeout.set_fg(Grey);
    skin.paragraph.align = Alignment::Center;
    skin.table.align = Alignment::Left;
    skin.term_text(&full).to_string()
}

pub fn markdown_table(matches: Vec<&Match>, allowlist: Vec<String>) -> String {
    let mut txt: Vec<String> = vec![];
    txt.push(HEADER.to_string());

    let empty = String::new();
    for m in matches.iter() {
        let f = &m.vulnerability;
        let fix = f.fixed_in_version.as_ref().unwrap_or(&empty);
        let wl = if allowlist.contains(&m.vulnerability.id) {
            "** ✔ **"
        } else {
            ""
        };
        let link = match f.links.first() {
            Some(l) => l,
            None => &empty,
        };
        let cve = if allowlist.contains(&f.id) {
            format!(" ~~ {} ~~ ", f.id)
        } else {
            f.id.to_string()
        };

        let row = format!(
            "| {} | {} | {} | {} | {} | {} |",
            wl, f.severity, m.artifact, cve, fix, link
        );
        txt.push(row);
    }

    txt.push("|-".to_string());
    txt.join("\n")
}
