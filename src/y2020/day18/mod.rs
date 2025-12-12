use crate::read_input;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Operand {
    Expression(Expression),
    Value(usize),
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Multiplication,
    Addition,
}

#[derive(Default, Debug, Clone)]
struct Expression {
    operands: Vec<Operand>,
    operations: Vec<Operation>,
}

impl Expression {
    pub fn evaluate_addition_first(&self) -> usize {
        let mut expression_without_additions = Expression::default();
        expression_without_additions
            .operands
            .push(match &self.operands[0] {
                Operand::Expression(e) => Operand::Value(e.clone().evaluate_addition_first()),
                Operand::Value(v) => Operand::Value(*v),
            });

        for (i, operation) in self.operations.iter().enumerate() {
            match operation {
                &Operation::Multiplication => {
                    match &self.operands[i + 1] {
                        Operand::Expression(e) => expression_without_additions
                            .operands
                            .push(Operand::Value(e.evaluate_addition_first())),
                        Operand::Value(e) => expression_without_additions
                            .operands
                            .push(Operand::Value(e.clone())),
                    }
                    expression_without_additions.operations.push(*operation);
                }
                &Operation::Addition => {
                    match expression_without_additions.operands.last_mut().unwrap() {
                        Operand::Value(v) => {
                            let to_add = match &self.operands[i + 1] {
                                Operand::Expression(e) => e.evaluate_addition_first(),
                                Operand::Value(v) => *v,
                            };
                            *v = *v + to_add;
                        }
                        _ => panic!("Last operand is not Value"),
                    };
                }
            }
        }

        expression_without_additions.evaluate()
    }

    pub fn evaluate(&self) -> usize {
        let mut result = 0;

        let mut operations = vec![&Operation::Addition];
        operations.extend(&self.operations);

        for (operation, operand) in operations.iter().zip(&self.operands) {
            let v = match operand {
                &Operand::Expression(ref e) => e.evaluate(),
                &Operand::Value(v) => v,
            };

            match operation {
                Operation::Addition => result += v,
                Operation::Multiplication => result *= v,
            }
        }

        result
    }
}

fn read_expressions() -> Vec<Expression> {
    read_input(2020, 18)
        .unwrap()
        .map_while(Result::ok)
        .map(|line| {
            let mut expressions_queue = VecDeque::new();
            expressions_queue.push_back(Expression::default());

            for c in line.chars() {
                match c {
                    '+' => {
                        expressions_queue
                            .back_mut()
                            .unwrap()
                            .operations
                            .push(Operation::Addition);
                    }
                    '*' => {
                        expressions_queue
                            .back_mut()
                            .unwrap()
                            .operations
                            .push(Operation::Multiplication);
                    }
                    '(' => {
                        expressions_queue.push_back(Expression::default());
                    }
                    ')' => {
                        let sub_expression = expressions_queue.pop_back().unwrap();
                        expressions_queue
                            .back_mut()
                            .unwrap()
                            .operands
                            .push(Operand::Expression(sub_expression));
                    }
                    _ => {
                        if c != ' ' {
                            expressions_queue
                                .back_mut()
                                .unwrap()
                                .operands
                                .push(Operand::Value(c.to_digit(10).unwrap().try_into().unwrap()));
                        }
                    }
                }
            }

            expressions_queue.pop_back().unwrap()
        })
        .collect()
}

pub fn run_part_1() {
    let v: usize = read_expressions().iter().map(|e| e.evaluate()).sum();
    assert_eq!(v, 45283905029161);
}

pub fn run_part_2() {
    let v: usize = read_expressions()
        .iter()
        .map(|e| e.evaluate_addition_first())
        .sum();

    assert_eq!(v, 216975281211165);
}
