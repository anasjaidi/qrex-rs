pub struct QueryActionBuilder<T>
where
    T: Sized,
{
    pub table: String,
    pub fields: Vec<String>,
    pub conditions: Vec<WhereCondition<T>>,
    pub all: bool,
}

pub enum WhereCondition<T>
where
    T: Sized,
{
    NATIVE(String),
    NULL,
    NOTNULL,
    AND(Vec<WhereCondition<T>>),
    OR(Vec<WhereCondition<T>>),
    IN(String, Vec<T>),
    NOTIN(String, Vec<T>),
    EQ(String, T),
    NEQ(String, T),
    BETWEEN(String, T, T),
    GT(String, T),
    GTE(String, T),
    LT(String, T),
    LE(String, T),
    LIKE(String, String),
}

impl<T> QueryActionBuilder<T> {
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

    pub fn from_table(&mut self, table: impl Into<String>) -> &Self {
        self.table = table.into();
        self
    }

    pub fn where_condition(&mut self) -> &Self {
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
