use a::check_update;

use super::*;

fn apply_rule(rule: &Rule, update: &Update) -> Update {
    let mut buf = update.clone();

    let (a, b) = rule;
    if let Some(a_index) = update.iter().position(|val| val == a) {
        let mut b_index = 0;
        while b_index < buf.len() {
            if buf[b_index] == *b && b_index < a_index {
                buf[a_index] = *b;
                buf[b_index] = *a;
            }

            b_index += 1;
        }
    };

    buf
}

pub fn solve((rules, updates): Input) -> u32 {
    let invalid = updates
        .into_iter()
        .filter(|u| !check_update(&rules, u))
        .collect_vec();

    let mut ans = 0;

    for update in invalid {
        let mut buf = update.clone();

        while !check_update(&rules, &buf) {
            for rule in rules.iter() {
                buf = apply_rule(rule, &buf)
            }
        }

        let middle = buf.get(buf.len() / 2).unwrap();
        ans += middle;
    }

    ans
}
