use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct PageOrderingRules {
    rules: HashMap<usize, HashSet<usize>>,
}

impl PageOrderingRules {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, (page_before, page_after): (usize, usize)) {
        self.rules
            .entry(page_before)
            .or_default()
            .insert(page_after);
    }

    #[cfg(test)]
    pub fn get_rules_for_page(&self, page: usize) -> Option<&HashSet<usize>> {
        self.rules.get(&page)
    }

    pub fn pagelist_is_valid(&self, pagelist: &[usize]) -> bool {
        for i in 0..pagelist.len() {
            let page = pagelist[i];
            let previous_pages = &pagelist[..i];

            if let Some(page_rules) = self.rules.get(&page) {
                if previous_pages.iter().any(|p| page_rules.contains(p)) {
                    return false;
                }
            }
        }

        true
    }
}

impl FromIterator<(usize, usize)> for PageOrderingRules {
    fn from_iter<T: IntoIterator<Item = (usize, usize)>>(iter: T) -> Self {
        let mut rules = PageOrderingRules::new();
        for rule in iter {
            rules.add_rule(rule);
        }
        rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_no_related_rules_means_any_order_is_acceptable() {
        let ordering_rules = PageOrderingRules::from_iter(vec![(1, 2), (5, 6)]);
        let updates_pagelist = vec![
            vec![1, 2, 5, 6],
            vec![1, 5, 2, 6],
            vec![1, 5, 6, 2],
            vec![5, 1, 2, 6],
            vec![5, 1, 6, 2],
            vec![5, 6, 1, 2],
        ];

        for pagelist in updates_pagelist {
            assert!(
                ordering_rules.pagelist_is_valid(&pagelist),
                "testing pagelist {pagelist:?}"
            );
        }
    }

    #[test]
    fn test_pagelist_is_valid() {
        let ordering_rules = PageOrderingRules::from_iter(vec![(1, 2), (5, 6)]);
        let pagelist = vec![1, 2, 5, 6];

        assert!(ordering_rules.pagelist_is_valid(&pagelist));
    }

    #[test]
    fn test_pagelist_is_invalid() {
        let ordering_rules = PageOrderingRules::from_iter(vec![(1, 2), (5, 6)]);
        let pagelist = vec![2, 1, 6, 5];

        assert!(!ordering_rules.pagelist_is_valid(&pagelist));
    }
}
