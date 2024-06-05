#[derive(Clone, Debug)]
enum Agregate {
    Sum(String),
    Count(String),
    Max(String),
    Min(String),
}

pub trait Select {
    fn set_fields(&mut self);
    fn get_fields(&self);
    fn select_all_fields(&mut self);
    fn select_alias_field(&mut self);
    fn select_agr(&mut self);
    fn select_agr_str(&mut self);
    fn alias(&mut self);
    fn select_fields(&mut self);
    fn select_fields_str(&mut self);
}
