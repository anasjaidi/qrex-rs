use super::condition::Condition;

pub trait GroupBy {
    fn get_group(&self) -> Vec<&str>;
    fn set_group(&mut self, group: Vec<String>);

    fn get_having_condition(&self) -> Option<&Condition>;

    fn set_having_condition(&mut self, condition: Condition);

    fn group_by_field(&mut self, field: &str) -> &mut Self {
        self.set_group(vec![field.to_owned()]);
        self
    }

    fn build_group_by(&self) -> Option<String> {
        let having = self.get_having_condition()?.build_conditions()?;

        let fields = self.get_group();

        if fields.is_empty() {
            return None;
        }

        let str_field = fields.join(", ");

        Some(format!("GROUP BY {} HAVING {}", str_field, having))
    }

    fn having(&mut self, condition: Condition) -> &Self {
        self.set_having_condition(condition);
        self
    }

    fn group_by_fields(&mut self, fields: Vec<String>) -> &mut Self {
        self.set_group(fields);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::query_builder::condition::Condition;

    use super::GroupBy;

    #[derive(Clone, Default)]
    struct GrouByTest {
        groups: Vec<String>,
        having: Option<Condition>,
    }

    impl GroupBy for GrouByTest {
        fn get_group(&self) -> Vec<&str> {
            self.groups.iter().map(|s| s.as_str()).collect()
        }

        fn set_having_condition(&mut self, condition: Condition) {}

        fn set_group(&mut self, group: Vec<String>) {}

        fn get_having_condition(&self) -> Option<&Condition> {
            self.having.as_ref()
        }
    }
}
