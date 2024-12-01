use super::*;

pub(super) fn is_safe(report: &Report) -> bool {
    let mut last_ordering = None;

    for (i, level) in report.iter().enumerate() {
        match report.get(i + 1) {
            None => return true,
            Some(next) => {
                let diff = level.abs_diff(*next);
                let ordering = level.cmp(next);

                if last_ordering.is_none() {
                    last_ordering = Some(ordering);
                }

                if !(1..4).contains(&diff) || last_ordering != Some(ordering) {
                    return false;
                }
            }
        }
    }

    true
}

pub fn solve(reports: Vec<Report>) -> usize {
    reports.into_iter().filter(is_safe).count()
}
