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

use super::{
    condition::{self, Condition},
    group_by::GroupBy,
    join::Join,
    order_by::OrderBy,
};

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

pub trait ClonableString: DynClone + ToString {}

impl<T> ClonableString for T where T: Clone + ToString {}

pub trait Select: GroupBy + OrderBy + Join {
    fn set_fields(&mut self, fields: impl Fn(&mut Vec<(String, String)>));

    fn get_fields(&self) -> Vec<(&str, &str)>;

    fn get_condition(&self) -> Option<Condition>;

    fn set_condition(&mut self, condition: Condition);

    fn set_table(&mut self, table: &str);

    fn get_table(&self) -> String;

    fn table(&mut self, table: &str) -> &mut Self {
        self.set_table(table);
        self
    }

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

    fn r#where(&mut self, condition: Condition) {
        self.set_condition(condition);
    }

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
