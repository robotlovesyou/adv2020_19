use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static!{
    static ref MESSAGE_REGEX: Regex = Regex::new(r"^\w+$").unwrap();
    static ref REPLACE_REGEX: Regex = Regex::new(r"\d+").unwrap();
    static ref LITERAL_REGEX: Regex = Regex::new("\"(?P<char>\\w)\"").unwrap();
}


fn read_rules<'a>(lines: impl Iterator<Item=&'a str>) -> HashMap<usize, String> {
    let mut rules = HashMap::new();
    for line in lines {
        if let Some(at) = line.find(':') {
            let (id_str, rule_str) = line.split_at(at);
            let id = id_str.parse::<usize>().expect("invalid id");
            let to_trim: &[_] = &[':', ' '];
            let rule = rule_str.trim_start_matches(to_trim).to_string();
            rules.insert(id, rule);
        } else {
            break
        }
    }
    rules
}

fn read_messages<'a>(lines: impl Iterator<Item=&'a str>) -> Vec<String> {
    let mut messages = Vec::new();
    for line in lines {
        if MESSAGE_REGEX.is_match(line) {
            messages.push(line.to_string());
        }
    }
    messages
}



fn main() {
    let mut rules = read_rules(include_str!("../input.txt").lines());
    let messages = read_messages(include_str!("../input.txt").lines());

    /**************************************************************************
    PART ONE
    **************************************************************************/
    // this is my solution
    let merged_regex = old_apply_rules(&mut rules);
    let ok_count = messages.iter().filter(|message| merged_regex.is_match(*message)).count();
    println!("there are {} matching messages", ok_count);

    // this is ported from Sophie Alpert's solution here https://github.com/sophiebits/adventofcode/blob/main/2020/day19.py
    // because my approach doesn't work for creating the necessary regex
    let merged_regex = Regex::new(&format!("^{}$", apply_rules(&mut rules, 0))).expect("invalid regex");
    let ok_count = messages.iter().filter(|message| merged_regex.is_match(*message)).count();
    println!("there are {} matching messages", ok_count);



    /**************************************************************************
    PART TWO
    this is ported from Sophie Alpert's solution here https://github.com/sophiebits/adventofcode/blob/main/2020/day19.py
    because my approach doesn't work for creating the necessary regex
    **************************************************************************/
    let merged_regex = Regex::new(&format!("^{}$", apply_rules_2(&mut rules, 0))).expect("invalid regex");
    let ok_count = messages.iter().filter(|message| merged_regex.is_match(*message)).count();
    println!("there are {} matching messages", ok_count);
}

fn make_rule_8(rules: &mut HashMap<usize, String>) -> String {
    format!("{}+", apply_rules_2(rules, 42))
}

fn make_rule_11(rules: &mut HashMap<usize, String>) -> String {
    let mut rule_11 = Vec::new();
    let a = apply_rules_2(rules, 42);
    let b: String = apply_rules_2(rules, 31);
    for n in 1..=9 {
        rule_11.push(format!("{}{{{}}}{}{{{}}}", a, n, b, n));
    }
    let joined = format!("(:?{})", rule_11.join("|"));
    joined
}

fn apply_rules(rules: &mut HashMap<usize, String>, id: usize) -> String {
    let rule = rules[&id].clone();
    let result = if let Some(lit_cap) = LITERAL_REGEX.captures(&rule) {
        lit_cap["char"].to_string()
    } else if rule.contains('|') {
        format!("(:?{})", rule.split('|')
            .map(|part| apply_sequence(rules, part.to_string()))
            .collect::<Vec<String>>()
            .join("|")
        )
    } else {
        apply_sequence(rules, rule)
    };
    result
}


fn apply_rules_2(rules: &mut HashMap<usize, String>, id: usize) -> String {
    let rule = rules[&id].clone();
    if id == 8 {
        return make_rule_8(rules);
    }
    if id == 11 {
        return make_rule_11(rules);
    }
    let result = if let Some(lit_cap) = LITERAL_REGEX.captures(&rule) {
        lit_cap["char"].to_string()
    } else if rule.contains('|') {
        format!("(:?{})", rule.split('|')
            .map(|part| apply_sequence_2(rules, part.to_string()))
            .collect::<Vec<String>>()
            .join("|")
        )
    } else {
        apply_sequence_2(rules, rule)
    };
    result
}

fn apply_sequence(rules: &mut HashMap<usize, String>, rule: String) -> String {
    REPLACE_REGEX
        .captures_iter(&rule)
        .map(|num_cap| apply_rules(rules, num_cap[0].parse::<usize>().unwrap()))
        .collect::<Vec<String>>()
        .join("")
}

fn apply_sequence_2(rules: &mut HashMap<usize, String>, rule: String) -> String {
    REPLACE_REGEX
        .captures_iter(&rule)
        .map(|num_cap| apply_rules_2(rules, num_cap[0].parse::<usize>().unwrap()))
        .collect::<Vec<String>>()
        .join("")
}

fn old_apply_rules(rules: &mut HashMap<usize, String>) -> Regex {
    let mut merged = rules[&0usize].clone();
    while REPLACE_REGEX.is_match(&merged) {
        let replaced = REPLACE_REGEX.replace(&merged, |caps: &Captures| {
            let id = caps[caps.len()-1].parse::<usize>().expect("cannot parse id");
            let rule = &rules[&id];
            if let Some(lit_cap) = LITERAL_REGEX.captures(&rule) {
                lit_cap["char"].to_string()
            } else if rule.contains('|') {
                format!("({})", rule)
            } else {
                rule.clone()
            }
        });
        merged = replaced.to_string();
    }
    let merged_regex = Regex::new(
        &format!(
            "^{}$",
            merged.chars().filter(|c| *c != ' ').collect::<String>())
    ).expect("invalid merged regex");
    merged_regex
}
