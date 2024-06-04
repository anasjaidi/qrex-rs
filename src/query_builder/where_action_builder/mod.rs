pub struct QueryActionBuilder<T>
where
    T: Sized,
{
    pub table: String,
    pub fields: Vec<String>,
    pub conditions: Option<WhereCondition<T>>,
    pub all: bool,
}

impl<T> Default for QueryActionBuilder<T> {
    fn default() -> Self {
        Self {
            table: String::new(),
            all: false,
            conditions: None,
            fields: vec![],
        }
    }
}

pub enum WhereCondition<T>
where
    T: Sized,
{
    NATIVE(String),
    NULL,
    NOTNULL,
    AND(Box<Self>, Box<Self>),
    OR(Box<Self>, Box<Self>),
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

impl<T> Clone for WhereCondition<T> {
    fn clone(&self) -> Self {
        todo!();
    }
}

impl<T> QueryActionBuilder<T>
where
    T: Sized,
{
    // TODO: fix this function is just a boilerplate for build function
    pub fn dummy_build(&self) -> String {
        let mut w = String::new();
        fn gc<T>(a: &WhereCondition<T>, w: &mut str) {}

        w
    }

    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn or_where(&mut self, condition: WhereCondition<T>) -> &Self {
        if let None = self.conditions {
            // TODO: handle error
        }
        self.conditions = Some(WhereCondition::OR(
            Box::new(self.conditions.as_ref().unwrap().clone()),
            Box::new(condition),
        ));
        self
    }

    pub fn whene(&mut self, condition: WhereCondition<T>) -> &Self {
        match &self.conditions {
            None => {
                self.conditions = Some(condition);
                self
            }
            Some(c) => {
                self.conditions =
                    Some(WhereCondition::OR(Box::new(c.clone()), Box::new(condition)));
                self
            }
        }
    }

    pub fn all_fields(&mut self) -> &Self {
        if self.fields.len() > 0 {
            // TODO: throw error
        };
        self.all = true;
        self
    }
}
