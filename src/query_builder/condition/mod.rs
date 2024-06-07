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

use super::select::GroupBy;

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
    fn build_conditions(self) -> Option<String> {
        fn gc(c: &Condition) -> String {
            match c {
                Condition::Or(lhs, rhs) => format!("({} Or {})", gc(lhs), gc(rhs)),
                Condition::And(lhs, rhs) => format!("({} And {})", gc(lhs), gc(rhs)),
                Condition::Null(f) => format!("{} IS Null", f),
                Condition::NotNull(f) => format!("{} IS NOT Null", f),
                Condition::In(f, d) => {
                    let values = d
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{} In ({})", f, values)
                }
                Condition::NotIn(f, d) => {
                    let values = d
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{} NOT In ({})", f, values)
                }
                Condition::Eq(f, d) => format!("{} = {}", f, d),
                Condition::Neq(f, d) => format!("{} != {}", f, d),
                Condition::Lt(f, d) => format!("{} < {}", f, d),
                Condition::Lte(f, d) => format!("{} <= {}", f, d),
                Condition::Gt(f, d) => format!("{} > {}", f, d),
                Condition::Gte(f, d) => format!("{} >= {}", f, d),
                Condition::Like(f, d) => format!("{} Like '{}'", f, d),
                Condition::Between(f, a, b) => format!("{} Between {} And {}", f, a, b),
                Condition::Native(f) => f.clone(),
            }
        }
        Some(gc(&self))
    }

    fn or(mut self, condition: Self) -> Self {
        self = Self::Or(Box::new(self.clone()), Box::new(condition));

        self
    }

    fn and(mut self, condition: Self) -> Self {
        self = Self::And(Box::new(self.clone()), Box::new(condition));
        self
    }
}

pub trait Having: GroupBy {
    fn set_having(&mut self, condition: Condition);
    fn get_having(&self) -> Option<&Condition>;

    fn build_having(&self) -> Option<String> {
        let condition = self.get_having()?;
        fn gc(c: &Condition) -> String {
            match c {
                Condition::Or(lhs, rhs) => format!("({} Or {})", gc(lhs), gc(rhs)),
                Condition::And(lhs, rhs) => format!("({} And {})", gc(lhs), gc(rhs)),
                Condition::Null(f) => format!("{} IS Null", f),
                Condition::NotNull(f) => format!("{} IS NOT Null", f),
                Condition::In(f, d) => {
                    let values = d
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{} In ({})", f, values)
                }
                Condition::NotIn(f, d) => {
                    let values = d
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    format!("{} NOT In ({})", f, values)
                }
                Condition::Eq(f, d) => format!("{} = {}", f, d),
                Condition::Neq(f, d) => format!("{} != {}", f, d),
                Condition::Lt(f, d) => format!("{} < {}", f, d),
                Condition::Lte(f, d) => format!("{} <= {}", f, d),
                Condition::Gt(f, d) => format!("{} > {}", f, d),
                Condition::Gte(f, d) => format!("{} >= {}", f, d),
                Condition::Like(f, d) => format!("{} Like '{}'", f, d),
                Condition::Between(f, a, b) => format!("{} Between {} And {}", f, a, b),
                Condition::Native(f) => f.clone(),
            }
        }
        Some(gc(condition))
    }

    fn or_having(&mut self, condition: Condition) -> &Self {
        if let Some(c) = self.get_having() {
            self.set_having(Condition::Or(Box::new(c.clone()), Box::new(condition)))
        } else {
            self.set_having(condition);
        }
        self
    }

    fn having(&mut self, condition: Condition) -> &Self {
        if let Some(c) = self.get_having() {
            self.set_having(Condition::And(Box::new(c.clone()), Box::new(condition)))
        } else {
            self.set_having(condition);
        }
        self
    }
}

#[test]
fn test() {
    let mut c = Condition::Null("anas".to_owned())
        .and(Condition::NotNull("jaidi".to_owned()))
        .or(Condition::Eq("anas".to_owned(), "jaidi".to_owned()));

    println!("{:?}", c);
}

//
// #[cfg(test)]
// mod test {
//
//     #[test]
//     fn test_none() {
//         let b =;
//         assert!(b.build_conditions().is_none())
//     }
//
//     #[test]
//     fn test_very_deep_nested_and_or_1() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Or(
//                     Box::new(Condition::And(
//                         Box::new(Condition::Or(
//                             Box::new(Condition::Null("field1".to_owned())),
//                             Box::new(Condition::NotNull("field2".to_owned()))
//                         )),
//                         Box::new(Condition::And(
//                             Box::new(Condition::Null("field3".to_owned())),
//                             Box::new(Condition::NotNull("field4".to_owned()))
//                         ))
//                     )),
//                     Box::new(Condition::Null("field5".to_owned()))
//                 )),
//                 Box::new(Condition::NotNull("field6".to_owned()))
//             )).build_conditions().unwrap(),
//             "((((field1 IS Null Or field2 IS NOT Null) And (field3 IS Null And field4 IS NOT Null)) Or field5 IS Null) And field6 IS NOT Null)"
//         );
//     }
//
//     #[test]
//     fn test_very_deep_nested_and_or_2() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Or(
//                 Box::new(Condition::And(
//                     Box::new(Condition::Or(
//                         Box::new(Condition::And(
//                             Box::new(Condition::Null("field1".to_owned())),
//                             Box::new(Condition::NotNull("field2".to_owned()))
//                         )),
//                         Box::new(Condition::Null("field3".to_owned()))
//                     )),
//                     Box::new(Condition::And(
//                         Box::new(Condition::NotNull("field4".to_owned())),
//                         Box::new(Condition::Null("field5".to_owned()))
//                     ))
//                 )),
//                 Box::new(Condition::And(
//                     Box::new(Condition::NotNull("field6".to_owned())),
//                     Box::new(Condition::Null("field7".to_owned()))
//                 ))
//             )).build_conditions().unwrap(),
//             "((((field1 IS Null And field2 IS NOT Null) Or field3 IS Null) And (field4 IS NOT Null And field5 IS Null)) Or (field6 IS NOT Null And field7 IS Null))"
//         );
//     }
//
//     fn test_very_deep_nested_and_or_3() {
//         let mut b = ConditionImpl::default();
//         let query = b
//             .whene(Condition::And(
//                 Box::new(Condition::Or(
//                     Box::new(Condition::And(
//                         Box::new(Condition::Or(
//                             Box::new(Condition::And(
//                                 Box::new(Condition::Null("field1".to_owned())),
//                                 Box::new(Condition::NotNull("field2".to_owned())),
//                             )),
//                             Box::new(Condition::Null("field3".to_owned())),
//                         )),
//                         Box::new(Condition::Null("field4".to_owned())),
//                     )),
//                     Box::new(Condition::NotNull("field5".to_owned())),
//                 )),
//                 Box::new(Condition::Or(
//                     Box::new(Condition::Null("field6".to_owned())),
//                     Box::new(Condition::NotNull("field7".to_owned())),
//                 )),
//             ))
//             .build_conditions()
//             .unwrap();
//
//         assert_eq!(
//             query,
//             "(((((field1 IS Null And field2 IS NOT Null) Or field3 IS Null) And field4 IS Null) Or field5 IS NOT Null) And (field6 IS Null Or field7 IS NOT Null))"
//         );
//     }
//
//     #[test]
//     fn test_very_deep_nested_and_or_4() {
//         let mut b = ConditionImpl::default();
//         let query = b
//             .whene(Condition::Or(
//                 Box::new(Condition::And(
//                     Box::new(Condition::Or(
//                         Box::new(Condition::And(
//                             Box::new(Condition::Or(
//                                 Box::new(Condition::Null("field1".to_owned())),
//                                 Box::new(Condition::NotNull("field2".to_owned())),
//                             )),
//                             Box::new(Condition::Null("field3".to_owned())),
//                         )),
//                         Box::new(Condition::And(
//                             Box::new(Condition::NotNull("field4".to_owned())),
//                             Box::new(Condition::Null("field5".to_owned())),
//                         )),
//                     )),
//                     Box::new(Condition::NotNull("field6".to_owned())),
//                 )),
//                 Box::new(Condition::And(
//                     Box::new(Condition::Or(
//                         Box::new(Condition::Null("field7".to_owned())),
//                         Box::new(Condition::NotNull("field8".to_owned())),
//                     )),
//                     Box::new(Condition::Null("field9".to_owned())),
//                 )),
//             ))
//             .build_conditions()
//             .unwrap();
//
//         assert_eq!(
//             query,
//             "(((((field1 IS Null Or field2 IS NOT Null) And field3 IS Null) Or (field4 IS NOT Null And field5 IS Null)) And field6 IS NOT Null) Or ((field7 IS Null Or field8 IS NOT Null) And field9 IS Null))"
//         );
//     }
//
//     #[test]
//     fn test_deep_nested_and_or_1() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Or(
//                     Box::new(Condition::Null("field1".to_owned())),
//                     Box::new(Condition::NotNull("field2".to_owned()))
//                 )),
//                 Box::new(Condition::And(
//                     Box::new(Condition::Null("field3".to_owned())),
//                     Box::new(Condition::NotNull("field4".to_owned()))
//                 ))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "((field1 IS Null Or field2 IS NOT Null) And (field3 IS Null And field4 IS NOT Null))"
//         );
//     }
//
//     #[test]
//     fn test_deep_nested_and_or_2() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::And(
//                     Box::new(Condition::Null("field1".to_owned())),
//                     Box::new(Condition::NotNull("field2".to_owned()))
//                 )),
//                 Box::new(Condition::Or(
//                     Box::new(Condition::Null("field3".to_owned())),
//                     Box::new(Condition::NotNull("field4".to_owned()))
//                 ))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "((field1 IS Null And field2 IS NOT Null) And (field3 IS Null Or field4 IS NOT Null))"
//         );
//     }
//
//     #[test]
//     fn test_deep_nested_and_or_3() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Or(
//                 Box::new(Condition::And(
//                     Box::new(Condition::Null("field1".to_owned())),
//                     Box::new(Condition::NotNull("field2".to_owned()))
//                 )),
//                 Box::new(Condition::And(
//                     Box::new(Condition::Null("field3".to_owned())),
//                     Box::new(Condition::NotNull("field4".to_owned()))
//                 ))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "((field1 IS Null And field2 IS NOT Null) Or (field3 IS Null And field4 IS NOT Null))"
//         );
//     }
//
//     #[test]
//     fn test_deep_nested_and_or_4() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Or(
//                 Box::new(Condition::Or(
//                     Box::new(Condition::Null("field1".to_owned())),
//                     Box::new(Condition::NotNull("field2".to_owned()))
//                 )),
//                 Box::new(Condition::And(
//                     Box::new(Condition::Null("field3".to_owned())),
//                     Box::new(Condition::NotNull("field4".to_owned()))
//                 ))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "((field1 IS Null Or field2 IS NOT Null) Or (field3 IS Null And field4 IS NOT Null))"
//         );
//     }
//
//     #[test]
//     fn test_deep_nested_and_or_5() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Or(
//                     Box::new(Condition::Null("field1".to_owned())),
//                     Box::new(Condition::And(
//                         Box::new(Condition::NotNull("field2".to_owned())),
//                         Box::new(Condition::Null("field3".to_owned()))
//                     ))
//                 )),
//                 Box::new(Condition::Null("field4".to_owned()))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "((field1 IS Null Or (field2 IS NOT Null And field3 IS Null)) And field4 IS Null)"
//         );
//     }
//
//     #[test]
//     fn test_deep_nested_and_or_6() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::And(
//                     Box::new(Condition::Or(
//                         Box::new(Condition::Null("field1".to_owned())),
//                         Box::new(Condition::NotNull("field2".to_owned()))
//                     )),
//                     Box::new(Condition::Null("field3".to_owned()))
//                 )),
//                 Box::new(Condition::NotNull("field4".to_owned()))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(((field1 IS Null Or field2 IS NOT Null) And field3 IS Null) And field4 IS NOT Null)"
//         );
//     }
//
//     #[test]
//     fn test_deep_nested_and_or_7() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Or(
//                 Box::new(Condition::And(
//                     Box::new(Condition::Or(
//                         Box::new(Condition::Null("field1".to_owned())),
//                         Box::new(Condition::NotNull("field2".to_owned()))
//                     )),
//                     Box::new(Condition::Null("field3".to_owned()))
//                 )),
//                 Box::new(Condition::And(
//                     Box::new(Condition::NotNull("field4".to_owned())),
//                     Box::new(Condition::Null("field5".to_owned()))
//                 ))
//             )).build_conditions().unwrap(),
//             "(((field1 IS Null Or field2 IS NOT Null) And field3 IS Null) Or (field4 IS NOT Null And field5 IS Null))"
//         );
//     }
//
//     #[test]
//     fn test_deep_nested_and_or_8() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Or(
//                 Box::new(Condition::And(
//                     Box::new(Condition::Or(
//                         Box::new(Condition::Null("field1".to_owned())),
//                         Box::new(Condition::NotNull("field2".to_owned()))
//                     )),
//                     Box::new(Condition::Or(
//                         Box::new(Condition::Null("field3".to_owned())),
//                         Box::new(Condition::NotNull("field4".to_owned()))
//                     ))
//                 )),
//                 Box::new(Condition::Null("field5".to_owned()))
//             )).build_conditions().unwrap(),
//             "(((field1 IS Null Or field2 IS NOT Null) And (field3 IS Null Or field4 IS NOT Null)) Or field5 IS Null)"
//         );
//     }
//
//     #[test]
//     fn test_deep_nested_and_or_9() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::And(
//                     Box::new(Condition::Or(
//                         Box::new(Condition::Null("field1".to_owned())),
//                         Box::new(Condition::NotNull("field2".to_owned()))
//                     )),
//                     Box::new(Condition::Null("field3".to_owned()))
//                 )),
//                 Box::new(Condition::Or(
//                     Box::new(Condition::NotNull("field4".to_owned())),
//                     Box::new(Condition::Null("field5".to_owned()))
//                 ))
//             )).build_conditions().unwrap(),
//             "(((field1 IS Null Or field2 IS NOT Null) And field3 IS Null) And (field4 IS NOT Null Or field5 IS Null))"
//         );
//     }
//
//     #[test]
//     fn test_deep_nested_and_or_10() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Or(
//                     Box::new(Condition::And(
//                         Box::new(Condition::Null("field1".to_owned())),
//                         Box::new(Condition::NotNull("field2".to_owned()))
//                     )),
//                     Box::new(Condition::Null("field3".to_owned()))
//                 )),
//                 Box::new(Condition::NotNull("field4".to_owned()))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(((field1 IS Null And field2 IS NOT Null) Or field3 IS Null) And field4 IS NOT Null)"
//         );
//     }
//     #[test]
//     fn test_gt_condition() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Gt("age".to_owned(), 30.to_string()))
//                 .build_conditions()
//                 .unwrap(),
//             "age > 30"
//         );
//     }
//
//     #[test]
//     fn test_gte_condition() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Gte("age".to_owned(), 30.to_string()))
//                 .build_conditions()
//                 .unwrap(),
//             "age >= 30"
//         );
//     }
//
//     #[test]
//     fn test_lt_condition() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Lt("age".to_owned(), 30.to_string()))
//                 .build_conditions()
//                 .unwrap(),
//             "age < 30"
//         );
//     }
//
//     #[test]
//     fn test_le_condition() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Lte("age".to_owned(), 30.to_string()))
//                 .build_conditions()
//                 .unwrap(),
//             "age <= 30"
//         );
//     }
//
//     #[test]
//     fn test_native_condition() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Native("age Between 20 And 30".to_owned()))
//                 .build_conditions()
//                 .unwrap(),
//             "age Between 20 And 30"
//         );
//     }
//
//     #[test]
//     fn test_combined_and_or() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Null("anas".to_owned())),
//                 Box::new(Condition::Or(
//                     Box::new(Condition::NotNull("jaidi".to_owned())),
//                     Box::new(Condition::In(
//                         "id".to_owned(),
//                         vec![1.to_string(), 2.to_string(), 3.to_string()]
//                     ))
//                 ))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(anas IS Null And (jaidi IS NOT Null Or id In (1, 2, 3)))"
//         );
//     }
//
//     #[test]
//     fn test_nested_and_or() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Or(
//                     Box::new(Condition::Null("anas".to_owned())),
//                     Box::new(Condition::NotNull("jaidi".to_owned()))
//                 )),
//                 Box::new(Condition::In(
//                     "id".to_owned(),
//                     vec![1.to_string(), 2.to_string(), 3.to_string()]
//                 ))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "((anas IS Null Or jaidi IS NOT Null) And id In (1, 2, 3))"
//         );
//     }
//
//     #[test]
//     fn test_combined_eq_and_gt() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Eq("name".to_owned(), 1.to_string())),
//                 Box::new(Condition::Gt("age".to_owned(), 18.to_string()))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(name = 1 And age > 18)"
//         );
//     }
//
//     #[test]
//     fn test_combined_eq_and_lt() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Eq("name".to_owned(), 1.to_string())),
//                 Box::new(Condition::Lt("age".to_owned(), 18.to_string()))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(name = 1 And age < 18)"
//         );
//     }
//
//     #[test]
//     fn test_combined_eq_and_le() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Eq("name".to_owned(), 1.to_string())),
//                 Box::new(Condition::Lte("age".to_owned(), 18.to_string()))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(name = 1 And age <= 18)"
//         );
//     }
//
//     #[test]
//     fn test_combined_eq_and_gte() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Eq("name".to_owned(), 1.to_string())),
//                 Box::new(Condition::Gte("age".to_owned(), 18.to_string()))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(name = 1 And age >= 18)"
//         );
//     }
//
//     #[test]
//     fn test_combined_like_and_between() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Like("name".to_owned(), "John%".to_owned())),
//                 Box::new(Condition::Between(
//                     "age".to_owned(),
//                     20.to_string(),
//                     30.to_string()
//                 ))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(name Like 'John%' And age Between 20 And 30)"
//         );
//     }
//
//     #[test]
//     fn test_combined_notin_and_null() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::NotIn(
//                     "id".to_owned(),
//                     vec![1.to_string(), 2.to_string(), 3.to_string()]
//                 )),
//                 Box::new(Condition::Null("name".to_owned()))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(id NOT In (1, 2, 3) And name IS Null)"
//         );
//     }
//
//     #[test]
//     fn test_combined_in_and_notnull() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::In(
//                     "id".to_owned(),
//                     vec![4.to_string(), 5.to_string(), 6.to_string()]
//                 )),
//                 Box::new(Condition::NotNull("name".to_owned()))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(id In (4, 5, 6) And name IS NOT Null)"
//         );
//     }
//
//     #[test]
//     fn test_combined_ne_and_gt() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Neq("id".to_owned(), 1.to_string())),
//                 Box::new(Condition::Gt("age".to_owned(), 20.to_string()))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(id != 1 And age > 20)"
//         );
//     }
//
//     #[test]
//     fn test_combined_between_and_in() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Between(
//                     "age".to_owned(),
//                     20.to_string(),
//                     30.to_string()
//                 )),
//                 Box::new(Condition::In(
//                     "id".to_owned(),
//                     vec![4.to_string(), 5.to_string(), 6.to_string()]
//                 ))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(age Between 20 And 30 And id In (4, 5, 6))"
//         );
//     }
//
//     #[test]
//     fn test_combined_like_and_in() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Like("name".to_owned(), "John%".to_owned())),
//                 Box::new(Condition::In(
//                     "id".to_owned(),
//                     vec![1.to_string(), 2.to_string(), 3.to_string()]
//                 ))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(name Like 'John%' And id In (1, 2, 3))"
//         );
//     }
//     #[test]
//     fn test_basic_null() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Null("anas".to_owned()))
//                 .build_conditions()
//                 .unwrap(),
//             "anas IS Null"
//         );
//     }
//
//     #[test]
//     fn test_basic_notnull() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::NotNull("jaidi".to_string()))
//                 .build_conditions()
//                 .unwrap(),
//             "jaidi IS NOT Null"
//         );
//     }
//
//     #[test]
//     fn test_and_condition() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::And(
//                 Box::new(Condition::Null("robin".to_owned())),
//                 Box::new(Condition::Null("hood".to_owned()))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(robin IS Null And hood IS Null)"
//         );
//     }
//
//     #[test]
//     fn test_or_condition() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Or(
//                 Box::new(Condition::Null("robin".to_owned())),
//                 Box::new(Condition::Null("hood".to_owned()))
//             ))
//             .build_conditions()
//             .unwrap(),
//             "(robin IS Null Or hood IS Null)"
//         );
//     }
//
//     #[test]
//     fn test_in_condition() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::In(
//                 "id".to_owned(),
//                 vec![1.to_string(), 2.to_string(), 3.to_string()]
//             ))
//             .build_conditions()
//             .unwrap(),
//             "id In (1, 2, 3)"
//         );
//     }
//
//     #[test]
//     fn test_notin_condition() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::NotIn(
//                 "id".to_owned(),
//                 vec![4.to_string(), 5.to_string(), 6.to_string()]
//             ))
//             .build_conditions()
//             .unwrap(),
//             "id NOT In (4, 5, 6)"
//         );
//     }
//
//     #[test]
//     fn test_eq_condition() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Eq("id".to_owned(), 1.to_string()))
//                 .build_conditions()
//                 .unwrap(),
//             "id = 1"
//         );
//     }
//
//     #[test]
//     fn test_neq_condition() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Neq("id".to_owned(), 1.to_string()))
//                 .build_conditions()
//                 .unwrap(),
//             "id != 1"
//         );
//     }
//
//     #[test]
//     fn test_like_condition() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Like("name".to_owned(), "John%".to_owned()))
//                 .build_conditions()
//                 .unwrap(),
//             "name Like 'John%'"
//         );
//     }
//
//     #[test]
//     fn test_between_condition() {
//         let mut b = ConditionImpl::default();
//         assert_eq!(
//             b.whene(Condition::Between(
//                 "id".to_owned(),
//                 1.to_string(),
//                 10.to_string()
//             ))
//             .build_conditions()
//             .unwrap(),
//             "id Between 1 And 10"
//         );
//     }
// }
