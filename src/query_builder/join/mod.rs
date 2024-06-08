use super::condition::Condition;

pub enum JoinType {
    Inner,
    Outer,
    Left,
    Right,
}

pub struct JoinEntry {
    table: String,
    join_type: JoinType,
    on: Option<Condition>,
}

// rust is the most diffuclt

pub trait Join {
    fn set_join(&mut self, f: impl FnOnce(&mut Vec<JoinEntry>));
    fn get_join(&self) -> Vec<&JoinEntry>;

    fn inner_join(&mut self, table_to_join: &str, condition: Option<Condition>) {
        self.set_join(|f| {
            f.push(JoinEntry {
                table: table_to_join.to_owned(),
                join_type: JoinType::Inner,
                on: condition, // TODO: me later
            })
        })
    }

    fn outer_join(&mut self, table_to_join: &str, condition: Option<Condition>) {
        self.set_join(|f| {
            f.push(JoinEntry {
                table: table_to_join.to_owned(),
                join_type: JoinType::Outer,
                on: condition, // TODO: me later
            })
        })
    }

    fn left_join(&mut self, table_to_join: &str, condition: Option<Condition>) {
        self.set_join(|f| {
            f.push(JoinEntry {
                table: table_to_join.to_owned(),
                join_type: JoinType::Left,
                on: condition, // TODO: me later
            })
        })
    }

    fn right_join(&mut self, table_to_join: &str, condition: Option<Condition>) {
        self.set_join(|f| {
            f.push(JoinEntry {
                table: table_to_join.to_owned(),
                join_type: JoinType::Right,
                on: condition, // TODO: me later
            })
        })
    }

    fn build_join(&self) -> Option<String> {
        let joins = self.get_join();
        if joins.is_empty() {
            return None;
        }

        let join_strings = joins
            .iter()
            .map(|f| match f.join_type {
                JoinType::Left => format!(
                    "LEFT JOIN {} {}",
                    f.table,
                    f.on.as_ref().map_or_else(
                        || "".to_owned(),
                        |d| d.build_conditions().unwrap_or("".to_owned())
                    )
                ),
                JoinType::Right => format!(
                    "RIGHT JOIN {} {}",
                    f.table,
                    f.on.as_ref().map_or_else(
                        || "".to_owned(),
                        |d| d.build_conditions().unwrap_or("".to_owned())
                    )
                ),
                JoinType::Inner => format!(
                    "INNER JOIN {} {}",
                    f.table,
                    f.on.as_ref().map_or_else(
                        || "".to_owned(),
                        |d| d.build_conditions().unwrap_or("".to_owned())
                    )
                ),
                JoinType::Outer => format!(
                    "OUTER JOIN {} {}",
                    f.table,
                    f.on.as_ref().map_or_else(
                        || "".to_owned(),
                        |d| d.build_conditions().unwrap_or("".to_owned())
                    )
                ),
            })
            .collect::<Vec<String>>()
            .join(" ");

        Some(join_strings)
    }
}
