pub trait BuildQuery {
    fn build(&mut self) -> Query;
}

pub struct Query {}
