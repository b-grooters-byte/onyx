use crate::token;



pub trait Node {
    fn token_literal(&self) -> String;
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {

    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn statements(&self) -> &Vec<Box<dyn Statement>> {
        &self.statements
    }

    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }
}

pub struct Identifier {
    token: token::Token,
    value: String,
}

impl Identifier {
    pub fn new(token: token::Token, value: String) -> Self {
        Identifier {
            token,
            value,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}   

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

impl LetStatement {
    pub fn name(&self) -> &Identifier {
        &self.name
    }
}