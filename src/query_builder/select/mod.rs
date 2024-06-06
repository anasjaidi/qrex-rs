// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   mod.rs                                             :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: ajaidi <ajaidi@student.42.fr>              +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2024/06/06 11:44:18 by ajaidi            #+#    #+#             //
//   Updated: 2024/06/06 14:27:03 by ajaidi           ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use dyn_clone::DynClone;

use super::condition::Condition;

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

impl ToString for Agregate {
    fn to_string(&self) -> String {
        match self {
            Agregate::Sum(column) => format!("SUM({})", column),
            Agregate::Count(column) => format!("COUNT({})", column),
            Agregate::Max(column) => format!("MAX({})", column),
            Agregate::Min(column) => format!("MIN({})", column),
            Agregate::Avg(column) => format!("AVG({})", column),
            Agregate::First(column) => format!("FIRST({})", column),
            Agregate::Last(column) => format!("LAST({})", column),
            Agregate::StdDev(column) => format!("STDDEV({})", column),
            Agregate::Var(column) => format!("VAR({})", column),
            Agregate::StringAgg(column, separator) => {
                format!("STRING_AGG({}, '{}')", column, separator)
            }
        }
    }
}

pub trait ClonableString: DynClone + ToString {}

impl<T> ClonableString for T where T: Clone + ToString {}

pub trait Select: Condition {
    fn set_fields(&mut self, fields: impl Fn(&mut Vec<(String, String)>));

    fn get_fields(&self) -> &[(&str, &str)];

    fn set_table(&mut self, table: &str);

    fn get_table(&self) -> String;

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

        let conditions = self.build_conditions()?;

        Some(format!(
            "SELECT {} FROM {} WHERE {}",
            fields,
            self.get_table(),
            conditions
        ))
    }

    fn select_all_fields(&mut self) {
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
    }

    fn select_alias_field(&mut self, field: (Box<dyn ClonableString>, &str)) -> &Self {
        self.set_fields(|f| {
            (*f).push((
                dyn_clone::clone_box(&*field.0).to_string(),
                field.1.to_owned(),
            ))
        });
        self
    }

    fn alias(&mut self, alias: &str) {
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
        })
    }

    fn select_fields(&mut self, fields: &[&str]) {
        for &f in fields.iter() {
            self.select_alias_field((Box::new(f.to_owned()), f));
        }
    }

    fn select_fields_str(&mut self, raw: &str) {
        todo!();
    }
}
