mod condition;
mod query;
mod select;
use super::query_builder::condition::{Condition, WhereCondition};

#[derive(Clone, Default)]
pub struct QueryActionBuilder {
    pub table: String,
    pub fields: Vec<String>,
    pub conditions: Option<WhereCondition>,
    pub all: bool,
}
impl Condition for QueryActionBuilder {
    fn set_condition(&mut self, condition: WhereCondition) {
        self.conditions = Some(condition);
    }
    fn get_condition(&self) -> Option<&WhereCondition> {
        self.conditions.as_ref()
    }
}

impl QueryActionBuilder {
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
    // pub fn build(&self) -> Result<String, ()> {
    //     let fields_invalid =
    //         (self.all && !self.fields.is_empty()) || (!self.all || self.fields.is_empty());
    //     let table_name_undifned = self.table.is_empty();
    //     let condition_empty = self.conditions.is_none();
    //
    //     if fields_invalid || table_name_undifned {
    //         return Err(());
    //     }
    //     let fields = if self.all {
    //         "*".to_owned()
    //     } else {
    //         self.fields.join(", ")
    //     };
    //
    //     let table: &str = &self.table;
    //     let conditions = if condition_empty {
    //         String::new()
    //     } else {
    //         self.build_conditions()
    //     };
    //     Ok(format!("select {} from {} {}", fields, table, conditions))
    // }
    // pub fn all_fields(&mut self) -> &Self {
    //     self.all = true;
    //     self
    // }
}
#[derive(Default)]
pub struct QueryBuilder;

impl QueryBuilder {
    pub fn new() -> Self {
        Self
    }
}
