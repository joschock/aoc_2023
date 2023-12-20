use std::{fs::read_to_string, collections::{BTreeMap, VecDeque, BTreeSet}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Level {
    Low,
    High
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Pulse {
    level: Level,
    src: String,
    dst: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum WidgetType {
    FlipFlop,
    Conjunction,
    Broadcaster,
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Widget {
    widget: WidgetType,
    label: String,
    state: Level,
    inputs: BTreeMap<String, Level>,
    outputs: Vec<String>
}

impl Widget {
    fn process_input(&mut self, input: Pulse) -> Vec<Pulse> {
        match self.widget {
            WidgetType::FlipFlop => {
                if input.level == Level::High {
                    vec![]
                } else {
                    if self.state == Level::Low {
                        self.state = Level::High;
                    } else {
                        self.state = Level::Low;
                    }
                    self.outputs.iter().map(|x|{
                        Pulse {src: self.label.to_owned(), dst: x.clone(), level: self.state}
                    }).collect()
                }
            },
            WidgetType::Conjunction => {
                self.inputs.insert(input.src, input.level);
                if self.inputs.values().all(|x|*x == Level::High) {
                    self.outputs.iter().map(|x|{
                        Pulse {src: self.label.to_owned(), dst: x.clone(), level: Level::Low}
                    }).collect()
                } else {
                    self.outputs.iter().map(|x|{
                        Pulse {src: self.label.to_owned(), dst: x.clone(), level: Level::High}
                    }).collect()
                }
            },
            WidgetType::Broadcaster => {
                self.outputs.iter().map(|x|{
                    Pulse {src: self.label.to_owned(), dst: x.clone(), level: input.level}
                }).collect()
            },
            WidgetType::Output => {
                vec![]
            }
        }
    }
}


fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let mut widgets: BTreeMap<String, Widget> = BTreeMap::new();
    let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();

    for line in input.lines() {
        let mut split;
        let widget;
        if line.starts_with("%") {
            split = line[1..].split(" -> ");
            widget = WidgetType::FlipFlop;
        } else if line.starts_with("&") {
            split = line[1..].split(" -> ");
            widget = WidgetType::Conjunction;
        } else if line.starts_with("broadcaster") {
            split = line.split(" -> ");
            widget = WidgetType::Broadcaster;
        } else {
            panic!("eh?");
        }
        let label = split.next().unwrap().to_owned();
        let outputs: Vec<String> = split.next().unwrap().split(", ").map(|x|x.to_owned()).collect();
        widgets.insert(label.clone(), Widget {
            widget,
            label,
            state: Level::Low,
            inputs: BTreeMap::new(),
            outputs
        });
    }

    let labels: Vec<String> = widgets.keys().cloned().collect();

    for label in labels {
        let outputs = widgets.get(&label).unwrap().outputs.clone();
        for output in outputs {
            if let Some(target) = widgets.get_mut(&output) {
                target.inputs.insert(label.clone(), Level::Low);
            } else {
                let mut output_widget = Widget {
                    label: output.clone(),
                    widget: WidgetType::Output,
                    inputs: BTreeMap::new(),
                    state: Level::Low,
                    outputs: Vec::new()
                };
                output_widget.inputs.insert(label.clone(), Level::Low);
                widgets.insert(output.clone(), output_widget);
            }
        }
    }

    //println!("widgets: {:#?}", widgets);

    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    let mut xn_period = 0..0;
    let mut qn_period = 0..0;
    let mut xf_period = 0..0;
    let mut zl_period = 0..0;

    'outer: for press in 0.. {
        pulse_queue.push_back(Pulse {src: "button".to_owned(), dst: "broadcaster".to_owned(), level: Level::Low});
        if press < 1000 {
            low_pulse_count += 1;
        }
        while !pulse_queue.is_empty() {
            let pulse = pulse_queue.pop_front().unwrap();
            if pulse.level == Level::Low {
                match pulse.dst.as_str() {
                    "xn" => {
                        if xn_period.start == 0 {
                            xn_period.start = press;
                        } else if xn_period.end == 0 {
                            xn_period.end = press;
                        }
                    },
                    "qn" => {
                        if qn_period.start == 0 {
                            qn_period.start = press;
                        } else if qn_period.end == 0 {
                            qn_period.end = press;
                        }
                    },
                    "xf" => {
                        if xf_period.start == 0 {
                            xf_period.start = press;
                        } else if xf_period.end == 0 {
                            xf_period.end = press;
                        }
                    },
                    "zl" => {
                        if zl_period.start == 0 {
                            zl_period.start = press;
                        } else if zl_period.end == 0 {
                            zl_period.end = press;
                        }
                    },
                    _ => {}
                }
                if xn_period.end != 0 && qn_period.end != 0 && xf_period.end != 0 && zl_period.end != 0 {
                    break 'outer;
                }
            }
            //println!("processing pulse: {:?}", pulse);
            let target = widgets.get_mut(&pulse.dst).unwrap();
            let new_pulses = target.process_input(pulse);
            for pulse in new_pulses {
                if press < 1000 {
                    if pulse.level == Level::Low {
                        low_pulse_count += 1;
                    } else {
                        high_pulse_count +=1;
                    }
                }
                pulse_queue.push_back(pulse);
            }
        }
    }
    println!("q1 pulse count: {:} * {:} = {:}", low_pulse_count, high_pulse_count, low_pulse_count * high_pulse_count);


    println!("periods: {:?}, {:?}, {:?}, {:?}", xn_period, qn_period, xf_period, zl_period);

    let lcm = [xn_period.len(), qn_period.len(), xf_period.len(), zl_period.len()].iter().fold(0, |acc, element| {
        if acc == 0 {
            *element
        } else {
            lcm(acc, *element)
        }
    });

    println!("q2: {:?}", lcm);

    // for button_press in 1.. {
    //     pulse_queue.push_back(Pulse {src: "button".to_owned(), dst: "broadcaster".to_owned(), level: Level::Low});
    //     low_pulse_count += 1;
    //     while !pulse_queue.is_empty() {
    //         let pulse = pulse_queue.pop_front().unwrap();
    //         //println!("processing pulse: {:?}", pulse);
    //         let target = widgets.get_mut(&pulse.dst).unwrap();
    //         let new_pulses = target.process_input(pulse);
    //         for pulse in new_pulses {
    //             if pulse.dst == "rx" && pulse.level == Level::Low {
    //                 println!("q2 button presses: {:}", button_press);
    //                 break;
    //             }
    //             pulse_queue.push_back(pulse);
    //         }
    //     }
    // }

}

fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = if a > b { (a, b) } else {(b,a)};

    loop {
        let res = a % b;
        if res == 0 {
            return b;
        }
        a = b;
        b = res;
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd (a, b)
}