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

use super::value::SqlValue;

#[derive(Clone)]
pub enum Condition {
    Native(String),
    Null(String),
    NotNull(String),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    In(String, Vec<SqlValue>),
    NotIn(String, Vec<SqlValue>),
    Eq(String, SqlValue),
    Neq(String, SqlValue),
    Between(String, SqlValue, SqlValue),
    Gt(String, SqlValue),
    Gte(String, SqlValue),
    Lt(String, SqlValue),
    Lte(String, SqlValue),
    Like(String, SqlValue),
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
                        .map(|v| v.to_sql())
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{} IN ({})", f, values)
                }
                Condition::NotIn(f, d) => {
                    let values = d
                        .iter()
                        .map(|v| v.to_sql())
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{} NOT IN ({})", f, values)
                }
                Condition::Eq(f, d) => format!("{} = {}", f, d.to_sql()),
                Condition::Neq(f, d) => format!("{} != {}", f, d.to_sql()),
                Condition::Lt(f, d) => format!("{} < {}", f, d.to_sql()),
                Condition::Lte(f, d) => format!("{} <= {}", f, d.to_sql()),
                Condition::Gt(f, d) => format!("{} > {}", f, d.to_sql()),
                Condition::Gte(f, d) => format!("{} >= {}", f, d.to_sql()),
                Condition::Like(f, d) => format!("{} Like {}", f, d.to_sql()),
                Condition::Between(f, a, b) => {
                    format!("{} BETWEEN {} AND {}", f, a.to_sql(), b.to_sql())
                }
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
        let condition = Condition::Eq("age".to_string(), SqlValue::Int(30));
        assert_eq!(condition.build_conditions().unwrap(), "age = 30");
    }

    #[test]
    fn test_neq_condition() {
        let condition = Condition::Neq(
            "status".to_string(),
            SqlValue::from_string_slice("inactive"),
        );
        assert_eq!(
            condition.build_conditions().unwrap(),
            "status != \'inactive\'"
        );
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
            vec![SqlValue::Int(1), SqlValue::Int(2), SqlValue::Int(3)],
        );
        assert_eq!(condition.build_conditions().unwrap(), "id IN (1, 2, 3)");
    }
    #[test]
    fn test_not_in_condition() {
        let condition = Condition::NotIn(
            "id".to_string(),
            vec![SqlValue::Int(1), SqlValue::Int(2), SqlValue::Int(3)],
        );
        assert_eq!(condition.build_conditions().unwrap(), "id NOT IN (1, 2, 3)");
    }

    #[test]
    fn test_between_condition() {
        let condition = Condition::Between(
            "salary".to_string(),
            SqlValue::Int(1000),
            SqlValue::Int(5000),
        );
        assert_eq!(
            condition.build_conditions().unwrap(),
            "salary BETWEEN 1000 AND 5000"
        );
    }

    #[test]
    fn test_like_condition() {
        let condition = Condition::Like("name".to_string(), SqlValue::from_string_slice("John%"));
        assert_eq!(condition.build_conditions().unwrap(), "name Like 'John%'");
    }

    #[test]
    fn test_gt_condition() {
        let condition = Condition::Gt("age".to_string(), SqlValue::Int(30));
        assert_eq!(condition.build_conditions().unwrap(), "age > 30");
    }

    #[test]
    fn test_lte_condition() {
        let condition = Condition::Lte("age".to_string(), SqlValue::Int(30));
        assert_eq!(condition.build_conditions().unwrap(), "age <= 30");
    }
    #[test]
    fn test_and_condition() {
        let condition = Condition::Eq("age".to_string(), SqlValue::Int(30))
            .and(Condition::NotNull("address".to_string()));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(age = 30 AND address IS NOT NULL)"
        );
    }

    #[test]
    fn test_or_condition() {
        let condition = Condition::Eq("age".to_string(), SqlValue::Int(30))
            .or(Condition::Null("address".to_string()));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(age = 30 OR address IS NULL)"
        );
    }

    #[test]
    fn test_nested_and_or_condition() {
        let condition = Condition::Eq("age".to_string(), SqlValue::Int(30))
            .and(Condition::NotNull("address".to_string()))
            .or(Condition::Lt("salary".to_string(), SqlValue::Int(5000)));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((age = 30 AND address IS NOT NULL) OR salary < 5000)"
        );
    }

    #[test]
    fn test_nested_or_and_condition() {
        let condition = Condition::Eq("age".to_string(), SqlValue::Int(30))
            .or(Condition::NotNull("address".to_string()))
            .and(Condition::Lt("salary".to_string(), SqlValue::Int(5000)));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((age = 30 OR address IS NOT NULL) AND salary < 5000)"
        );
    }

    #[test]
    fn test_gt_and_lte_condition() {
        let condition = Condition::Gt("age".to_string(), SqlValue::Int(25))
            .and(Condition::Lte("age".to_string(), SqlValue::Int(35)));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(age > 25 AND age <= 35)"
        );
    }

    #[test]
    fn test_eq_and_like_condition() {
        let condition = Condition::Eq("name".to_string(), SqlValue::from_string_slice("John")).and(
            Condition::Like("surname".to_string(), SqlValue::from_string_slice("Doe%")),
        );
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(name = 'John' AND surname Like 'Doe%')"
        );
    }

    #[test]
    fn test_nested_conditions_with_between() {
        let condition = Condition::Eq("department".to_string(), SqlValue::from_string_slice("HR"))
            .and(Condition::Between(
                "salary".to_string(),
                SqlValue::Int(3000),
                SqlValue::Int(7000),
            ))
            .or(Condition::Like(
                "position".to_string(),
                SqlValue::from_string_slice("Manager%"),
            ));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((department = 'HR' AND salary BETWEEN 3000 AND 7000) OR position Like 'Manager%')"
        );
    }

    #[test]
    fn test_complex_condition_with_multiple_and_or() {
        let condition = Condition::Eq("status".to_string(), SqlValue::from_string_slice("active"))
            .and(
                Condition::Eq("role".to_string(), SqlValue::from_string_slice("admin"))
                    .or(Condition::NotNull("last_login".to_string())),
            )
            .and(Condition::Gt("age".to_string(), SqlValue::Int(25)))
            .or(Condition::Lt("age".to_string(), SqlValue::Int(60)));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(((status = 'active' AND (role = 'admin' OR last_login IS NOT NULL)) AND age > 25) OR age < 60)"
        );
    }

    #[test]
    fn test_gt_or_lte_and_like_condition() {
        let condition = Condition::Gt("experience".to_string(), SqlValue::Int(5))
            .or(Condition::Lte("experience".to_string(), SqlValue::Int(2)))
            .and(Condition::Like(
                "skills".to_string(),
                SqlValue::from_string_slice("%Rust%"),
            ));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((experience > 5 OR experience <= 2) AND skills Like '%Rust%')"
        );
    }

    #[test]
    fn test_nested_conditions_with_not_in() {
        let condition = Condition::NotIn(
            "id".to_string(),
            vec![SqlValue::Int(1), SqlValue::Int(2), SqlValue::Int(3)],
        )
        .and(
            Condition::Like("name".to_string(), SqlValue::from_string_slice("Alice%"))
                .or(Condition::Null("address".to_string())),
        );
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(id NOT IN (1, 2, 3) AND (name Like 'Alice%' OR address IS NULL))"
        );
    }

    #[test]
    fn test_complex_condition_with_multiple_between_and_like() {
        let condition = Condition::Between(
            "salary".to_string(),
            SqlValue::Int(4000),
            SqlValue::Int(8000),
        )
        .and(Condition::Like(
            "position".to_string(),
            SqlValue::from_string_slice("%Engineer%"),
        ))
        .or(
            Condition::Between("age".to_string(), SqlValue::Int(30), SqlValue::Int(50))
                .and(Condition::NotNull("department".to_string())),
        );
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((salary BETWEEN 4000 AND 8000 AND position Like '%Engineer%') OR (age BETWEEN 30 AND 50 AND department IS NOT NULL))"
        );
    }
    #[test]
    fn test_nested_conditions_with_multiple_and_or_not_in() {
        let condition = Condition::Eq("status".to_string(), SqlValue::from_string_slice("active"))
            .and(Condition::Or(
                Box::new(Condition::NotNull("last_login".to_string())),
                Box::new(Condition::NotIn(
                    "department".to_string(),
                    vec![
                        SqlValue::from_string_slice("HR"),
                        SqlValue::from_string_slice("IT"),
                    ],
                )),
            ))
            .and(Condition::Gt("age".to_string(), SqlValue::Int(25)))
            .or(Condition::Lt("age".to_string(), SqlValue::Int(60)));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(((status = 'active' AND (last_login IS NOT NULL OR department NOT IN ('HR', 'IT'))) AND age > 25) OR age < 60)"
        );
    }

    #[test]
    fn test_complex_condition_with_multiple_between_and_like_not_null() {
        let condition = Condition::Between(
            "salary".to_string(),
            SqlValue::Int(4000),
            SqlValue::Int(8000),
        )
        .and(Condition::Like(
            "position".to_string(),
            SqlValue::from_string_slice("%Engineer%"),
        ))
        .or(
            Condition::Between("age".to_string(), SqlValue::Int(30), SqlValue::Int(50))
                .and(Condition::NotNull("department".to_string())),
        )
        .and(Condition::NotIn(
            "location".to_string(),
            vec![
                SqlValue::from_string_slice("New York"),
                SqlValue::from_string_slice("London"),
            ],
        ));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(((salary BETWEEN 4000 AND 8000 AND position Like '%Engineer%') OR (age BETWEEN 30 AND 50 AND department IS NOT NULL)) AND location NOT IN ('New York', 'London'))"
        );
    }

    #[test]
    fn test_nested_conditions_with_multiple_gt_lte_and_like() {
        let condition = Condition::Gt("experience".to_string(), SqlValue::Int(5))
            .or(Condition::Lte("experience".to_string(), SqlValue::Int(2)))
            .and(Condition::Like(
                "skills".to_string(),
                SqlValue::from_string_slice("%Rust%"),
            ))
            .or(Condition::Gte("salary".to_string(), SqlValue::Int(7000)));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "(((experience > 5 OR experience <= 2) AND skills Like '%Rust%') OR salary >= 7000)"
        );
    }

    #[test]
    fn test_complex_condition_with_nested_and_or_like_not_null() {
        let condition = Condition::Eq("status".to_string(), SqlValue::from_string_slice("active"))
            .and(Condition::Or(
                Box::new(Condition::NotNull("last_login".to_string())),
                Box::new(
                    Condition::Like(
                        "department".to_string(),
                        SqlValue::from_string_slice("%Eng%"),
                    )
                    .and(Condition::NotNull("manager".to_string())),
                ),
            ))
            .or(Condition::Like(
                "role".to_string(),
                SqlValue::from_string_slice("%admin%"),
            ));
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((status = 'active' AND (last_login IS NOT NULL OR (department Like '%Eng%' AND manager IS NOT NULL))) OR role Like '%admin%')"
        );
    }

    #[test]
    fn test_complex_condition_with_multiple_or_between_and_like() {
        let condition = Condition::Or(
            Box::new(
                Condition::Eq("status".to_string(), SqlValue::from_string_slice("active")).and(
                    Condition::Or(
                        Box::new(Condition::NotNull("last_login".to_string())),
                        Box::new(
                            Condition::Like(
                                "department".to_string(),
                                SqlValue::from_string_slice("%Eng%"),
                            )
                            .and(Condition::NotNull("manager".to_string())),
                        ),
                    ),
                ),
            ),
            Box::new(
                Condition::Between(
                    "salary".to_string(),
                    SqlValue::Int(5000),
                    SqlValue::Int(10000),
                )
                .or(Condition::In(
                    "age_group".to_string(),
                    vec![
                        SqlValue::from_string_slice("20-30"),
                        SqlValue::from_string_slice("30-40"),
                        SqlValue::from_string_slice("40-50"),
                    ],
                )
                .and(Condition::NotNull("address".to_string()))),
            ),
        );
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((status = 'active' AND (last_login IS NOT NULL OR (department Like '%Eng%' AND manager IS NOT NULL))) OR (salary BETWEEN 5000 AND 10000 OR (age_group IN ('20-30', '30-40', '40-50') AND address IS NOT NULL)))"
        );
    }
    #[test]
    fn test_complex_condition_with_multiple_nested_conditions() {
        let condition = Condition::And(
            Box::new(
                Condition::Eq("status".to_string(), SqlValue::from_string_slice("active"))
                    .or(Condition::NotNull("last_login".to_string())),
            ),
            Box::new(Condition::Or(
                Box::new(
                    Condition::Gte("salary".to_string(), SqlValue::Int(7000))
                        .and(Condition::NotNull("position".to_string())),
                ),
                Box::new(
                    Condition::Lt("age".to_string(), SqlValue::Int(30))
                        .or(Condition::NotNull("department".to_string())),
                ),
            )),
        );
        assert_eq!(
            condition.build_conditions().unwrap(),
            "((status = 'active' OR last_login IS NOT NULL) AND ((salary >= 7000 AND position IS NOT NULL) OR (age < 30 OR department IS NOT NULL)))"
        );
    }
}
