use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize
}

#[derive(Debug)]
struct Rule {
    sample: char,
    op: char,
    value: usize,
    dest: String,
}

fn parse_part(line: &str) -> Part {
    let mut split = line[1..line.len()-1].split(",");
    let x = split.next().unwrap()[2..].parse().unwrap();
    let m = split.next().unwrap()[2..].parse().unwrap();
    let a = split.next().unwrap()[2..].parse().unwrap();
    let s = split.next().unwrap()[2..].parse().unwrap();

    Part {x,m,a,s}
}

fn parse_workflow(line: &str) -> (String, Vec<Rule>) {
    let work_start = line.find("{").unwrap();
    let key = line[..work_start].to_owned();

    let mut rules = Vec::new();
    for rule_string in line[work_start+1..line.len()-1].split(",") {
        if let Some(colon) = rule_string.find(":") {
            let sample = rule_string.chars().nth(0).unwrap();
            let op = rule_string.chars().nth(1).unwrap();
            let value = rule_string[2..colon].parse().unwrap();
            let dest = rule_string[colon+1..].to_owned();
            rules.push(Rule{sample, op, value, dest});
        } else {
            rules.push(Rule {sample: '.', op: 'x', value: 0, dest: rule_string.to_owned()});
        }
    }
    (key, rules)
}

fn evaluate_part(part: &Part, workflows: &HashMap<String, Vec<Rule>>) -> usize {
    let mut current_workflow = "in".to_owned();

    while !(current_workflow == "A" || current_workflow == "R") {
        let workflow = workflows.get(&current_workflow).unwrap();
        for rule in workflow {
            let value = match rule.sample {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                '.' => {
                    current_workflow = rule.dest.clone();
                    break;
                }
                _ => panic!("eh?")
            };

            let matches = match rule.op {
                '>' => value > rule.value,
                '<' => value < rule.value,
                _ => panic!("eh?")
            };

            if matches {
                current_workflow = rule.dest.clone();
                break;
            }
        }
    }

    match current_workflow.as_str() {
        "A" => part.x + part.m + part.a + part.s,
        "R" => 0,
        _=> panic!("eh?")
    }
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();

    let mut parts: Vec<Part> = Vec::new();
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();

    for line in input.lines() {
        if line == "" {
            continue;
        }
        if line.starts_with("{") {
            parts.push(parse_part(line));
        } else {
            let (key, workflow) = parse_workflow(line);
            workflows.insert(key, workflow);
        }
    }
    let mut sum1 = 0;
    for part in parts {
        let value = evaluate_part(&part, &workflows);
        //println!("value: {:}", value);
        sum1+=value;
    }
    println!("q1: {:}", sum1);
}
