use super::*;

pub fn check_rule(rule: &Rule, update: &Update) -> bool {
    let (a, b) = rule;
    if let Some(a_index) = update.iter().position(|val| val == a) {
        for (b_index, val) in update.iter().enumerate() {
            if val == b && b_index < a_index {
                return false;
            }
        }
    };

    true
}

pub fn check_update(rules: &Vec<Rule>, update: &Update) -> bool {
    for rule in rules {
        if !check_rule(rule, update) {
            return false;
        }
    }

    true
}

pub fn solve((rules, updates): Input) -> u32 {
    let mut ans = 0;
    for update in updates {
        println!("Checking update {update:?}");

        if check_update(&rules, &update) {
            println!("valid");
            ans += update.get(update.len() / 2).unwrap();
        }
    }

    ans
}
