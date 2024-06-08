pub enum Order {
    Asc,
    Desc,
}

pub trait OrderBy {
    fn get_order(&self) -> &[(&str, &Order)];
    fn set_order(&mut self, group: Vec<(String, Order)>);

    fn raw_order(&mut self, raw: &str) -> &mut Self;

    fn order_by_expression(&mut self, exp: &str) -> &mut Self {
        // TODO: IMPL ME LATER
        self
    }

    // fn order_by_row(&mut self, row: u32, order: Order) -> &mut Self {
    //     self.set_order(vec![(row.to_string(), Order::Asc)]);
    //     self
    // }
    //
    fn order_by_row_asc(&mut self, row: u32) -> &mut Self {
        self.set_order(vec![(row.to_string(), Order::Asc)]);
        self
    }

    fn order_by_row(&mut self, row: u32, order: Order) -> &mut Self {
        self.set_order(vec![(row.to_string(), order)]);
        self
    }

    fn order_by_row_desc(&mut self, row: u32) -> &mut Self {
        self.set_order(vec![(row.to_string(), Order::Desc)]);
        self
    }

    fn order_by_field_asc(&mut self, field: &str) -> &mut Self {
        self.set_order(vec![(field.to_owned(), Order::Asc)]);
        self
    }

    fn order_by_field_desc(&mut self, field: &str) -> &mut Self {
        self.set_order(vec![(field.to_owned(), Order::Desc)]);
        self
    }

    fn order_by_field(&mut self, field: &str, order: Order) -> &mut Self {
        self.set_order(vec![(field.to_owned(), order)]);
        self
    }

    fn order_by_fields(&mut self, fields: Vec<(String, Order)>) -> &mut Self {
        self.set_order(fields);
        self
    }
}
