// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   mod.rs                                             :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: ajaidi <ajaidi@student.42.fr>              +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2024/06/06 11:44:18 by ajaidi            #+#    #+#             //
//   Updated: 2024/06/06 19:49:41 by ajaidi           ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

#![allow(unused)]
use std::fmt;

use dyn_clone::DynClone;

use super::condition::{Condition, Having};

#[derive(Clone, Debug)]
pub enum Agregate {
    Sum(String),
    Count(String),
    Max(String),
    Min(String),
    Avg(String),
    First(String),
    Last(String),
    StdDev(String),
    Var(String),
    StringAgg(String, String), // Takes column and separator as arguments
}
impl fmt::Display for Agregate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Agregate::Sum(column) => write!(f, "SUM({})", column),
            Agregate::Count(column) => write!(f, "COUNT({})", column),
            Agregate::Max(column) => write!(f, "MAX({})", column),
            Agregate::Min(column) => write!(f, "MIN({})", column),
            Agregate::Avg(column) => write!(f, "AVG({})", column),
            Agregate::First(column) => write!(f, "FIRST({})", column),
            Agregate::Last(column) => write!(f, "LAST({})", column),
            Agregate::StdDev(column) => write!(f, "STDDEV({})", column),
            Agregate::Var(column) => write!(f, "VAR({})", column),
            Agregate::StringAgg(column, separator) => {
                write!(f, "STRING_AGG({}, '{}')", column, separator)
            }
        }
    }
}

pub enum OrderDirection {
    Asc,
    Desc,
}

pub enum OrderByClauses {
    Field(String),
    Fields(Vec<String>),
    Alias(String),
    Aliases(Vec<String>),
    Expression(String),
    Agregate(String, String),
    Having(Condition),
}

pub trait ClonableString: DynClone + ToString {}

impl<T> ClonableString for T where T: Clone + ToString {}

pub trait Select {
    fn set_fields(&mut self, fields: impl Fn(&mut Vec<(String, String)>));

    fn get_fields(&self) -> Vec<(&str, &str)>;

    fn set_table(&mut self, table: &str);

    fn get_table(&self) -> String;

    fn with_table(&mut self, table: &str) -> &mut Self {
        self.set_table(table);
        self
    }

    // fn get_group(&self) -> &[&str];
    //
    // fn set_group(&mut self);
    //
    // fn get_order(&self) -> &[&OrderByClauses];
    //
    // fn set_order(&mut self, fields: &[&OrderByClauses]);
    //
    fn build_select(&self) -> Option<String> {
        let fields_vec = self.get_fields();

        if fields_vec.is_empty() {
            return None;
        }

        let fields = if fields_vec[0].0 == "*" {
            String::from("*")
        } else {
            fields_vec
                .iter()
                .map(|f| format!("{} as {}", f.0, f.1,))
                .collect::<Vec<String>>()
                .join(", ")
        };

        let conditions = ""; //self.build_conditions()?;

        Some(format!(
            "SELECT {} FROM {} WHERE {}",
            fields,
            self.get_table(),
            conditions
        ))
    }

    // fn group_by(&mut self) {
    //     todo!()
    // }
    //
    // fn order_by(&mut self) {
    //     todo!()
    // }
    //
    fn select_all_fields(&mut self) -> &Self {
        let fields = self.get_fields();
        let mut exist = false;
        if !fields.is_empty() && fields[0].0 == "#**#" {
            exist = true;
        }

        self.set_fields(|f| {
            if exist {
                let item = f.get_mut(0).unwrap();
                item.0 = "*".to_string();
            } else {
                f.insert(0, (String::from("*"), "".to_string()))
            }
        });
        self.set_fields(|f| *f = vec![("*".to_string(), String::new())]);
        self
    }

    fn select_alias_field(&mut self, field: impl ClonableString, alias: &str) -> &Self {
        self.set_fields(|f| {
            (*f).push((dyn_clone::clone_box(&field).to_string(), alias.to_owned()))
        });
        self
    }

    fn alias(&mut self, alias: &str) -> &Self {
        let fields = self.get_fields();
        let mut exist = false;
        if !fields.is_empty() && fields[0].0 == "*" {
            exist = true;
        }

        self.set_fields(|f| {
            if exist {
                let item = f.get_mut(0).unwrap();
                item.1 = alias.to_string();
            } else {
                f.insert(0, (String::from("#**#"), alias.to_string()))
            }
        });
        self
    }

    fn select_fields(&mut self, fields: &[&str]) -> &mut Self {
        for &f in fields.iter() {
            self.select_alias_field(f, f);
        }
        self
    }
    //
    // #[allow(unused)]
    // fn select_fields_str(&mut self, raw: &str) {
    //     todo!();
    //
    //     let raw_query = raw.to_owned();
    //
    //     // let mut fields: Vec<_> = vec![];
    //
    //     //fields.push("");
    // }
}

pub enum Order {
    Asc,
    Desc,
}

pub trait OrderBy {
    fn get_order(&self) -> &[(&str, &Order)];
    fn set_order(&mut self, group: Vec<(String, Order)>);

    fn raw_order(&mut self, raw: &str) -> &mut Self;

    fn order_by_expression(&mut self, exp: &str) -> &mut Self {
        // TODO: IMPL ME LATER
        self
    }

    // fn order_by_row(&mut self, row: u32, order: Order) -> &mut Self {
    //     self.set_order(vec![(row.to_string(), Order::Asc)]);
    //     self
    // }
    //
    fn order_by_row_asc(&mut self, row: u32) -> &mut Self {
        self.set_order(vec![(row.to_string(), Order::Asc)]);
        self
    }

    fn order_by_row(&mut self, row: u32, order: Order) -> &mut Self {
        self.set_order(vec![(row.to_string(), order)]);
        self
    }

    fn order_by_row_desc(&mut self, row: u32) -> &mut Self {
        self.set_order(vec![(row.to_string(), Order::Desc)]);
        self
    }

    fn order_by_field_asc(&mut self, field: &str) -> &mut Self {
        self.set_order(vec![(field.to_owned(), Order::Asc)]);
        self
    }

    fn order_by_field_desc(&mut self, field: &str) -> &mut Self {
        self.set_order(vec![(field.to_owned(), Order::Desc)]);
        self
    }

    fn order_by_field(&mut self, field: &str, order: Order) -> &mut Self {
        self.set_order(vec![(field.to_owned(), order)]);
        self
    }

    fn order_by_fields(&mut self, fields: Vec<(String, Order)>) -> &mut Self {
        self.set_order(fields);
        self
    }
}

pub trait GroupBy {
    fn get_group(&self) -> &[&str];
    fn set_group(&mut self, group: Vec<String>);

    fn group_by_field(&mut self, field: &str) -> &mut Self {
        self.set_group(vec![field.to_owned()]);
        self
    }

    fn group_by_fields(&mut self, fields: Vec<String>) -> &mut Self {
        self.set_group(fields);
        self
    }
}
