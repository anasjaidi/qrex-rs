#[derive(Clone, Debug)]
enum Agregate {
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

pub trait Select {
    fn set_fields(&mut self, fields: Vec<(String, String)>);

    fn get_fields(&self) -> &[String];

    fn select_all_fields(&mut self) {
        let fields = self.get_fields();
        self.set_fields()l
    }
    fn select_alias_field(&mut self);
    fn select_agr(&mut self);
    fn select_agr_str(&mut self);
    fn alias(&mut self);
    fn select_fields(&mut self);
    fn select_fields_str(&mut self);
}
