use super::*;
use super::a::is_safe;

pub fn solve(reports: Vec<Report>) -> usize {
    reports
        .into_iter()
        .filter(|report| {
            if is_safe(report) {
                true
            } else {
                let mut i = 0;
                while i < report.len() {
                    let mut buf = report.clone();
                    buf.remove(i);
                    let safe = is_safe(&buf);
                    if safe {
                        return true;
                    }
                    i += 1;
                }
                false
            }
        })
        .count()
}
