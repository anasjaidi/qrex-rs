// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   mod.rs                                             :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: ajaidi <ajaidi@student.42.fr>              +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2024/06/04 23:44:30 by ajaidi            #+#    #+#             //
//   Updated: 2024/06/06 19:50:45 by ajaidi           ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

#![allow(unused)]

#[derive(Debug, Clone)]
pub enum Condition {
    Native(String),
    Null(String),
    NotNull(String),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    In(String, Vec<String>),
    NotIn(String, Vec<String>),
    Eq(String, String),
    Neq(String, String),
    Between(String, String, String),
    Gt(String, String),
    Gte(String, String),
    Lt(String, String),
    Lte(String, String),
    Like(String, String),
}

impl Condition {
    pub fn build_conditions(&self) -> Option<String> {
        fn gc(c: &Condition) -> String {
            match c {
                Condition::Or(lhs, rhs) => format!("({} OR {})", gc(lhs), gc(rhs)),
                Condition::And(lhs, rhs) => format!("({} AND {})", gc(lhs), gc(rhs)),
                Condition::Null(f) => format!("{} IS NULL", f),
                Condition::NotNull(f) => format!("{} IS NOT NULL", f),
                Condition::In(f, d) => {
                    let values = d
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{} IN ({})", f, values)
                }
                Condition::NotIn(f, d) => {
                    let values = d
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{} NOT IN ({})", f, values)
                }
                Condition::Eq(f, d) => format!("{} = {}", f, d),
                Condition::Neq(f, d) => format!("{} != {}", f, d),
                Condition::Lt(f, d) => format!("{} < {}", f, d),
                Condition::Lte(f, d) => format!("{} <= {}", f, d),
                Condition::Gt(f, d) => format!("{} > {}", f, d),
                Condition::Gte(f, d) => format!("{} >= {}", f, d),
                Condition::Like(f, d) => format!("{} Like '{}'", f, d),
                Condition::Between(f, a, b) => format!("{} BETWEEN {} And {}", f, a, b),
                Condition::Native(f) => f.clone(),
            }
        }
        Some(gc(self))
    }

    pub fn or(mut self, condition: Self) -> Self {
        self = Self::Or(Box::new(self.clone()), Box::new(condition));

        self
    }

    pub fn and(mut self, condition: Self) -> Self {
        self = Self::And(Box::new(self.clone()), Box::new(condition));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_condition() {
        let condition = Condition::Eq("age".to_string(), "30".to_string());
        assert_eq!(condition.build_conditions().unwrap(), "age = 30");
    }

    #[test]
    fn test_neq_condition() {
        let condition = Condition::Neq("status".to_string(), "inactive".to_string());
        assert_eq!(condition.build_conditions().unwrap(), "status != inactive");
    }

    #[test]
    fn test_null_condition() {
        let condition = Condition::Null("address".to_string());
        assert_eq!(condition.build_conditions().unwrap(), "address IS NULL");
    }

    #[test]
    fn test_not_null_condition() {
        let condition = Condition::NotNull("phone".to_string());
        assert_eq!(condition.build_conditions().unwrap(), "phone IS NOT NULL");
    }

    #[test]
    fn test_in_condition() {
        let condition = Condition::In(
            "id".to_string(),
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
        );
        assert_eq!(condition.build_conditions().unwrap(), "id IN (1, 2, 3)");
    }

    #[test]
    fn test_not_in_condition() {
        let condition = Condition::NotIn(
            "id".to_string(),
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
        );
        assert_eq!(condition.build_conditions().unwrap(), "id NOT IN (1, 2, 3)");
    }

    #[test]
    fn test_between_condition() {
        let condition =
            Condition::Between("salary".to_string(), "1000".to_string(), "5000".to_string());
        assert_eq!(
            condition.build_conditions().unwrap(),
            "salary BETWEEN 1000 And 5000"
        );
    }

    #[test]
    fn test_like_condition() {
        let condition = Condition::Like("name".to_string(), "John%".to_string());
        assert_eq!(condition.build_conditions().unwrap(), "name Like 'John%'");
    }

    #[test]
    fn test_and_condition() {
        let condition = Condition::Eq("age".to_string(), "30".to_string())
            .and(Condition::NotNull("address".to_string()));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(age = 30 AND address IS NOT NULL)"
        );
    }

    #[test]
    fn test_or_condition() {
        let condition = Condition::Eq("age".to_string(), "30".to_string())
            .or(Condition::Null("address".to_string()));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(age = 30 OR address IS NULL)"
        );
    }
    #[test]
    fn test_nested_and_or_condition() {
        let condition = Condition::Eq("age".to_string(), "30".to_string())
            .and(Condition::NotNull("address".to_string()))
            .or(Condition::Lt("salary".to_string(), "5000".to_string()));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((age = 30 AND address IS NOT NULL) OR salary < 5000)"
        );
    }

    #[test]
    fn test_nested_or_and_condition() {
        let condition = Condition::Eq("age".to_string(), "30".to_string())
            .or(Condition::NotNull("address".to_string()))
            .and(Condition::Lt("salary".to_string(), "5000".to_string()));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((age = 30 OR address IS NOT NULL) AND salary < 5000)"
        );
    }

    #[test]
    fn test_combined_conditions_with_in() {
        let condition = Condition::In(
            "id".to_string(),
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
        )
        .and(Condition::Gt("score".to_string(), "90".to_string()))
        .or(Condition::Null("status".to_string()));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((id IN (1, 2, 3) AND score > 90) OR status IS NULL)"
        );
    }

    #[test]
    fn test_combined_conditions_with_between() {
        let condition =
            Condition::Between("age".to_string(), "18".to_string(), "30".to_string()).and(
                Condition::NotIn("id".to_string(), vec!["4".to_string(), "5".to_string()]),
            );
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(age BETWEEN 18 And 30 AND id NOT IN (4, 5))"
        );
    }

    #[test]
    fn test_complex_combination_with_like() {
        let condition = Condition::Like("name".to_string(), "John%".to_string())
            .and(Condition::Eq("status".to_string(), "active".to_string()))
            .or(Condition::In(
                "department".to_string(),
                vec!["IT".to_string(), "HR".to_string()],
            ));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((name Like 'John%' AND status = active) OR department IN (IT, HR))"
        );
    }

    #[test]
    fn test_combined_nested_conditions() {
        let condition = Condition::Eq("age".to_string(), "30".to_string()).and(Condition::Or(
            Box::new(Condition::Null("address".to_string())),
            Box::new(Condition::Gt("salary".to_string(), "5000".to_string())),
        ));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(age = 30 AND (address IS NULL OR salary > 5000))"
        );
    }

    #[test]
    fn test_combined_conditions_with_not_in() {
        let condition = Condition::NotIn("id".to_string(), vec!["1".to_string(), "2".to_string()])
            .or(Condition::Lt("age".to_string(), "20".to_string()));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(id NOT IN (1, 2) OR age < 20)"
        );
    }

    #[test]
    fn test_combined_conditions_with_eq_and_like() {
        let condition = Condition::Eq("role".to_string(), "admin".to_string()).and(
            Condition::Like("email".to_string(), "%@example.com".to_string()),
        );
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(role = admin AND email Like '%@example.com')"
        );
    }

    #[test]
    fn test_combined_conditions_with_gt_and_lte() {
        let condition = Condition::Gt("created_at".to_string(), "2023-01-01".to_string()).and(
            Condition::Lte("created_at".to_string(), "2023-12-31".to_string()),
        );
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(created_at > 2023-01-01 AND created_at <= 2023-12-31)"
        );
    }

    #[test]
    fn test_complex_nested_conditions() {
        let condition = Condition::Or(
            Box::new(
                Condition::Eq("age".to_string(), "30".to_string())
                    .and(Condition::NotNull("address".to_string())),
            ),
            Box::new(Condition::Lt("salary".to_string(), "5000".to_string()).or(
                Condition::Between("experience".to_string(), "2".to_string(), "5".to_string()),
            )),
        );
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((age = 30 AND address IS NOT NULL) OR (salary < 5000 OR experience BETWEEN 2 And 5))"
        );
    }
    #[test]
    fn test_deeply_nested_and_or_conditions() {
        let condition = Condition::And(
            Box::new(Condition::Or(
                Box::new(Condition::Eq("age".to_string(), "30".to_string())),
                Box::new(Condition::Gt("salary".to_string(), "5000".to_string())),
            )),
            Box::new(Condition::And(
                Box::new(Condition::NotNull("address".to_string())),
                Box::new(Condition::Or(
                    Box::new(Condition::Lt("experience".to_string(), "10".to_string())),
                    Box::new(Condition::Like("name".to_string(), "%Smith%".to_string())),
                )),
            )),
        );
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((age = 30 OR salary > 5000) AND (address IS NOT NULL AND (experience < 10 OR name Like '%Smith%')))"
        );
    }

    #[test]
    fn test_combined_in_not_in_with_and_or() {
        let condition = Condition::In("id".to_string(), vec!["1".to_string(), "2".to_string()])
            .or(Condition::NotIn(
                "department".to_string(),
                vec!["HR".to_string(), "Finance".to_string()],
            ))
            .and(Condition::Eq("status".to_string(), "active".to_string()));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((id IN (1, 2) OR department NOT IN (HR, Finance)) AND status = active)"
        );
    }

    #[test]
    fn test_combined_between_with_nested_and_or() {
        let condition = Condition::Between("age".to_string(), "18".to_string(), "30".to_string())
            .and(Condition::Or(
                Box::new(Condition::Lt("salary".to_string(), "4000".to_string())),
                Box::new(Condition::Gt("salary".to_string(), "6000".to_string())),
            ));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(age BETWEEN 18 And 30 AND (salary < 4000 OR salary > 6000))"
        );
    }

    #[test]
    fn test_combined_not_null_with_in_and_between() {
        let condition = Condition::NotNull("name".to_string())
            .and(Condition::In(
                "department".to_string(),
                vec!["IT".to_string(), "Sales".to_string()],
            ))
            .or(Condition::Between(
                "age".to_string(),
                "25".to_string(),
                "35".to_string(),
            ));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((name IS NOT NULL AND department IN (IT, Sales)) OR age BETWEEN 25 And 35)"
        );
    }

    #[test]
    fn test_combined_eq_with_multiple_and_or() {
        let condition = Condition::Eq("role".to_string(), "manager".to_string())
            .and(Condition::Or(
                Box::new(Condition::Eq("department".to_string(), "IT".to_string())),
                Box::new(Condition::Eq("department".to_string(), "HR".to_string())),
            ))
            .and(Condition::Gt("experience".to_string(), "5".to_string()))
            .or(Condition::Lt("age".to_string(), "40".to_string()));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(((role = manager AND (department = IT OR department = HR)) AND experience > 5) OR age < 40)"
        );
    }

    #[test]
    fn test_combined_gt_lte_like_and_in() {
        let condition = Condition::Gt("created_at".to_string(), "2022-01-01".to_string())
            .and(Condition::Lte(
                "created_at".to_string(),
                "2023-01-01".to_string(),
            ))
            .and(Condition::Like(
                "description".to_string(),
                "%important%".to_string(),
            ))
            .or(Condition::In(
                "category".to_string(),
                vec!["urgent".to_string(), "critical".to_string()],
            ));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(((created_at > 2022-01-01 AND created_at <= 2023-01-01) AND description Like '%important%') OR category IN (urgent, critical))"
        );
    }

    #[test]
    fn test_combined_complex_conditions() {
        let condition = Condition::And(
            Box::new(Condition::And(
                Box::new(Condition::Gt("age".to_string(), "25".to_string())),
                Box::new(Condition::Lt("age".to_string(), "35".to_string())),
            )),
            Box::new(Condition::Or(
                Box::new(Condition::NotNull("address".to_string())),
                Box::new(Condition::And(
                    Box::new(Condition::Null("address".to_string())),
                    Box::new(Condition::Like(
                        "description".to_string(),
                        "%urgent%".to_string(),
                    )),
                )),
            )),
        )
        .or(Condition::In(
            "department".to_string(),
            vec!["IT".to_string(), "HR".to_string()],
        ));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(((age > 25 AND age < 35) AND (address IS NOT NULL OR (address IS NULL AND description Like '%urgent%'))) OR department IN (IT, HR))"
        );
    }

    #[test]
    fn test_combined_ne_null_in_like_conditions() {
        let condition = Condition::Neq("status".to_string(), "inactive".to_string())
            .and(Condition::Null("terminated_at".to_string()))
            .and(Condition::In(
                "region".to_string(),
                vec!["North".to_string(), "South".to_string()],
            ))
            .or(Condition::Like(
                "remarks".to_string(),
                "%urgent%".to_string(),
            ));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(((status != inactive AND terminated_at IS NULL) AND region IN (North, South)) OR remarks Like '%urgent%')"
        );
    }

    #[test]
    fn test_combined_and_or_not_in_like_conditions() {
        let condition = Condition::And(
            Box::new(Condition::NotIn(
                "status".to_string(),
                vec!["inactive".to_string(), "pending".to_string()],
            )),
            Box::new(Condition::Or(
                Box::new(Condition::Like(
                    "remarks".to_string(),
                    "%critical%".to_string(),
                )),
                Box::new(Condition::Like("remarks".to_string(), "%high%".to_string())),
            )),
        )
        .and(Condition::Gt("priority".to_string(), "1".to_string()));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((status NOT IN (inactive, pending) AND (remarks Like '%critical%' OR remarks Like '%high%')) AND priority > 1)"
        );
    }
    #[test]
    fn test_complex_combined_conditions_with_all_operators() {
        let condition = Condition::And(
            Box::new(Condition::Or(
                Box::new(Condition::And(
                    Box::new(Condition::Eq("role".to_string(), "admin".to_string())),
                    Box::new(Condition::NotNull("last_login".to_string())),
                )),
                Box::new(Condition::In(
                    "department".to_string(),
                    vec!["IT".to_string(), "HR".to_string()],
                )),
            )),
            Box::new(Condition::And(
                Box::new(Condition::Between(
                    "age".to_string(),
                    "25".to_string(),
                    "45".to_string(),
                )),
                Box::new(Condition::Like(
                    "email".to_string(),
                    "%@example.com".to_string(),
                )),
            )),
        )
        .or(Condition::NotIn(
            "status".to_string(),
            vec!["inactive".to_string(), "banned".to_string()],
        ));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((((role = admin AND last_login IS NOT NULL) OR department IN (IT, HR)) AND (age BETWEEN 25 And 45 AND email Like '%@example.com')) OR status NOT IN (inactive, banned))"
        );
    }
    #[test]
    fn test_nested_and_or_with_in_not_in() {
        let condition = Condition::And(
            Box::new(Condition::Or(
                Box::new(Condition::In(
                    "country".to_string(),
                    vec!["USA".to_string(), "Canada".to_string()],
                )),
                Box::new(Condition::NotIn(
                    "state".to_string(),
                    vec!["Texas".to_string(), "Florida".to_string()],
                )),
            )),
            Box::new(Condition::And(
                Box::new(Condition::NotNull("city".to_string())),
                Box::new(Condition::Like("name".to_string(), "%Corp%".to_string())),
            )),
        );
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((country IN ('USA', 'Canada') OR state NOT IN ('Texas', 'Florida')) AND (city IS NOT NULL AND name Like '%Corp%'))"
        );
    }
}
