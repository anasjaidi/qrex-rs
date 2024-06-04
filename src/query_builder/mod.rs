use self::{insert_action_builder::InsertActionBuilder, where_action_builder::QueryActionBuilder};

mod insert_action_builder;
mod query;
mod where_action_builder;

#[derive(Default)]
pub struct QueryBuilder;

impl QueryBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn select(&self) -> QueryActionBuilder<u32> {
        QueryActionBuilder {
            conditions: todo!(),
            table: String::new(),
            all: false,
            fields: vec![],
        }
    }
}
