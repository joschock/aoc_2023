use std::{fs::read_to_string, collections::HashMap, ops::Range};

#[derive(Debug, Default, Clone)]
struct Part {
    ranges: [Range<usize>; 4]
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
    let x = x..x+1;
    let m = split.next().unwrap()[2..].parse().unwrap();
    let m = m..m+1;
    let a = split.next().unwrap()[2..].parse().unwrap();
    let a = a..a+1;
    let s = split.next().unwrap()[2..].parse().unwrap();
    let s = s..s+1;
    Part {ranges: [x,m,a,s]}
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

fn split_parts(current_parts: &Part, rule: &Rule) -> (Part, Part) {
    let split_range = match rule.sample {
        '.' => return (current_parts.clone(), Default::default()),
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("eh?")
    };

    match rule.op {
        '<' => {
            let current_start = current_parts.ranges[split_range].start;
            let current_end = current_parts.ranges[split_range].end;

            if rule.value >= current_end {
                //matches everything in the current range
                (current_parts.clone(), Default::default())
            } else if rule.value < current_start {
                //matches nothing in the current range
                (Default::default(), current_parts.clone())
            } else {
                let mut applied_parts = current_parts.clone();
                let mut not_applied_parts = current_parts.clone();

                applied_parts.ranges[split_range] = current_start..rule.value;
                not_applied_parts.ranges[split_range] = rule.value..current_end;

                (applied_parts, not_applied_parts)
            }
        },
        '>' => {
            let current_start = current_parts.ranges[split_range].start;
            let current_end = current_parts.ranges[split_range].end;

            if rule.value >= current_end {
                //matches nothing  in the current range
                (Default::default(), current_parts.clone())
            } else if rule.value < current_start {
                //matches everything in the current range
                (current_parts.clone(), Default::default())
            } else {
                let mut applied_parts = current_parts.clone();
                let mut not_applied_parts = current_parts.clone();

                applied_parts.ranges[split_range] = rule.value+1..current_end;
                not_applied_parts.ranges[split_range] = current_start..rule.value+1;

                (applied_parts, not_applied_parts)
            }
        },
        _ => panic!("eh?")
    }
}

fn evaluate_part_range(part: &Part, workflow_label: &String, workflows: &HashMap<String, Vec<Rule>>) -> Vec<Part> {
    //base cases
    if workflow_label == "A" {
        return vec![part.clone()];
    }
    if workflow_label == "R" {
        return vec![]
    }

    let mut accepted_part_ranges = Vec::new();
    let mut current_parts = part.clone();

    let workflow = workflows.get(workflow_label).unwrap();
    for rule in workflow {
        let applied_parts;
        (applied_parts, current_parts) = split_parts(&current_parts, rule);

        accepted_part_ranges.extend(evaluate_part_range(&applied_parts, &rule.dest, workflows));
    }
    accepted_part_ranges.into_iter().filter(|x| x.ranges.iter().all(|y| y.len() > 0)).collect()
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
        let matching_parts = evaluate_part_range(&part, &"in".to_owned(), &workflows);
        //println!("matching_parts: {:?}", matching_parts);
        if !matching_parts.is_empty() {
            sum1 += part.ranges.iter().map(|x|x.start).sum::<usize>();
        }
    }
    println!("q1: {:}", sum1);

    let all_parts = Part {ranges: [1..4001, 1..4001, 1..4001, 1..4001]};

    let matching_parts = evaluate_part_range(&all_parts, &"in".to_owned(), &workflows);
    //println!("matching_parts: {:?}",matching_parts);

    let sum2: usize = matching_parts.iter()
        .map(|x|{
            x.ranges.iter().map(|y|y.len()).product::<usize>()
        }).sum();
    println!("q2: {:}", sum2);

}
