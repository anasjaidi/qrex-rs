#[derive(Default)]
pub struct QueryBuilder {
    pub table: String,
    pub query: String,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_table(&mut self, table: impl Into<String>) -> &Self {
        self.table = table.into();
        self
    }

    pub fn create_table() {}

    pub fn drop_table() {}

    pub fn select(&self) -> QueryActionBuilder {
        QueryActionBuilder {
            table: self.table.to_owned(),
            all: false,
            fields: vec![],
        }
    }

    pub fn insert() -> InsertActionBuilder {
        InsertActionBuilder
    }
}

pub struct QueryActionBuilder {
    table: String,
    fields: Vec<String>,

    all: bool,
}

pub enum WhereCondition<T> {
    NATIVE(String),
    NULL,
    NOTNULL,
    AND(Vec<WhereCondition<T>>),
    OR(Vec<WhereCondition<T>>),
    IN(String, Vec<T>),
    NOTIN(String, Vec<T>),
    EQ(String, T),
    NEQ(String, T),
    BETWEEN,
    GT,
    GTE,
    LT,
    LE,
    LIKE,
}

impl QueryActionBuilder {
    pub fn fields(&mut self, fields: &[&str]) -> &Self {
        if self.all {
            // TODO: throw error
        }
        self.fields = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>();
        self
    }

    fn where_condition(&mut self) -> &Self {
        self
    }

    pub fn all_fields(&mut self) -> &Self {
        if self.fields.len() > 0 {
            // TODO: throw error
        };
        self.all = true;
        self
    }
}

/////////////////////////////////////////////
pub trait BuildQuery {
    fn build(&mut self) -> Query;
}

pub struct InsertActionBuilder;

pub struct Query {}
