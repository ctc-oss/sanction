use crate::vuln::Match;
use termimad::*;
use crossterm::style::Color::*;

static HEADER: &str = r#"
|:-:|:--------:|---------|-----|-----
|   | category | package | cve | fix |
|---|----------|---------|-----|-----"#;

pub fn dump_table(matches: Vec<&Match>, allowlist: Vec<String>) {
    let mut txt: Vec<String> = vec![];
    txt.push(HEADER.to_string());

    let empty = String::new();
    for m in matches.iter() {
        let f = &m.vulnerability;
        let fix = f.fixed_in_version.as_ref().unwrap_or(&empty);
        let wl = if allowlist.contains(&m.vulnerability.id) {
            "*X*"
        } else {
            ""
        };
        let cve = match f.links.first() {
            Some(l) => format!("[{}]({})", f.id, l),
            None => "N/A".to_string(),
        };
        let row = format!(
            "| ** {} ** | ~~ {} ~~ | {} | {} | {} |",
            wl, f.severity, m.artifact, cve, fix
        );
        txt.push(row);
    }

    txt.push("|-".to_string());
    let full: String = txt.join("\n");

    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(Yellow);
    skin.paragraph.align = Alignment::Center;
    skin.table.align = Alignment::Left;
    println!("{}", skin.term_text(&full));
}
