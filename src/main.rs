pub mod ast;
pub mod runtime;
pub mod tokens;

use std::{env, collections::HashMap};
use std::fs::File;
use std::io::Read;

use ast::{Node, Visitor};
use runtime::{Interpreter, Context, ValueType};
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
    fn visit_relational_expression(&mut self, node: &Node) -> () {
        println!("{}visit_relative_expression:", " ".repeat(self.indent));
        self.indent += 2;
        if let Node::RelationalExpression {
            lhs,
            op,
            rhs,
        } = node
        {
            println!("{}lhs:", " ".repeat(self.indent));
            self.indent += 2;
            lhs.accept(self);
            self.indent -= 2;
            println!("{}op:", " ".repeat(self.indent));
            self.indent += 2;
            rhs.accept(self);
            self.indent -= 2;
        } else {
            panic!("Expected RelativeExpression node");
        }
        self.indent -= 2;
    }

    fn visit_logical_expression(&mut self, node: &Node) -> () {
        println!("{}visit_logical_expression:", " ".repeat(self.indent));
        self.indent += 2;
        if let Node::LogicalExpression {
            lhs,
            op,
            rhs,
        } = node
        {
            println!("{}lhs:", " ".repeat(self.indent));
            self.indent += 2;
            lhs.accept(self);
            self.indent -= 2;
            println!("{}op:", " ".repeat(self.indent));
            self.indent += 2;
            rhs.accept(self);
            self.indent -= 2;
        } else {
            panic!("Expected LogicalExpression node");
        }
        self.indent -= 2;
    }
}

fn main() -> () {
    run_test_assert();
    let ctx = execute_return_global_ctx(String::from("prototyping.scorch"));
    dbg!(ctx);
}

fn run_test_assert() {
    let ctx = execute_return_global_ctx(String::from("test.scorch"));
    let variables = [
        "rel_t1", "rel_t2", "rel_t3", "rel_t4", "rel_t5", "rel_t6", "rel_t7", "rel_t8",
        "rel_t9", "rel_t10", "rel_t11", "rel_t12",
    ];
    let expected_results = [
        true,  // rel_t1 := 5 < 10
        false, // rel_t2 := 5 > 10
        true,  // rel_t3 := 5 <= 10
        false, // rel_t4 := 5 >= 10
        false, // rel_t5 := 5 == 10
        true,  // rel_t6 := 5 != 10
        true,  // rel_t7 := 5 == 5
        false, // rel_t8 := 5 != 5
        true,  // rel_t9  := 5 <= 5
        true,  // rel_t10 := 5 >= 5
        false, // rel_t11 := 5 < 5
        false, // rel_t12 := 5 > 5
    ];
    for i in 0..11 {
        let variable = variables[i];
        let expected_result = expected_results[i];
        let value = *ctx.variables[*&variable].clone();
        if let ValueType::Bool(v) = value {
            if v == expected_result {
                println!("test passed: {}", variable);
            } else {
                panic!("failed test: bool value");
            }
        } else  {
            dbg!(variables);
            dbg!(variable);
            panic!("failed test: bool value");
        }
    }
}

fn execute_return_global_ctx(filename: String) -> Box<Context> {
    let mut tokenizer = tokens::create_tokenizer();
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    tokenizer.tokenize(&contents.as_str());

    let tokens = tokenizer.tokens;
    let ast_root = ast::parse_program(&tokens);

    let mut interpreter = Interpreter {
        context: runtime::Context::new(),
    };

    ast_root.accept(&mut interpreter);
    
    let ctx = interpreter.context;
    
    return Box::new(ctx);
}
