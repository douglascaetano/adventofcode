mod page_ordering_rules;

use std::str::FromStr;

pub use self::page_ordering_rules::PageOrderingRules;

#[derive(Debug)]
pub struct InputData {
    ordering_rules: PageOrderingRules,
    updates_pagelist: Vec<Vec<usize>>,
}

impl InputData {
    pub fn get_correct_updates(&self) -> Vec<&[usize]> {
        self.updates_pagelist
            .iter()
            .filter(|pagelist| self.ordering_rules.pagelist_is_valid(pagelist))
            .map(|pagelist| pagelist.as_slice())
            .collect()
    }

    pub fn fix_incorrect_updates(&self) -> Vec<Vec<usize>> {
        self.updates_pagelist
            .iter()
            .filter(|pagelist| !self.ordering_rules.pagelist_is_valid(pagelist))
            .map(|pagelist| self.ordering_rules.fix_pagelist(pagelist))
            .collect()
    }
}

impl FromStr for InputData {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let ordering_rules = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| -> Result<_, Self::Err> {
                let mut parts = line.split('|');
                let page_a = parts.next().ok_or("invalid input")?.parse()?;
                let page_b = parts.next().ok_or("invalid input")?.parse()?;

                parts
                    .next()
                    .is_none()
                    .then_some((page_a, page_b))
                    .ok_or("invalid input".into())
            })
            .collect::<Result<PageOrderingRules, _>>()?;

        let updates_pagelist = lines
            .map(|line| {
                line.split(',')
                    .map(|part| part.parse())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(InputData {
            ordering_rules,
            updates_pagelist,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_read_input() {
        let input = "1|2\n\
             5|6\n\
             \n\
             1,3,4,5\n\
             2,3,4,5";
        let data = InputData::from_str(input).unwrap();

        assert!(data
            .ordering_rules
            .get_rules_for_page(1)
            .is_some_and(|rules| rules.difference(&HashSet::from([2])).count() == 0));
        assert!(data
            .ordering_rules
            .get_rules_for_page(5)
            .is_some_and(|rules| rules.difference(&HashSet::from([6])).count() == 0));
        assert_eq!(
            data.updates_pagelist,
            vec![vec![1, 3, 4, 5], vec![2, 3, 4, 5]]
        );
    }

    #[test]
    fn test_get_correct_updates() {
        let data = InputData {
            ordering_rules: PageOrderingRules::from_iter(vec![(1, 2), (5, 6)]),
            updates_pagelist: vec![vec![1, 2, 5, 6], vec![2, 1, 6, 5]],
        };

        assert_eq!(data.get_correct_updates(), vec![&vec![1, 2, 5, 6]]);
    }

    #[test]
    fn test_fix_incorrect_updates() {
        let data = InputData {
            ordering_rules: PageOrderingRules::from_iter(vec![(1, 2), (5, 6)]),
            updates_pagelist: vec![vec![1, 2, 5, 6], vec![2, 6, 5]],
        };

        assert_eq!(data.fix_incorrect_updates(), vec![vec![2, 5, 6]]);
    }
}
