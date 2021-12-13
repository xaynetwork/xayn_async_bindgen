macro_rules! __assert_trimmed_line_eq {
    ($left:expr, $right:expr) => {{
        let left = $left;
        let mut left = $crate::test_utils::trimmed_non_empty_lines(&left);
        let right = $right;
        let mut right = $crate::test_utils::trimmed_non_empty_lines(&right);
        for (left, right) in (&mut left).zip(&mut right) {
            assert_eq!(left, right);
        }
        assert!(left.next().is_none());
        assert!(right.next().is_none());
    }};
}

pub(crate) use __assert_trimmed_line_eq as assert_trimmed_line_eq;

pub(crate) fn trimmed_non_empty_lines(s: &str) -> impl Iterator<Item = &str> {
    s.lines().flat_map(|line| {
        let line = line.trim();
        (!line.is_empty()).then(|| line)
    })
}
