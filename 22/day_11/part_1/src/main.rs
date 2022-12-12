#![allow(non_snake_case)]

// --- STD Imports ---
use regex::Regex;


struct Monkey
{
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: i64,
    throwTrue: usize,
    throwFalse: usize,
    inspectCounter: usize
} // struct Monkey


impl Monkey
{
    fn addItem(&mut self, item: i64)
    {
        self.items.push(item);
    }

    fn monkeyBusiness(&mut self) -> Vec<(usize,i64)>
    {
        let mut ops: Vec<(usize,i64)> = Default::default();
        let numberOfItems = self.items.len();
        self.inspectCounter += numberOfItems;
        for item in self.items.iter().copied() {
                print!("inspect {}", item);
                let value = self.operation.as_ref()(item) / 3;
                print!(" => {}", value);
                ops.push((if value % self.test == 0 {self.throwTrue} else {self.throwFalse}, value));
                println!(" => throw to {}", if value % self.test == 0 {self.throwTrue} else {self.throwFalse});
        }
        self.items.clear();
        return ops;
    }

    fn getInspectCounter(&self) -> &usize
    {
        return &self.inspectCounter;
    }
} // impl Monkey


fn main()
{
    let mut monkeys: Vec<Monkey> = Default::default();

    if let Ok(monkeyPattern) = Regex::new(r"Monkey [0-9]:\n  Starting items: (.*)\n  Operation: new = old (\+|\*) (.+)\n  Test: divisible by ([0-9]+)\n    If true: throw to monkey ([0-9]+)\n    If false: throw to monkey ([0-9]+)") {
        if let Ok(itemPattern) = Regex::new("[0-9]+") {
            if let Ok(file) = std::fs::File::open("input") {
                if let Ok(contents) = std::io::read_to_string(&file) {
                    for maybeMonkey in monkeyPattern.find_iter(&contents) {
                        if let Some(captures) = monkeyPattern.captures(maybeMonkey.as_str()) {

                            let mut operation: Box<dyn Fn(i64) -> i64> = Box::new(|i| {0});
                            let mut operand: Option<i64> = Default::default(); // <== if None => old
                            if let Some(operationMatch) = captures.get(3) {
                                if let Ok(op) = operationMatch.as_str().parse::<i64>() {
                                    operand.insert(op);
                                } // if operand
                            } // if operationMatch
                            if let Some(opMatch) = captures.get(2) {
                                if opMatch.as_str() == "+"{
                                    if let Some(opr) = operand {
                                        operation = Box::new(move |i| i + opr);
                                    } else {
                                        operation = Box::new(|i| i + i);
                                    } // operand == "old"
                                } else if opMatch.as_str() == "*" {
                                    if let Some(opr) = operand {
                                        operation = Box::new(move |i| i * opr);
                                    } else {
                                        operation = Box::new(|i| i * i);
                                    } // operand == "old"
                                } else {
                                    println!("Invalid operation: {}", opMatch.as_str());
                                    break;
                                }
                            } // if opMatch

                            let mut test: i64 = 0;
                            if let Some(testMatch) = captures.get(4) {
                                if let Ok(testValue) = testMatch.as_str().parse::<i64>() {
                                    test = testValue;
                                } else {
                                    println!("Failed to parse test: {}", testMatch.as_str());
                                    break;
                                }
                            } // if testMatch

                            let mut throwTrue: usize = 0;
                            if let Some(throwTrueMatch) = captures.get(5) {
                                if let Ok(throwTrueValue) = throwTrueMatch.as_str().parse::<usize>() {
                                    throwTrue = throwTrueValue;
                                } else {
                                    println!("Failed to parse true branch: {}", throwTrueMatch.as_str());
                                    break;
                                }
                            } // if Some(throwTrueMatch)

                            let mut throwFalse: usize = 0;
                            if let Some(throwFalseMatch) = captures.get(6) {
                                if let Ok(throwFalseValue) = throwFalseMatch.as_str().parse::<usize>() {
                                    throwFalse = throwFalseValue;
                                } else {
                                    println!("Failed to parse false branch: {}", throwFalseMatch.as_str());
                                    break;
                                }
                            } // if Some(throwFalseMatch)

                            let mut monkey = Monkey {
                                items: Default::default(),
                                operation: operation,
                                test: test,
                                throwTrue: throwTrue,
                                throwFalse: throwFalse,
                                inspectCounter: 0
                            };

                            if let Some(itemMatch) = captures.get(1) {
                                for maybeItem in itemPattern.find_iter(itemMatch.as_str()) {
                                    if let Ok(item) = maybeItem.as_str().parse::<i64>() {
                                        monkey.addItem(item);
                                    } // if item
                                } // for maybeItem in captures.find_iter
                            } // if itemMatch

                            monkeys.push(monkey);

                        } else {
                            println!("Couldn't find captures in monkey: {}", maybeMonkey.as_str());
                        } // not maybeMonkey
                    } // for maybeMonkey in monkeyPattern.captures
                } // contenst = file.read()
            } // if file
        } // if Ok(itemValuePattern)
    } // if Ok(monkeyPattern)

    for _ in 0..20 {
        for i_monkey in 0..monkeys.len() {
            let ops = monkeys[i_monkey].monkeyBusiness();
            for (j_monkey, value) in ops {
                monkeys[j_monkey].addItem(value);
            } // for monkey in monkeys
        } // for monkey in monkeys
    } // for round in range(20)

    monkeys.sort_by(|left, right| left.getInspectCounter().cmp(right.getInspectCounter()));
    for monkey in monkeys.iter() {
        println!("{}", monkey.getInspectCounter());
    }
    println!("{}", monkeys.iter().map(|m| m.getInspectCounter()).rev().take(2).product::<usize>());
}
