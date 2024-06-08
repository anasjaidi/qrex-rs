pub enum JoinType {
    Inner,
    Outer,
    Left,
    Right,
}

pub struct JoinEntry {
    table: String,
    join_type: JoinType,
    on: Option<()>,
}

// rust is the most diffuclt

pub trait Join {
    // fn inner_join(&self, table_to_join: &str) {
    //     self.set_join(|f| {
    //         f.push(JoinEntry {
    //             table: table_to_join.to_owned(),
    //             join_type: JoinType::Inner,
    //             on: None, // TODO: me later
    //         })
    //     })
    // }
    //
    // fn outer_join(&self, table_to_join: &str) {
    //     self.set_join(|f| {
    //         f.push(JoinEntry {
    //             table: table_to_join.to_owned(),
    //             join_type: JoinType::Outer,
    //             on: None, // TODO: me later
    //         })
    //     })
    // }
    //
    // fn left_join(&self, table_to_join: &str) {
    //     self.set_join(|f| {
    //         f.push(JoinEntry {
    //             table: table_to_join.to_owned(),
    //             join_type: JoinType::Left,
    //             on: None, // TODO: me later
    //         })
    //     })
    // }
    //
    // fn right_join(&self, table_to_join: &str) {
    //     self.set_join(|f| {
    //         f.push(JoinEntry {
    //             table: table_to_join.to_owned(),
    //             join_type: JoinType::Right,
    //             on: None, // TODO: me later
    //         })
    //     })
    // }
    //
    // fn on(&mut self, condition: WhereCondition) -> &Self {
    //     self.whene(condition)
    // }
    //
    // fn or_on(&mut self, condition: WhereCondition) -> &Self {
    //     self.or_where(condition)
    // }
    //
    fn build_join() -> String {
        format!("select _ from _ _ join _ on _ where _ ")
    }
}
