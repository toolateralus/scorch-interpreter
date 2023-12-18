pub mod ast;
pub mod runtime;
pub mod tokens;

use std::env;
use std::fs::File;
use std::io::Read;

use ast::{Node, Visitor};
use runtime::Interpreter;
use tokens::*;
pub struct PrintVisitor {
    pub indent: usize,
}

impl Visitor<()> for PrintVisitor {
    fn visit_block(&mut self, node: &Node) {
        println!("{}visit_block:", " ".repeat(self.indent));
        self.indent += 2;
        if let Node::Block(statements) = node {
            for statement in statements {
                statement.accept(self);
            }
        } else {
            panic!("Expected Block node");
        }
        self.indent -= 2;
    }
    fn visit_declaration(&mut self, node: &Node) -> () {
        println!("{}visit_declaration:", " ".repeat(self.indent));
        self.indent += 2;
        if let Node::DeclStmt {
            target_type,
            id,
            expression,
        } = node
        {
            println!("{}type: {}", " ".repeat(self.indent), target_type);
            println!("{}id: {}", " ".repeat(self.indent), id);
            println!("{}expression:", " ".repeat(self.indent));
            self.indent += 2;
            expression.accept(self);
            self.indent -= 2;
        } else {
            panic!("Expected Declaration node");
        }
        self.indent -= 2;
    }
    fn visit_identifier(&mut self, _node: &Node) -> () {
        println!("{}visit_identifier:", " ".repeat(self.indent));
    }
    fn visit_number(&mut self, _node: &Node) -> () {
        println!("{}visit_number:", " ".repeat(self.indent));
    }
    fn visit_term(&mut self, _node: &Node) -> () {
        println!("{}visit_number:", " ".repeat(self.indent));
    }
    fn visit_factor(&mut self, node: &Node) -> () {
        if let Node::Number(value) = node {
            println!("{}visit_factor: {}", " ".repeat(self.indent), value);
        } else if let Node::Identifier(id) = node {
            println!("{}visit_factor: {}", " ".repeat(self.indent), id);
        } else if let Node::Expression(root) = node {
            println!("{}visit_factor:", " ".repeat(self.indent));
            self.indent += 2;
            root.accept(self);
            self.indent -= 2;
        } else {
            dbg!(node);
            panic!("Expected Number or Identifier node");
        }
    }
    fn visit_assignment(&mut self, _node: &Node) -> () {
        println!("{}visit_number:", " ".repeat(self.indent));
    }
    fn visit_binary_op(&mut self, node: &Node) -> () {
        match node {
            Node::AddOp(lhs, rhs) => {
                println!("{}visit_add_op:", " ".repeat(self.indent));
                self.indent += 2;
                lhs.accept(self);
                rhs.accept(self);
                self.indent -= 2;
            }
            Node::SubOp(lhs, rhs) => {
                println!("{}visit_sub_op:", " ".repeat(self.indent));
                self.indent += 2;
                lhs.accept(self);
                rhs.accept(self);
                self.indent -= 2;
            }
            Node::MulOp(lhs, rhs) => {
                println!("{}visit_mul_op:", " ".repeat(self.indent));
                self.indent += 2;
                lhs.accept(self);
                rhs.accept(self);
                self.indent -= 2;
            }
            Node::DivOp(lhs, rhs) => {
                println!("{}visit_div_op:", " ".repeat(self.indent));
                self.indent += 2;
                lhs.accept(self);
                rhs.accept(self);
                self.indent -= 2;
            }
            _ => panic!("Expected binary operation node"),
        }
    }
    fn visit_string(&mut self, node: &Node) -> () {
        if let Node::String(value) = node {
            println!("{}visit_string: {}", " ".repeat(self.indent), value);
        } else {
            panic!("Expected String node");
        }
    }
    fn visit_expression(&mut self, node: &Node) -> () {
        println!("{}visit_expression:", " ".repeat(self.indent));
        self.indent += 2;
        if let Node::Expression(root) = node {
            root.accept(self);
        } else {
            panic!("Expected Expression node");
        }
        self.indent -= 2;
    }

    fn visit_eof(&mut self, _node: &Node) -> () {
        () // do nothing.
    }

    fn visit_not_op(&mut self, node: &Node) -> () {
        println!("{}visit_not_op:", " ".repeat(self.indent));
        self.indent += 2;
        if let Node::Expression(root) = node {
            root.accept(self);
        } else {
            panic!("Expected Expression node");
        }
        self.indent -= 2;
    }

    fn visit_neg_op(&mut self, node: &Node) -> () {
        println!("{}visit_neg_op:", " ".repeat(self.indent));
        self.indent += 2;
        if let Node::Expression(root) = node {
            root.accept(self);
        } else {
            panic!("Expected Expression node");
        }
        self.indent -= 2;
    }

    fn visit_bool(&mut self, node: &Node) -> () {
        if let Node::Bool(value) = node {
            println!("{}visit_bool: {}", " ".repeat(self.indent), value);
        } else {
            panic!("Expected Bool node");
        }
    }

    fn visit_where_stmnt(&mut self, node: &Node) -> () {
        println!("{}visit_where_stmnt:", " ".repeat(self.indent));
        self.indent += 2;
        if let Node::WhereStmnt {
            condition,
            block: true_block,
            or_stmnt,
        } = node
        {
            println!("{}condition:", " ".repeat(self.indent));
            self.indent += 2;
            condition.accept(self);
            self.indent -= 2;
            println!("{}body:", " ".repeat(self.indent));
            self.indent += 2;
            true_block.accept(self);
            self.indent -= 2;
            println!("{}otherwise:", " ".repeat(self.indent));
            self.indent += 2;
            or_stmnt.as_ref().unwrap().accept(self);
            self.indent -= 2;
        } else {
            panic!("Expected WhereStmnt node");
        }
        self.indent -= 2;
    }

    fn visit_or_stmnt(&mut self, node: &Node) -> () {
        println!("{}visit_or_stmnt:", " ".repeat(self.indent));
        self.indent += 2;
        if let Node::OrStmnt {
            condition: _,
            block,
            or_stmnt: _,
        } = node
        {
            block.accept(self);
        } else {
            panic!("Expected OrStmnt node");
        }
        self.indent -= 2;
    }
}

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    println!("Command-line arguments: {:?}", args);

    let mut tokenizer = tokens::create_tokenizer();

    let mut file = File::open("prototyping.scorch").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    tokenizer.tokenize(&contents.as_str());

    let tokens = tokenizer.tokens;
    let ast_root = ast::parse_program(&tokens);

    let mut visitor = Interpreter {
        context: runtime::Context::new(),
    };

    ast_root.accept(&mut visitor);

    dbg!(visitor.context);
}
