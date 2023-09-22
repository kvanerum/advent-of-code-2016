use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::ops::Not;

fn main() {
    let input: Vec<String> = fs::read_to_string("day7/resources/input.txt")
        .expect("read input file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let regex_out_brackets = Regex::new(r"([^\[\]]+)(?:$|\[)").expect("parse regex");
    let regex_in_brackets = Regex::new(r"\[(\w+)]").expect("parse regex");

    let mut count = input
        .iter()
        .filter(|s| supports_tls(s, &regex_out_brackets, &regex_in_brackets))
        .count();

    println!("{}", count);

    count = input
        .iter()
        .filter(|s| supports_ssl(s, &regex_out_brackets, &regex_in_brackets))
        .count();

    println!("{}", count);
}

fn supports_tls(ip: &str, regex_out_brackets: &Regex, regex_in_brackets: &Regex) -> bool {
    let out_brackets: Vec<_> = regex_out_brackets
        .captures_iter(ip)
        .map(|c| c.extract())
        .map(|(_, [x])| x)
        .collect();

    let in_brackets: Vec<_> = regex_in_brackets
        .captures_iter(ip)
        .map(|c| c.extract())
        .map(|(_, [x])| x)
        .collect();

    return out_brackets.iter().any(|s| contains_abba(s))
        && in_brackets.iter().any(|s| contains_abba(s)).not();
}

fn supports_ssl(ip: &str, regex_out_brackets: &Regex, regex_in_brackets: &Regex) -> bool {
    let out_brackets: Vec<&str> = regex_out_brackets
        .captures_iter(ip)
        .map(|c| c.extract())
        .map(|(_, [x])| x)
        .collect();

    let in_brackets: Vec<_> = regex_in_brackets
        .captures_iter(ip)
        .map(|c| c.extract())
        .map(|(_, [x])| x)
        .collect();

    let ab_pairs: HashSet<_> = out_brackets
        .iter()
        .flat_map(|s| get_aba_pairs(&s))
        .collect();

    return in_brackets.iter().any(|s| contains_bab(s, &ab_pairs));
}

fn contains_abba(str: &str) -> bool {
    if str.len() < 4 {
        return false;
    }

    for i in 0..(str.chars().count() - 3) {
        let c0 = str.chars().nth(i).unwrap();
        let c1 = str.chars().nth(i + 1).unwrap();
        let c2 = str.chars().nth(i + 2).unwrap();
        let c3 = str.chars().nth(i + 3).unwrap();

        if c0 != c1 && c0 == c3 && c1 == c2 {
            return true;
        }
    }

    return false;
}

fn get_aba_pairs(str: &str) -> HashSet<(char, char)> {
    let mut result = HashSet::new();
    if str.len() < 3 {
        return result;
    }

    for i in 0..(str.chars().count() - 2) {
        let c0 = str.chars().nth(i).unwrap();
        let c1 = str.chars().nth(i + 1).unwrap();
        let c2 = str.chars().nth(i + 2).unwrap();

        if c0 != c1 && c0 == c2 {
            result.insert((c0, c1));
        }
    }

    return result;
}

fn contains_bab(str: &str, pairs: &HashSet<(char, char)>) -> bool {
    if str.len() < 3 {
        return false;
    }

    for p in pairs {
        let needle = format!("{b}{a}{b}", a = p.0, b = p.1);

        if str.contains(&needle) {
            return true;
        }
    }

    return false;
}
