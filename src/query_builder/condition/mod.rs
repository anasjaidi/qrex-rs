// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   mod.rs                                             :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: ajaidi <ajaidi@student.42.fr>              +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2024/06/04 23:44:30 by ajaidi            #+#    #+#             //
//   Updated: 2024/06/05 00:19:32 by ajaidi           ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

#[derive(Debug, Clone)]
pub enum WhereCondition {
    NATIVE(String),
    NULL(String),
    NOTNULL(String),
    AND(Box<Self>, Box<Self>),
    OR(Box<Self>, Box<Self>),
    IN(String, Vec<String>),
    NOTIN(String, Vec<String>),
    EQ(String, String),
    NEQ(String, String),
    BETWEEN(String, String, String),
    GT(String, String),
    GTE(String, String),
    LT(String, String),
    LTE(String, String),
    LIKE(String, String),
}

pub trait Condition {
    fn set_condition(&mut self, condition: WhereCondition);
    fn get_condition(&self) -> Option<&WhereCondition>;

    fn build_conditions(&self) -> Option<String> {
        let condition = self.get_condition();
        if condition.is_none() {
            return None;
        }
        let condition = condition.unwrap();
        fn gc(c: &WhereCondition) -> String {
            match c {
                WhereCondition::OR(lhs, rhs) => format!("({} OR {})", gc(lhs), gc(rhs)),
                WhereCondition::AND(lhs, rhs) => format!("({} AND {})", gc(lhs), gc(rhs)),
                WhereCondition::NULL(f) => format!("{} IS NULL", f),
                WhereCondition::NOTNULL(f) => format!("{} IS NOT NULL", f),
                WhereCondition::IN(f, d) => {
                    let values = d
                        .iter()
                        .map(|v| format!("{}", v))
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{} IN ({})", f, values)
                }
                WhereCondition::NOTIN(f, d) => {
                    let values = d
                        .iter()
                        .map(|v| format!("{}", v))
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{} NOT IN ({})", f, values)
                }
                WhereCondition::EQ(f, d) => format!("{} = {}", f, d),
                WhereCondition::NEQ(f, d) => format!("{} != {}", f, d),
                WhereCondition::LT(f, d) => format!("{} < {}", f, d),
                WhereCondition::LTE(f, d) => format!("{} <= {}", f, d),
                WhereCondition::GT(f, d) => format!("{} > {}", f, d),
                WhereCondition::GTE(f, d) => format!("{} >= {}", f, d),
                WhereCondition::LIKE(f, d) => format!("{} LIKE '{}'", f, d),
                WhereCondition::BETWEEN(f, a, b) => format!("{} BETWEEN {} AND {}", f, a, b),
                WhereCondition::NATIVE(f) => f.clone(),
            }
        }
        Some(gc(condition))
    }

    fn or_where(&mut self, condition: WhereCondition) -> &Self {
        if let Some(c) = self.get_condition() {
            self.set_condition(WhereCondition::OR(Box::new(c.clone()), Box::new(condition)))
        } else {
            self.set_condition(condition);
        }
        self
    }

    fn whene(&mut self, condition: WhereCondition) -> &Self {
        if let Some(c) = self.get_condition() {
            self.set_condition(WhereCondition::AND(
                Box::new(c.clone()),
                Box::new(condition),
            ))
        } else {
            self.set_condition(condition);
        }
        self
    }
}

#[cfg(test)]
mod test {
    use super::{Condition, WhereCondition};

    #[derive(Default)]
    struct ConditionImpl {
        c: Option<WhereCondition>,
    }

    impl Condition for ConditionImpl {
        fn get_condition(&self) -> Option<&WhereCondition> {
            self.c.as_ref()
        }
        fn set_condition(&mut self, condition: WhereCondition) {
            self.c = Some(condition)
        }
    }

    #[test]
    fn test_none() {
        let b = ConditionImpl::default();
        assert!(b.build_conditions() == None)
    }

    #[test]
    fn test_very_deep_nested_and_or_1() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::OR(
                    Box::new(WhereCondition::AND(
                        Box::new(WhereCondition::OR(
                            Box::new(WhereCondition::NULL("field1".to_owned())),
                            Box::new(WhereCondition::NOTNULL("field2".to_owned()))
                        )),
                        Box::new(WhereCondition::AND(
                            Box::new(WhereCondition::NULL("field3".to_owned())),
                            Box::new(WhereCondition::NOTNULL("field4".to_owned()))
                        ))
                    )),
                    Box::new(WhereCondition::NULL("field5".to_owned()))
                )),
                Box::new(WhereCondition::NOTNULL("field6".to_owned()))
            )).build_conditions().unwrap(),
            "((((field1 IS NULL OR field2 IS NOT NULL) AND (field3 IS NULL AND field4 IS NOT NULL)) OR field5 IS NULL) AND field6 IS NOT NULL)"
        );
    }

    #[test]
    fn test_very_deep_nested_and_or_2() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::OR(
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::OR(
                        Box::new(WhereCondition::AND(
                            Box::new(WhereCondition::NULL("field1".to_owned())),
                            Box::new(WhereCondition::NOTNULL("field2".to_owned()))
                        )),
                        Box::new(WhereCondition::NULL("field3".to_owned()))
                    )),
                    Box::new(WhereCondition::AND(
                        Box::new(WhereCondition::NOTNULL("field4".to_owned())),
                        Box::new(WhereCondition::NULL("field5".to_owned()))
                    ))
                )),
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::NOTNULL("field6".to_owned())),
                    Box::new(WhereCondition::NULL("field7".to_owned()))
                ))
            )).build_conditions().unwrap(),
            "((((field1 IS NULL AND field2 IS NOT NULL) OR field3 IS NULL) AND (field4 IS NOT NULL AND field5 IS NULL)) OR (field6 IS NOT NULL AND field7 IS NULL))"
        );
    }

    fn test_very_deep_nested_and_or_3() {
        let mut b = ConditionImpl::default();
        let query = b
            .whene(WhereCondition::AND(
                Box::new(WhereCondition::OR(
                    Box::new(WhereCondition::AND(
                        Box::new(WhereCondition::OR(
                            Box::new(WhereCondition::AND(
                                Box::new(WhereCondition::NULL("field1".to_owned())),
                                Box::new(WhereCondition::NOTNULL("field2".to_owned())),
                            )),
                            Box::new(WhereCondition::NULL("field3".to_owned())),
                        )),
                        Box::new(WhereCondition::NULL("field4".to_owned())),
                    )),
                    Box::new(WhereCondition::NOTNULL("field5".to_owned())),
                )),
                Box::new(WhereCondition::OR(
                    Box::new(WhereCondition::NULL("field6".to_owned())),
                    Box::new(WhereCondition::NOTNULL("field7".to_owned())),
                )),
            ))
            .build_conditions()
            .unwrap();

        assert_eq!(
            query,
            "(((((field1 IS NULL AND field2 IS NOT NULL) OR field3 IS NULL) AND field4 IS NULL) OR field5 IS NOT NULL) AND (field6 IS NULL OR field7 IS NOT NULL))"
        );
    }

    #[test]
    fn test_very_deep_nested_and_or_4() {
        let mut b = ConditionImpl::default();
        let query = b
            .whene(WhereCondition::OR(
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::OR(
                        Box::new(WhereCondition::AND(
                            Box::new(WhereCondition::OR(
                                Box::new(WhereCondition::NULL("field1".to_owned())),
                                Box::new(WhereCondition::NOTNULL("field2".to_owned())),
                            )),
                            Box::new(WhereCondition::NULL("field3".to_owned())),
                        )),
                        Box::new(WhereCondition::AND(
                            Box::new(WhereCondition::NOTNULL("field4".to_owned())),
                            Box::new(WhereCondition::NULL("field5".to_owned())),
                        )),
                    )),
                    Box::new(WhereCondition::NOTNULL("field6".to_owned())),
                )),
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::OR(
                        Box::new(WhereCondition::NULL("field7".to_owned())),
                        Box::new(WhereCondition::NOTNULL("field8".to_owned())),
                    )),
                    Box::new(WhereCondition::NULL("field9".to_owned())),
                )),
            ))
            .build_conditions()
            .unwrap();

        assert_eq!(
            query,
            "(((((field1 IS NULL OR field2 IS NOT NULL) AND field3 IS NULL) OR (field4 IS NOT NULL AND field5 IS NULL)) AND field6 IS NOT NULL) OR ((field7 IS NULL OR field8 IS NOT NULL) AND field9 IS NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_1() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::OR(
                    Box::new(WhereCondition::NULL("field1".to_owned())),
                    Box::new(WhereCondition::NOTNULL("field2".to_owned()))
                )),
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::NULL("field3".to_owned())),
                    Box::new(WhereCondition::NOTNULL("field4".to_owned()))
                ))
            ))
            .build_conditions()
            .unwrap(),
            "((field1 IS NULL OR field2 IS NOT NULL) AND (field3 IS NULL AND field4 IS NOT NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_2() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::NULL("field1".to_owned())),
                    Box::new(WhereCondition::NOTNULL("field2".to_owned()))
                )),
                Box::new(WhereCondition::OR(
                    Box::new(WhereCondition::NULL("field3".to_owned())),
                    Box::new(WhereCondition::NOTNULL("field4".to_owned()))
                ))
            ))
            .build_conditions()
            .unwrap(),
            "((field1 IS NULL AND field2 IS NOT NULL) AND (field3 IS NULL OR field4 IS NOT NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_3() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::OR(
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::NULL("field1".to_owned())),
                    Box::new(WhereCondition::NOTNULL("field2".to_owned()))
                )),
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::NULL("field3".to_owned())),
                    Box::new(WhereCondition::NOTNULL("field4".to_owned()))
                ))
            ))
            .build_conditions()
            .unwrap(),
            "((field1 IS NULL AND field2 IS NOT NULL) OR (field3 IS NULL AND field4 IS NOT NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_4() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::OR(
                Box::new(WhereCondition::OR(
                    Box::new(WhereCondition::NULL("field1".to_owned())),
                    Box::new(WhereCondition::NOTNULL("field2".to_owned()))
                )),
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::NULL("field3".to_owned())),
                    Box::new(WhereCondition::NOTNULL("field4".to_owned()))
                ))
            ))
            .build_conditions()
            .unwrap(),
            "((field1 IS NULL OR field2 IS NOT NULL) OR (field3 IS NULL AND field4 IS NOT NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_5() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::OR(
                    Box::new(WhereCondition::NULL("field1".to_owned())),
                    Box::new(WhereCondition::AND(
                        Box::new(WhereCondition::NOTNULL("field2".to_owned())),
                        Box::new(WhereCondition::NULL("field3".to_owned()))
                    ))
                )),
                Box::new(WhereCondition::NULL("field4".to_owned()))
            ))
            .build_conditions()
            .unwrap(),
            "((field1 IS NULL OR (field2 IS NOT NULL AND field3 IS NULL)) AND field4 IS NULL)"
        );
    }

    #[test]
    fn test_deep_nested_and_or_6() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::OR(
                        Box::new(WhereCondition::NULL("field1".to_owned())),
                        Box::new(WhereCondition::NOTNULL("field2".to_owned()))
                    )),
                    Box::new(WhereCondition::NULL("field3".to_owned()))
                )),
                Box::new(WhereCondition::NOTNULL("field4".to_owned()))
            ))
            .build_conditions()
            .unwrap(),
            "(((field1 IS NULL OR field2 IS NOT NULL) AND field3 IS NULL) AND field4 IS NOT NULL)"
        );
    }

    #[test]
    fn test_deep_nested_and_or_7() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::OR(
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::OR(
                        Box::new(WhereCondition::NULL("field1".to_owned())),
                        Box::new(WhereCondition::NOTNULL("field2".to_owned()))
                    )),
                    Box::new(WhereCondition::NULL("field3".to_owned()))
                )),
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::NOTNULL("field4".to_owned())),
                    Box::new(WhereCondition::NULL("field5".to_owned()))
                ))
            )).build_conditions().unwrap(),
            "(((field1 IS NULL OR field2 IS NOT NULL) AND field3 IS NULL) OR (field4 IS NOT NULL AND field5 IS NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_8() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::OR(
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::OR(
                        Box::new(WhereCondition::NULL("field1".to_owned())),
                        Box::new(WhereCondition::NOTNULL("field2".to_owned()))
                    )),
                    Box::new(WhereCondition::OR(
                        Box::new(WhereCondition::NULL("field3".to_owned())),
                        Box::new(WhereCondition::NOTNULL("field4".to_owned()))
                    ))
                )),
                Box::new(WhereCondition::NULL("field5".to_owned()))
            )).build_conditions().unwrap(),
            "(((field1 IS NULL OR field2 IS NOT NULL) AND (field3 IS NULL OR field4 IS NOT NULL)) OR field5 IS NULL)"
        );
    }

    #[test]
    fn test_deep_nested_and_or_9() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::AND(
                    Box::new(WhereCondition::OR(
                        Box::new(WhereCondition::NULL("field1".to_owned())),
                        Box::new(WhereCondition::NOTNULL("field2".to_owned()))
                    )),
                    Box::new(WhereCondition::NULL("field3".to_owned()))
                )),
                Box::new(WhereCondition::OR(
                    Box::new(WhereCondition::NOTNULL("field4".to_owned())),
                    Box::new(WhereCondition::NULL("field5".to_owned()))
                ))
            )).build_conditions().unwrap(),
            "(((field1 IS NULL OR field2 IS NOT NULL) AND field3 IS NULL) AND (field4 IS NOT NULL OR field5 IS NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_10() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::OR(
                    Box::new(WhereCondition::AND(
                        Box::new(WhereCondition::NULL("field1".to_owned())),
                        Box::new(WhereCondition::NOTNULL("field2".to_owned()))
                    )),
                    Box::new(WhereCondition::NULL("field3".to_owned()))
                )),
                Box::new(WhereCondition::NOTNULL("field4".to_owned()))
            ))
            .build_conditions()
            .unwrap(),
            "(((field1 IS NULL AND field2 IS NOT NULL) OR field3 IS NULL) AND field4 IS NOT NULL)"
        );
    }
    #[test]
    fn test_gt_condition() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::GT("age".to_owned(), 30.to_string()))
                .build_conditions()
                .unwrap(),
            "age > 30"
        );
    }

    #[test]
    fn test_gte_condition() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::GTE("age".to_owned(), 30.to_string()))
                .build_conditions()
                .unwrap(),
            "age >= 30"
        );
    }

    #[test]
    fn test_lt_condition() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::LT("age".to_owned(), 30.to_string()))
                .build_conditions()
                .unwrap(),
            "age < 30"
        );
    }

    #[test]
    fn test_le_condition() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::LTE("age".to_owned(), 30.to_string()))
                .build_conditions()
                .unwrap(),
            "age <= 30"
        );
    }

    #[test]
    fn test_native_condition() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::NATIVE("age BETWEEN 20 AND 30".to_owned()))
                .build_conditions()
                .unwrap(),
            "age BETWEEN 20 AND 30"
        );
    }

    #[test]
    fn test_combined_and_or() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::NULL("anas".to_owned())),
                Box::new(WhereCondition::OR(
                    Box::new(WhereCondition::NOTNULL("jaidi".to_owned())),
                    Box::new(WhereCondition::IN(
                        "id".to_owned(),
                        vec![1.to_string(), 2.to_string(), 3.to_string()]
                    ))
                ))
            ))
            .build_conditions()
            .unwrap(),
            "(anas IS NULL AND (jaidi IS NOT NULL OR id IN (1, 2, 3)))"
        );
    }

    #[test]
    fn test_nested_and_or() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::OR(
                    Box::new(WhereCondition::NULL("anas".to_owned())),
                    Box::new(WhereCondition::NOTNULL("jaidi".to_owned()))
                )),
                Box::new(WhereCondition::IN(
                    "id".to_owned(),
                    vec![1.to_string(), 2.to_string(), 3.to_string()]
                ))
            ))
            .build_conditions()
            .unwrap(),
            "((anas IS NULL OR jaidi IS NOT NULL) AND id IN (1, 2, 3))"
        );
    }

    #[test]
    fn test_combined_eq_and_gt() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::EQ("name".to_owned(), 1.to_string())),
                Box::new(WhereCondition::GT("age".to_owned(), 18.to_string()))
            ))
            .build_conditions()
            .unwrap(),
            "(name = 1 AND age > 18)"
        );
    }

    #[test]
    fn test_combined_eq_and_lt() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::EQ("name".to_owned(), 1.to_string())),
                Box::new(WhereCondition::LT("age".to_owned(), 18.to_string()))
            ))
            .build_conditions()
            .unwrap(),
            "(name = 1 AND age < 18)"
        );
    }

    #[test]
    fn test_combined_eq_and_le() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::EQ("name".to_owned(), 1.to_string())),
                Box::new(WhereCondition::LTE("age".to_owned(), 18.to_string()))
            ))
            .build_conditions()
            .unwrap(),
            "(name = 1 AND age <= 18)"
        );
    }

    #[test]
    fn test_combined_eq_and_gte() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::EQ("name".to_owned(), 1.to_string())),
                Box::new(WhereCondition::GTE("age".to_owned(), 18.to_string()))
            ))
            .build_conditions()
            .unwrap(),
            "(name = 1 AND age >= 18)"
        );
    }

    #[test]
    fn test_combined_like_and_between() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::LIKE("name".to_owned(), "John%".to_owned())),
                Box::new(WhereCondition::BETWEEN(
                    "age".to_owned(),
                    20.to_string(),
                    30.to_string()
                ))
            ))
            .build_conditions()
            .unwrap(),
            "(name LIKE 'John%' AND age BETWEEN 20 AND 30)"
        );
    }

    #[test]
    fn test_combined_notin_and_null() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::NOTIN(
                    "id".to_owned(),
                    vec![1.to_string(), 2.to_string(), 3.to_string()]
                )),
                Box::new(WhereCondition::NULL("name".to_owned()))
            ))
            .build_conditions()
            .unwrap(),
            "(id NOT IN (1, 2, 3) AND name IS NULL)"
        );
    }

    #[test]
    fn test_combined_in_and_notnull() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::IN(
                    "id".to_owned(),
                    vec![4.to_string(), 5.to_string(), 6.to_string()]
                )),
                Box::new(WhereCondition::NOTNULL("name".to_owned()))
            ))
            .build_conditions()
            .unwrap(),
            "(id IN (4, 5, 6) AND name IS NOT NULL)"
        );
    }

    #[test]
    fn test_combined_ne_and_gt() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::NEQ("id".to_owned(), 1.to_string())),
                Box::new(WhereCondition::GT("age".to_owned(), 20.to_string()))
            ))
            .build_conditions()
            .unwrap(),
            "(id != 1 AND age > 20)"
        );
    }

    #[test]
    fn test_combined_between_and_in() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::BETWEEN(
                    "age".to_owned(),
                    20.to_string(),
                    30.to_string()
                )),
                Box::new(WhereCondition::IN(
                    "id".to_owned(),
                    vec![4.to_string(), 5.to_string(), 6.to_string()]
                ))
            ))
            .build_conditions()
            .unwrap(),
            "(age BETWEEN 20 AND 30 AND id IN (4, 5, 6))"
        );
    }

    #[test]
    fn test_combined_like_and_in() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::LIKE("name".to_owned(), "John%".to_owned())),
                Box::new(WhereCondition::IN(
                    "id".to_owned(),
                    vec![1.to_string(), 2.to_string(), 3.to_string()]
                ))
            ))
            .build_conditions()
            .unwrap(),
            "(name LIKE 'John%' AND id IN (1, 2, 3))"
        );
    }
    #[test]
    fn test_basic_null() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::NULL("anas".to_owned()))
                .build_conditions()
                .unwrap(),
            "anas IS NULL"
        );
    }

    #[test]
    fn test_basic_notnull() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::NOTNULL("jaidi".to_string()))
                .build_conditions()
                .unwrap(),
            "jaidi IS NOT NULL"
        );
    }

    #[test]
    fn test_and_condition() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::NULL("robin".to_owned())),
                Box::new(WhereCondition::NULL("hood".to_owned()))
            ))
            .build_conditions()
            .unwrap(),
            "(robin IS NULL AND hood IS NULL)"
        );
    }

    #[test]
    fn test_or_condition() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::OR(
                Box::new(WhereCondition::NULL("robin".to_owned())),
                Box::new(WhereCondition::NULL("hood".to_owned()))
            ))
            .build_conditions()
            .unwrap(),
            "(robin IS NULL OR hood IS NULL)"
        );
    }

    #[test]
    fn test_in_condition() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::IN(
                "id".to_owned(),
                vec![1.to_string(), 2.to_string(), 3.to_string()]
            ))
            .build_conditions()
            .unwrap(),
            "id IN (1, 2, 3)"
        );
    }

    #[test]
    fn test_notin_condition() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::NOTIN(
                "id".to_owned(),
                vec![4.to_string(), 5.to_string(), 6.to_string()]
            ))
            .build_conditions()
            .unwrap(),
            "id NOT IN (4, 5, 6)"
        );
    }

    #[test]
    fn test_eq_condition() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::EQ("id".to_owned(), 1.to_string()))
                .build_conditions()
                .unwrap(),
            "id = 1"
        );
    }

    #[test]
    fn test_neq_condition() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::NEQ("id".to_owned(), 1.to_string()))
                .build_conditions()
                .unwrap(),
            "id != 1"
        );
    }

    #[test]
    fn test_like_condition() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::LIKE("name".to_owned(), "John%".to_owned()))
                .build_conditions()
                .unwrap(),
            "name LIKE 'John%'"
        );
    }

    #[test]
    fn test_between_condition() {
        let mut b = ConditionImpl::default();
        assert_eq!(
            b.whene(WhereCondition::BETWEEN(
                "id".to_owned(),
                1.to_string(),
                10.to_string()
            ))
            .build_conditions()
            .unwrap(),
            "id BETWEEN 1 AND 10"
        );
    }
}
