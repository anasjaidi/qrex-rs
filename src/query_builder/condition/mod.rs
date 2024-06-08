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
    pub fn build_conditions(&self) -> Option<String> {
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
