use crate::vuln::Match;

pub fn dump_table(matches: Vec<&Match>, allowlist: Vec<String>) {
    println!("| A | category | package | cve | fix |");
    println!("|---|----------|---------|-----|-----|");
    let empty = String::new();
    for m in matches.iter() {
        let f = &m.vulnerability;
        let fix = f.fixed_in_version.as_ref().unwrap_or(&empty);
        let wl = if allowlist.contains(&m.vulnerability.id) {
            "X"
        } else {
            ""
        };
        let cve = match f.links.first() {
            Some(l) => format!("[{}]({})", f.id, l),
            None => "N/A".to_string(),
        };
        println!(
            "| {} | {} | {} | {} | {} |",
            wl, f.severity, m.artifact, cve, fix
        );
    }
}
