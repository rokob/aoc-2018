extern crate utils;
#[allow(unused_imports)]
use utils::{read_file, split_ws, HashMap, HashSet};
use std::cmp::Reverse;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Type {
    Cold,
    Radiation,
    Slashing,
    Fire,
    Bludge,
}
use Type::*;

impl Type {
    fn from(s: &str) -> Self {
        match s {
            "cold" => Cold,
            "radiation" => Radiation,
            "slashing" => Slashing,
            "fire" => Fire,
            "bludgeoning" => Bludge,
            _ => panic!("bad input"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Army {
    Immune,
    Infection,
}
use Army::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Group {
    id: usize,
    units: usize,
    hp: usize,
    attack: usize,
    initiative: usize,
    damage_type: Type,
    immune: Vec<Type>,
    weak: Vec<Type>,
    army: Army,
}

use std::fmt;
impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} units each with {} hit points", self.units, self.hp)?;
        if !self.immune.is_empty() || !self.weak.is_empty() {
            write!(f, "(")?;
        }
        if !self.immune.is_empty() {
            write!(f, "immune to ")?;
            if self.immune.len() == 1 {
                write!(f, "{:?}", self.immune[0])?;
            } else {
                write!(f, "{:?}", self.immune[0])?;
                let mut immunes = self.immune.iter();
                immunes.next();
                for i in immunes {
                    write!(f, ", {:?}", i)?;
                }
            }
        }
        if !self.immune.is_empty() && !self.weak.is_empty() {
            write!(f, "; weak to ")?;
        } else if !self.weak.is_empty() {
            write!(f, "weak to ")?;
        }
        if !self.weak.is_empty() {
            if self.weak.len() == 1 {
                write!(f, "{:?}", self.weak[0])?;
            } else {
                write!(f, "{:?}", self.weak[0])?;
                let mut weaks = self.weak.iter();
                weaks.next();
                for i in weaks {
                    write!(f, ", {:?}", i)?;
                }
            }
        }
        if !self.immune.is_empty() || !self.weak.is_empty() {
            write!(f, ")")?;
        }
        write!(f, " with an attack that does {} {:?} damage at initiative {}",
               self.attack, self.damage_type, self.initiative)
    }
}

impl Group {
    fn power(&self) -> usize {
        self.units * self.attack
    }

    fn select_target(&self, others: &Vec<Group>, seen: &mut HashSet<usize>) -> Option<usize> {
        let mut targets = others
            .iter()
            .filter(|o| !seen.contains(&o.id) && o.units > 0)
            .map(|o| (self.damage(o), o.power(), o.initiative, o.id)).collect::<Vec<_>>();
        targets.sort();
        targets.reverse();
        if targets.len() == 0 {
            return None;
        }
        let first = targets.first().unwrap();
        if first.0 == 0 {
            None
        } else {
            seen.insert(first.3);
            Some(first.3)
        }
    }

    fn damage(&self, other: &Group) -> usize {
        if other.immune.contains(&self.damage_type) {
            return 0;
        }
        let mult = if other.weak.contains(&self.damage_type) {
            2
        } else {
            1
        };
        mult * self.power()
    }
}

fn process(line: &str, army: Army, id: usize) -> Group {
    let parts = line.split(" units each with ").collect::<Vec<_>>();
    let units = parts[0].parse::<usize>().unwrap();
    let parts = parts[1].split(" hit points ").collect::<Vec<_>>();
    let hp = parts[0].parse::<usize>().unwrap();

    let (immune, weak, parts) = if parts[1].starts_with("with an") {
        (vec![], vec![], &parts[1][25..])
    } else {
        let parts = parts[1].split(" with an attack that does ").collect::<Vec<_>>();
        let (immune, weak) = handle_modifier(parts[0]);
        (immune, weak, parts[1])
    };
    let parts = parts.split(" damage at initiative ").collect::<Vec<_>>();
    let (attack, damage_type) = handle_attack(parts[0]);
    let initiative = parts[1].parse::<usize>().unwrap();

    Group {
        id,
        units,
        hp,
        attack,
        initiative,
        damage_type,
        immune,
        weak,
        army,
    }
}

fn handle_modifier(s: &str) -> (Vec<Type>, Vec<Type>) {
    if s.is_empty() {
        return (vec![], vec![]);
    }
    let s = s.trim_start_matches("(").trim_end_matches(")");
    let parts = s.split(";").collect::<Vec<_>>();
    if parts.len() == 1 {
        if parts[0].trim().starts_with("weak to") {
            let mods = parts[0].trim()[8..].split(", ").map(|p| Type::from(p)).collect::<Vec<Type>>();
            return (vec![], mods);
        } else {
            let mods = parts[0].trim()[10..].split(", ").map(|p| Type::from(p)).collect::<Vec<Type>>();
            return (mods, vec![]);
        }
    } else {
        if parts[0].trim().starts_with("weak to") {
            let weak = parts[0].trim()[8..].split(", ").map(|p| Type::from(p)).collect::<Vec<Type>>();
            let immune = parts[1].trim()[10..].split(", ").map(|p| Type::from(p)).collect::<Vec<Type>>();
            return (immune, weak);
        } else {
            let weak = parts[1].trim()[8..].split(", ").map(|p| Type::from(p)).collect::<Vec<Type>>();
            let immune = parts[0].trim()[10..].split(", ").map(|p| Type::from(p)).collect::<Vec<Type>>();
            return (immune, weak);
        }
    }
}

fn handle_attack(s: &str) -> (usize, Type) {
    let parts = s.split(" ").collect::<Vec<_>>();
    let attack = parts[0].parse::<usize>().unwrap();
    let damage_type = Type::from(parts[1]);
    (attack, damage_type)
}

fn main() {
    let mut reading_immune = false;
    let mut reading_infection = false;
    let mut immune: Vec<Group> = Vec::new();
    let mut infection: Vec<Group> = Vec::new();
    for line in include_str!("../input.txt").lines() {
        if line.starts_with("Immune System:") {
            reading_immune = true;
            reading_infection = false;
            continue;
        }

        if line.starts_with("Infection:") {
            reading_infection = true;
            reading_immune = false;
            continue;
        }

        if reading_immune {
            if line.is_empty() {
                reading_immune = false;
                continue;
            }

            let val = process(line, Immune, immune.len());
            immune.push(val);
        }

        if reading_infection {
            let val = process(line, Infection, infection.len());
            infection.push(val);
        }
    }

    let result: usize;
    loop {
        immune.sort_by_key(|g| (g.units == 0, Reverse(g.power()), Reverse(g.initiative)));
        infection.sort_by_key(|g| (g.units == 0, Reverse(g.power()), Reverse(g.initiative)));

        let mut immune_targets = HashMap::new();
        let mut immune_selected = HashSet::new();
        for g in immune.iter() {
            if g.units == 0 {
                immune_targets.insert(g.id, None);
                continue;
            }
            let target = g.select_target(&infection, &mut immune_selected);
            immune_targets.insert(g.id, target);
        }
        let mut infection_targets = HashMap::new();
        let mut infection_selected = HashSet::new();
        for g in infection.iter() {
            if g.units == 0 {
                infection_targets.insert(g.id, None);
                continue;
            }
            let target = g.select_target(&immune, &mut infection_selected);
            infection_targets.insert(g.id, target);
        }

        immune.sort_by_key(|g| (g.units == 0, Reverse(g.initiative)));
        infection.sort_by_key(|g| (g.units == 0, Reverse(g.initiative)));

        let mut a = 0;
        let mut b = 0;
        'attack: loop {
            if a >= immune.len() && b >= infection.len() { break; }
            if a >= immune.len() {
                while b < infection.len() {
                    let next_infect = &infection[b];
                    if next_infect.units == 0 { b += 1; continue; }
                    if let Some(target_id) = infection_targets.get(&next_infect.id) {
                        if let Some(target) = target_id {
                            for group in immune.iter_mut() {
                                if group.id == *target {
                                    attack(&next_infect, group);
                                    break;
                                }
                            }
                        }
                    }
                    b += 1;
                }
                break;
            }
            if b >= infection.len() {
                while a < immune.len() {
                    let next_immune = &immune[a];
                    if next_immune.units == 0 { a += 1 ;continue; }
                    if let Some(target_id) = immune_targets.get(&next_immune.id) {
                        if let Some(target) = target_id {
                            for group in infection.iter_mut() {
                                if group.id == *target {
                                    attack(&next_immune, group);
                                    break;
                                }
                            }
                        }
                    }
                    a += 1;
                }
                break;
            }

            let a_turn: bool;
            {
                let next_immune = &immune[a];
                let next_infect = &infection[b];

                if next_immune.units == 0 { a += 1; continue; }
                if next_infect.units == 0 { b += 1; continue; }

                a_turn = next_immune.initiative > next_infect.initiative;
            }

            if a_turn {
                let next_immune = &immune[a];
                if let Some(target_id) = immune_targets.get(&next_immune.id) {
                    if let Some(target) = target_id {
                        for group in infection.iter_mut() {
                            if group.id == *target {
                                attack(&next_immune, group);
                                break;
                            }
                        }
                    }
                }
                a += 1;
            } else {
                let next_infect = &infection[b];
                if let Some(target_id) = infection_targets.get(&next_infect.id) {
                    if let Some(target) = target_id {
                        for group in immune.iter_mut() {
                            if group.id == *target {
                                attack(&next_infect, group);
                                break;
                            }
                        }
                    }
                }
                b += 1;
            }
        }

        let immune_left = immune.iter().filter(|g| g.units > 0).count() > 0;
        if !immune_left {
            result = infection.iter().map(|g| g.units).sum();
            break;
        }
        let infect_left = infection.iter().filter(|g| g.units > 0).count() > 0;
        if !infect_left {
            result = immune.iter().map(|g| g.units).sum();
            break;
        }
    }
    println!("{}", result);
}

fn attack(attacker: &Group, defender: &mut Group) {
    let damage = attacker.damage(defender);
    let units_killed = damage / defender.hp;
    if units_killed >= defender.units {
        defender.units = 0;
    } else {
        defender.units -= units_killed;
    }
}
