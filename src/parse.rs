pub struct Parser {
    source: String,
}

impl Parser {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn parse(source: String) -> Result<String, ()> {
        todo!()
    }
}
