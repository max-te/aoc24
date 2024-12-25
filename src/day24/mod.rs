use std::{cell::RefCell, ops::Deref};

use aoc_runner_derive::aoc;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

type WireName = String;
#[derive(Debug, Clone, PartialEq, Eq)]
enum WireValue {
    Determined(bool),
    GateAnd(WireName, WireName),
    GateOr(WireName, WireName),
    GateXor(WireName, WireName),
}

fn parse(input: &str) -> (FxHashMap<WireName, RefCell<WireValue>>, i8) {
    let mut wires = FxHashMap::default();
    let mut lines = input.lines();
    let mut z_index: i8 = -1;
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let (wire, value) = line.split_once(": ").unwrap();
        if wire.starts_with("z") {
            z_index += 1;
        }
        wires.insert(
            wire.to_string(),
            RefCell::new(WireValue::Determined(value == "1")),
        );
    }
    for line in lines {
        let (rule, wire) = line.split_once(" -> ").unwrap();
        let (a, operand, b) = rule.split(' ').collect_tuple().unwrap();
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        if wire.starts_with("z") {
            z_index += 1;
        }
        wires.insert(
            wire.to_string(),
            RefCell::new(match operand {
                "AND" => WireValue::GateAnd(a.to_string(), b.to_string()),
                "OR" => WireValue::GateOr(a.to_string(), b.to_string()),
                "XOR" => WireValue::GateXor(a.to_string(), b.to_string()),
                _ => unreachable!(),
            }),
        );
    }

    (wires, z_index)
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> usize {
    let (mut wires, mut z_index) = parse(input);

    let mut res: usize = 0;
    while z_index >= 0 {
        let z_wire = format!("z{:02}", z_index);
        let z = determine_wire_value(&wires, z_wire).unwrap();
        res = (res << 1) | if z { 1 } else { 0 };
        z_index -= 1;
    }

    res
}

fn determine_wire_value(
    wires: &FxHashMap<WireName, RefCell<WireValue>>,
    wire_name: impl AsRef<str>,
) -> Result<bool, WireName> {
    let wire_name = wire_name.as_ref();
    let mut cell = wires
        .get(wire_name)
        .ok_or_else(|| wire_name.to_owned())?
        .try_borrow_mut()
        .or_else(|_| Err(wire_name.to_owned()))?;
    match cell.deref() {
        WireValue::Determined(value) => Ok(*value),
        WireValue::GateAnd(a, b) => {
            let a_val = determine_wire_value(wires, a)?;
            let b_val = determine_wire_value(wires, b)?;
            let val = a_val && b_val;
            *cell = WireValue::Determined(val);
            Ok(val)
        }
        WireValue::GateOr(a, b) => {
            let a_val = determine_wire_value(wires, a)?;
            let b_val = determine_wire_value(wires, b)?;
            let val = a_val || b_val;
            *cell = WireValue::Determined(val);
            Ok(val)
        }
        WireValue::GateXor(a, b) => {
            let a_val = determine_wire_value(wires, a)?;
            let b_val = determine_wire_value(wires, b)?;
            let val = a_val ^ b_val;
            *cell = WireValue::Determined(val);
            Ok(val)
        }
    }
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> String {
    let (wires, z_max) = parse(input);
    // let mut expected_inputs = FxHashSet::default();
    let mut wrong_gates = FxHashSet::default();
    for z in 0..z_max {
        let z_wire = format!("z{:02}", z);
        // expected_inputs.insert(format!("x{:02}", z));
        // expected_inputs.insert(format!("y{:02}", z));
        // let inputs = controlling_inputs(&wires, &z_wire);
        // if inputs != expected_inputs {
        //     eprintln!("Expected {expected_inputs:?} for {z_wire}, got {inputs:?}");
        // }

        let z_gate = wires.get(&z_wire).unwrap().borrow();
        if !matches!(z_gate.deref(), WireValue::GateXor(_, _)) {
            eprintln!("Expected xor for {z_wire}, got {:?}", z_gate.deref());
            wrong_gates.insert(z_wire);
        }
    }
    let z_max_wire = format!("z{:02}", z_max);
    dbg!(&z_max_wire);
    {
        let z_max_gate = wires.get(&z_max_wire).unwrap().borrow();
        if !matches!(z_max_gate.deref(), WireValue::GateOr(_, _)) {
            eprintln!(
                "Exptected carry {z_max_wire} to be an or gate, was {:?}",
                z_max_gate.deref()
            );
            wrong_gates.insert(z_max_wire.clone());
        }
    }
    for (wire, gate) in wires.iter() {
        let gate = gate.borrow();
        match gate.deref() {
            WireValue::Determined(_) => {
                if !wire.starts_with("x") && !wire.starts_with("y") {
                    eprintln!("Did not expect {wire} to be pre-determined.");
                    wrong_gates.insert(wire.clone());
                }
            }
            WireValue::GateAnd(a, b) => {
                if a != "x00" {
                    if !wires.values().any(|v| {
                        let v = v.borrow();
                        matches!(v.deref(), WireValue::GateOr(a2, b2) if a2 == wire || b2 == wire)
                    }) {
                        eprintln!("Expected {gate:?} at {wire} to feed into or gate.");
                        wrong_gates.insert(wire.clone());
                    }
                }
                if !a.starts_with("x") {
                    let a_gate = wires.get(a).unwrap().borrow();
                    let b_gate = wires.get(b).unwrap().borrow();
                    if !(matches!(
                        a_gate.deref(),
                        WireValue::GateXor(_, _) | WireValue::GateOr(_, _)
                    ) || a_gate.deref()
                        == &WireValue::GateAnd(String::from("x00"), String::from("y00")))
                    {
                        eprintln!("Expected {gate:?} at {wire} input {a} to be a xor or or gate, was {:?}.", a_gate.deref());
                        wrong_gates.insert(a.clone());
                    }
                    if !(matches!(
                        b_gate.deref(),
                        WireValue::GateXor(_, _) | WireValue::GateOr(_, _)
                    ) || b_gate.deref()
                        == &WireValue::GateAnd(String::from("x00"), String::from("y00")))
                    {
                        eprintln!("Expected {gate:?} at {wire} input {b} to be a xor or or gate, was {:?}.", b_gate.deref());
                        wrong_gates.insert(b.clone());
                    }
                }
            }
            WireValue::GateOr(a, b) => {
                if wire != &z_max_wire {
                    if !wires.values().any(|v| {
                        let v = v.borrow();
                        matches!(v.deref(), WireValue::GateAnd(a2, b2) if a2 == wire || b2 == wire)
                    }) {
                        eprintln!("Expected {gate:?} at {wire} to feed into last z or and gate.");
                        wrong_gates.insert(wire.clone());
                    }
                    if !wires.values().any(|v| {
                        let v = v.borrow();
                        matches!(v.deref(), WireValue::GateXor(a2, b2) if a2 == wire || b2 == wire)
                    }) {
                        eprintln!("Expected {gate:?} at {wire} to feed into last z or xor gate.");
                        wrong_gates.insert(wire.clone());
                    }
                }
                if a != "x00" {
                    let a_gate = wires.get(a).unwrap().borrow();
                    let b_gate = wires.get(b).unwrap().borrow();
                    if !matches!(a_gate.deref(), WireValue::GateAnd(_, _)) {
                        eprintln!(
                            "Expected {gate:?} at {wire} input {a} to be an and gate, was {:?}.",
                            a_gate.deref()
                        );
                        wrong_gates.insert(a.clone());
                    }
                    if !matches!(b_gate.deref(), WireValue::GateAnd(_, _)) {
                        eprintln!(
                            "Expected {gate:?} at {wire} input {b} to be an and gate, was {:?}.",
                            b_gate.deref()
                        );
                        wrong_gates.insert(b.clone());
                    }
                }
            }
            WireValue::GateXor(a, b) => {
                if !wire.starts_with("z") {
                    if !wires.values().any(|v| {
                        let v = v.borrow();
                        matches!(v.deref(), WireValue::GateAnd(a2, b2) if a2 == wire || b2 == wire)
                    }) {
                        eprintln!("Expected {gate:?} at {wire} to feed into z-wire or and gate.");
                        wrong_gates.insert(wire.clone());
                    }
                    if !wires.values().any(|v| {
                        let v = v.borrow();
                        matches!(v.deref(), WireValue::GateXor(a2, b2) if a2 == wire || b2 == wire)
                    }) {
                        eprintln!("Expected {gate:?} at {wire} to feed into z-wire or xor gate.");
                        wrong_gates.insert(wire.clone());
                    }
                    if !a.starts_with("x") {
                        let a_gate = wires.get(a).unwrap().borrow();
                        let b_gate = wires.get(b).unwrap().borrow();

                        match a_gate.deref() {
                            WireValue::GateOr(a2, _) | WireValue::GateXor(a2, _) => {
                                if wire.starts_with("z") {
                                    if a2.starts_with("x") {
                                        eprintln!("Expected {gate:?} at {wire} input {a} to be a 2nd order gate, was {a_gate:?}");
                                        wrong_gates.insert(wire.clone());
                                    }
                                } else {
                                    if !a2.starts_with("x") {
                                        eprintln!("Expected {gate:?} at {wire} input {a} to be a 1st order gate, was {a_gate:?}");
                                        wrong_gates.insert(wire.clone());
                                    }
                                }
                            }
                            _ => {}
                        }

                        if !(matches!(
                            a_gate.deref(),
                            WireValue::GateXor(_, _) | WireValue::GateOr(_, _)
                        ) || a_gate.deref()
                            == &WireValue::GateAnd(String::from("x00"), String::from("y00")))
                        {
                            eprintln!("Expected {gate:?} at {wire} input {a} to be a xor or or gate, was {:?}.", a_gate.deref());
                            wrong_gates.insert(a.clone());
                        }

                        if !(matches!(
                            b_gate.deref(),
                            WireValue::GateXor(_, _) | WireValue::GateOr(_, _)
                        ) || b_gate.deref()
                            == &WireValue::GateAnd(String::from("x00"), String::from("y00")))
                        {
                            eprintln!("Expected {gate:?} at {wire} input {b} to be a xor or or gate, was {:?}.", b_gate.deref());
                            wrong_gates.insert(b.clone());
                        }
                    }
                } else if wire != "z00" {
                    if a.starts_with("x") || b.starts_with("y") {
                        eprintln!("Did not expect {gate:?} at {wire} to feed into z-wire.");
                        wrong_gates.insert(wire.clone());
                    }
                }
            }
        }
    }

    wrong_gates.iter().sorted().join(",")
}

fn controlling_inputs(
    wires: &FxHashMap<WireName, RefCell<WireValue>>,
    wire_name: impl AsRef<str>,
) -> FxHashSet<WireName> {
    let wire_name = wire_name.as_ref();
    match wires.get(wire_name).unwrap().borrow().deref() {
        WireValue::GateAnd(a, b) | WireValue::GateOr(a, b) | WireValue::GateXor(a, b) => {
            let a_inputs = controlling_inputs(wires, a);
            let b_inputs = controlling_inputs(wires, b);
            let mut inputs = a_inputs;
            inputs.extend(b_inputs);
            inputs
        }
        WireValue::Determined(_) => FxHashSet::from_iter([wire_name.to_string()]),
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn test_part1_small() {
        let input = include_str!("test-small.txt");
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1(input), 2024);
    }
}
