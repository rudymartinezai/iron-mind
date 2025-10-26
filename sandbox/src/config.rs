use std::fs;

use crate::utils;

pub fn load_rules(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).unwrap_or_else(|_| {
        utils::log_using_mock_config(path);
        utils::default_config().to_owned()
    });

    parse_rules(&contents)
}

fn parse_rules(contents: &str) -> Vec<String> {
    contents
        .lines()
        .filter_map(parse_rule)
        .collect()
}

fn parse_rule(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    let mut parts = trimmed.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some("deny"), Some(pattern)) => Some(pattern.to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::parse_rules;

    #[test]
    fn parses_deny_lines() {
        let config = "
            # comment
            deny 192.168.1.
            allow 10.0.0.
            deny 172.16.
        ";
        let rules = parse_rules(config);
        assert_eq!(rules, vec!["192.168.1.".to_string(), "172.16.".to_string()]);
    }
}
