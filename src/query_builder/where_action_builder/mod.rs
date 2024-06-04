use std::fmt::{Debug, Display};

#[derive(Clone)]
pub struct QueryActionBuilder<T>
where
    T: Sized + Clone,
{
    pub table: String,
    pub fields: Vec<String>,
    pub conditions: Option<WhereCondition<T>>,
    pub all: bool,
}

impl<T> Default for QueryActionBuilder<T>
where
    T: Sized + Clone,
{
    fn default() -> Self {
        Self {
            table: String::new(),
            all: false,
            conditions: None,
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum WhereCondition<T>
where
    T: Sized + Clone,
{
    NATIVE(String),
    NULL(String),
    NOTNULL(String),
    AND(Box<Self>, Box<Self>),
    OR(Box<Self>, Box<Self>),
    IN(String, Vec<T>),
    NOTIN(String, Vec<T>),
    EQ(String, T),
    NEQ(String, T),
    BETWEEN(String, T, T),
    GT(String, T),
    GTE(String, T),
    LT(String, T),
    LE(String, T),
    LIKE(String, String),
}

// impl<T> Clone for WhereCondition<T>
// where
//     T: Clone,
// {
//     fn clone(&self) -> Self {
//         match self {
//             WhereCondition::NATIVE(s) => WhereCondition::NATIVE(s.clone()),
//             WhereCondition::NULL(s) => WhereCondition::NULL(s.clone()),
//             WhereCondition::NOTNULL(s) => WhereCondition::NOTNULL(s.clone()),
//             WhereCondition::AND(box1, box2) => WhereCondition::AND(box1.clone(), box2.clone()),
//             WhereCondition::OR(box1, box2) => WhereCondition::OR(box1.clone(), box2.clone()),
//             WhereCondition::IN(s, vec) => WhereCondition::IN(s.clone(), vec.clone()),
//             WhereCondition::NOTIN(s, vec) => WhereCondition::NOTIN(s.clone(), vec.clone()),
//             WhereCondition::EQ(s, t) => WhereCondition::EQ(s.clone(), t.clone()),
//             WhereCondition::NEQ(s, t) => WhereCondition::NEQ(s.clone(), t.clone()),
//             WhereCondition::BETWEEN(s, t1, t2) => {
//                 WhereCondition::BETWEEN(s.clone(), t1.clone(), t2.clone())
//             }
//             WhereCondition::GT(s, t) => WhereCondition::GT(s.clone(), t.clone()),
//             WhereCondition::GTE(s, t) => WhereCondition::GTE(s.clone(), t.clone()),
//             WhereCondition::LT(s, t) => WhereCondition::LT(s.clone(), t.clone()),
//             WhereCondition::LE(s, t) => WhereCondition::LE(s.clone(), t.clone()),
//             WhereCondition::LIKE(s1, s2) => WhereCondition::LIKE(s1.clone(), s2.clone()),
//         }
//     }
// }
impl<T> QueryActionBuilder<T>
where
    T: Sized + Clone,
{
    // TODO: fix this function is just a boilerplate for build function
    pub fn dummy_build(&self) -> String
    where
        T: Clone + Sized + Display + Debug,
    {
        fn gc<T>(c: &WhereCondition<T>) -> String
        where
            T: Clone + Sized + Display + Debug,
        {
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
                WhereCondition::LE(f, d) => format!("{} <= {}", f, d),
                WhereCondition::GT(f, d) => format!("{} > {}", f, d),
                WhereCondition::GTE(f, d) => format!("{} >= {}", f, d),
                WhereCondition::LIKE(f, d) => format!("{} LIKE '{}'", f, d),
                WhereCondition::BETWEEN(f, a, b) => format!("{} BETWEEN {} AND {}", f, a, b),
                WhereCondition::NATIVE(f) => f.clone(),
            }
        }

        if let Some(conditions) = &self.conditions {
            gc(conditions)
        } else {
            String::new()
        }
    }
    pub fn new() -> Self {
        Self::default()
    }
    //
    // pub fn fields(&mut self, fields: &[&str]) -> &Self {
    //     if self.all {
    //         // TODO: throw error
    //     }
    //
    //     self.fields = fields
    //         .iter()
    //         .map(|f| f.to_string())
    //         .collect::<Vec<String>>();
    //     self
    // }
    //
    // pub fn from_table(&mut self, table: impl Into<String>) -> &Self {
    //     self.table = table.into();
    //     self
    // }

    pub fn or_where(&mut self, condition: WhereCondition<T>) -> &Self {
        if self.conditions.is_none() {
            // TODO: handle error
        }
        self.conditions = Some(WhereCondition::OR(
            Box::new(self.conditions.as_ref().unwrap().clone()),
            Box::new(condition),
        ));
        self
    }

    pub fn whene(&mut self, condition: WhereCondition<T>) -> &mut Self {
        match &self.conditions {
            None => {
                self.conditions = Some(condition);
                self
            }
            Some(c) => {
                self.conditions = Some(WhereCondition::AND(
                    Box::new(c.clone()),
                    Box::new(condition),
                ));
                self
            }
        }
    }

    // pub fn all_fields(&mut self) -> &Self {
    //     if self.fields.len() > 0 {
    //         // TODO: throw error
    //     };
    //     self.all = true;
    //     self
    // }
}

#[cfg(test)]
mod test {
    use super::QueryActionBuilder;
    use crate::query_builder::where_action_builder::WhereCondition;

    #[test]
    fn test_very_deep_nested_and_or_1() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            )).dummy_build(),
            "((((field1 IS NULL OR field2 IS NOT NULL) AND (field3 IS NULL AND field4 IS NOT NULL)) OR field5 IS NULL) AND field6 IS NOT NULL)"
        );
    }

    #[test]
    fn test_very_deep_nested_and_or_2() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            )).dummy_build(),
            "((((field1 IS NULL AND field2 IS NOT NULL) OR field3 IS NULL) AND (field4 IS NOT NULL AND field5 IS NULL)) OR (field6 IS NOT NULL AND field7 IS NULL))"
        );
    }

    fn test_very_deep_nested_and_or_3() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            .dummy_build();

        assert_eq!(
            query,
            "(((((field1 IS NULL AND field2 IS NOT NULL) OR field3 IS NULL) AND field4 IS NULL) OR field5 IS NOT NULL) AND (field6 IS NULL OR field7 IS NOT NULL))"
        );
    }

    #[test]
    fn test_very_deep_nested_and_or_4() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            .dummy_build();

        assert_eq!(
            query,
            "(((((field1 IS NULL OR field2 IS NOT NULL) AND field3 IS NULL) OR (field4 IS NOT NULL AND field5 IS NULL)) AND field6 IS NOT NULL) OR ((field7 IS NULL OR field8 IS NOT NULL) AND field9 IS NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_1() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            .dummy_build(),
            "((field1 IS NULL OR field2 IS NOT NULL) AND (field3 IS NULL AND field4 IS NOT NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_2() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            .dummy_build(),
            "((field1 IS NULL AND field2 IS NOT NULL) AND (field3 IS NULL OR field4 IS NOT NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_3() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            .dummy_build(),
            "((field1 IS NULL AND field2 IS NOT NULL) OR (field3 IS NULL AND field4 IS NOT NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_4() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            .dummy_build(),
            "((field1 IS NULL OR field2 IS NOT NULL) OR (field3 IS NULL AND field4 IS NOT NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_5() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            .dummy_build(),
            "((field1 IS NULL OR (field2 IS NOT NULL AND field3 IS NULL)) AND field4 IS NULL)"
        );
    }

    #[test]
    fn test_deep_nested_and_or_6() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            .dummy_build(),
            "(((field1 IS NULL OR field2 IS NOT NULL) AND field3 IS NULL) AND field4 IS NOT NULL)"
        );
    }

    #[test]
    fn test_deep_nested_and_or_7() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            )).dummy_build(),
            "(((field1 IS NULL OR field2 IS NOT NULL) AND field3 IS NULL) OR (field4 IS NOT NULL AND field5 IS NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_8() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            )).dummy_build(),
            "(((field1 IS NULL OR field2 IS NOT NULL) AND (field3 IS NULL OR field4 IS NOT NULL)) OR field5 IS NULL)"
        );
    }

    #[test]
    fn test_deep_nested_and_or_9() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            )).dummy_build(),
            "(((field1 IS NULL OR field2 IS NOT NULL) AND field3 IS NULL) AND (field4 IS NOT NULL OR field5 IS NULL))"
        );
    }

    #[test]
    fn test_deep_nested_and_or_10() {
        let mut b = QueryActionBuilder::<u32>::default();
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
            .dummy_build(),
            "(((field1 IS NULL AND field2 IS NOT NULL) OR field3 IS NULL) AND field4 IS NOT NULL)"
        );
    }
    #[test]
    fn test_gt_condition() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::GT("age".to_owned(), 30))
                .dummy_build(),
            "age > 30"
        );
    }

    #[test]
    fn test_gte_condition() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::GTE("age".to_owned(), 30))
                .dummy_build(),
            "age >= 30"
        );
    }

    #[test]
    fn test_lt_condition() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::LT("age".to_owned(), 30))
                .dummy_build(),
            "age < 30"
        );
    }

    #[test]
    fn test_le_condition() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::LE("age".to_owned(), 30))
                .dummy_build(),
            "age <= 30"
        );
    }

    #[test]
    fn test_native_condition() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::NATIVE("age BETWEEN 20 AND 30".to_owned()))
                .dummy_build(),
            "age BETWEEN 20 AND 30"
        );
    }

    #[test]
    fn test_multiple_conditions() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::NULL("anas".to_owned()))
                .whene(WhereCondition::NOTNULL("jaidi".to_owned()))
                .dummy_build(),
            "(anas IS NULL AND jaidi IS NOT NULL)"
        );
    }

    #[test]
    fn test_combined_and_or() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::NULL("anas".to_owned())),
                Box::new(WhereCondition::OR(
                    Box::new(WhereCondition::NOTNULL("jaidi".to_owned())),
                    Box::new(WhereCondition::IN("id".to_owned(), vec![1, 2, 3]))
                ))
            ))
            .dummy_build(),
            "(anas IS NULL AND (jaidi IS NOT NULL OR id IN (1, 2, 3)))"
        );
    }

    #[test]
    fn test_nested_and_or() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::OR(
                    Box::new(WhereCondition::NULL("anas".to_owned())),
                    Box::new(WhereCondition::NOTNULL("jaidi".to_owned()))
                )),
                Box::new(WhereCondition::IN("id".to_owned(), vec![1, 2, 3]))
            ))
            .dummy_build(),
            "((anas IS NULL OR jaidi IS NOT NULL) AND id IN (1, 2, 3))"
        );
    }

    #[test]
    fn test_combined_eq_and_gt() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::EQ("name".to_owned(), 1)),
                Box::new(WhereCondition::GT("age".to_owned(), 18))
            ))
            .dummy_build(),
            "(name = 1 AND age > 18)"
        );
    }

    #[test]
    fn test_combined_eq_and_lt() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::EQ("name".to_owned(), 1)),
                Box::new(WhereCondition::LT("age".to_owned(), 18))
            ))
            .dummy_build(),
            "(name = 1 AND age < 18)"
        );
    }

    #[test]
    fn test_combined_eq_and_le() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::EQ("name".to_owned(), 1)),
                Box::new(WhereCondition::LE("age".to_owned(), 18))
            ))
            .dummy_build(),
            "(name = 1 AND age <= 18)"
        );
    }

    #[test]
    fn test_combined_eq_and_gte() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::EQ("name".to_owned(), 1)),
                Box::new(WhereCondition::GTE("age".to_owned(), 18))
            ))
            .dummy_build(),
            "(name = 1 AND age >= 18)"
        );
    }

    #[test]
    fn test_combined_like_and_between() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::LIKE("name".to_owned(), "John%".to_owned())),
                Box::new(WhereCondition::BETWEEN("age".to_owned(), 20, 30))
            ))
            .dummy_build(),
            "(name LIKE 'John%' AND age BETWEEN 20 AND 30)"
        );
    }

    #[test]
    fn test_combined_notin_and_null() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::NOTIN("id".to_owned(), vec![1, 2, 3])),
                Box::new(WhereCondition::NULL("name".to_owned()))
            ))
            .dummy_build(),
            "(id NOT IN (1, 2, 3) AND name IS NULL)"
        );
    }

    #[test]
    fn test_combined_in_and_notnull() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::IN("id".to_owned(), vec![4, 5, 6])),
                Box::new(WhereCondition::NOTNULL("name".to_owned()))
            ))
            .dummy_build(),
            "(id IN (4, 5, 6) AND name IS NOT NULL)"
        );
    }

    #[test]
    fn test_combined_ne_and_gt() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::NEQ("id".to_owned(), 1)),
                Box::new(WhereCondition::GT("age".to_owned(), 20))
            ))
            .dummy_build(),
            "(id != 1 AND age > 20)"
        );
    }

    #[test]
    fn test_combined_between_and_in() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::BETWEEN("age".to_owned(), 20, 30)),
                Box::new(WhereCondition::IN("id".to_owned(), vec![4, 5, 6]))
            ))
            .dummy_build(),
            "(age BETWEEN 20 AND 30 AND id IN (4, 5, 6))"
        );
    }

    #[test]
    fn test_combined_like_and_in() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::LIKE("name".to_owned(), "John%".to_owned())),
                Box::new(WhereCondition::IN("id".to_owned(), vec![1, 2, 3]))
            ))
            .dummy_build(),
            "(name LIKE 'John%' AND id IN (1, 2, 3))"
        );
    }
    #[test]
    fn test_basic_null() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::NULL("anas".to_owned()))
                .dummy_build(),
            "anas IS NULL"
        );
    }

    #[test]
    fn test_basic_notnull() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::NOTNULL("jaidi".to_string()))
                .dummy_build(),
            "jaidi IS NOT NULL"
        );
    }

    #[test]
    fn test_and_condition() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::AND(
                Box::new(WhereCondition::NULL("robin".to_owned())),
                Box::new(WhereCondition::NULL("hood".to_owned()))
            ))
            .dummy_build(),
            "(robin IS NULL AND hood IS NULL)"
        );
    }

    #[test]
    fn test_or_condition() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::OR(
                Box::new(WhereCondition::NULL("robin".to_owned())),
                Box::new(WhereCondition::NULL("hood".to_owned()))
            ))
            .dummy_build(),
            "(robin IS NULL OR hood IS NULL)"
        );
    }

    #[test]
    fn test_in_condition() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::IN("id".to_owned(), vec![1, 2, 3]))
                .dummy_build(),
            "id IN (1, 2, 3)"
        );
    }

    #[test]
    fn test_notin_condition() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::NOTIN("id".to_owned(), vec![4, 5, 6]))
                .dummy_build(),
            "id NOT IN (4, 5, 6)"
        );
    }

    #[test]
    fn test_eq_condition() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::EQ("id".to_owned(), 1))
                .dummy_build(),
            "id = 1"
        );
    }

    #[test]
    fn test_neq_condition() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::NEQ("id".to_owned(), 1))
                .dummy_build(),
            "id != 1"
        );
    }

    #[test]
    fn test_like_condition() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::LIKE("name".to_owned(), "John%".to_owned()))
                .dummy_build(),
            "name LIKE 'John%'"
        );
    }

    #[test]
    fn test_between_condition() {
        let mut b = QueryActionBuilder::<u32>::default();
        assert_eq!(
            b.whene(WhereCondition::BETWEEN("id".to_owned(), 1, 10))
                .dummy_build(),
            "id BETWEEN 1 AND 10"
        );
    }
}
