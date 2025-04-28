use core::panic;
use std::fmt::Debug;

use draftlang_parser::types::{AstNode, Verb};

use crate::types::check_same_type;

/// Compare two AstNodes according to a given comparism
///
/// The following comparisms are supported for the given types:
///
/// - `Str`: `Equal`, `NotEqual`
/// - `Number`: `Equal`, `NotEqual`, `LargerThan`, `LargerThanOrEqual`, `LessThan`, `LessThanOrEqual`
/// - `Boolean`: `Equal`, `NotEqual`
/// - `Null`: `Equal`, `NotEqual`
///
/// All other comparisms will result in a panic
///
/// # Panics
///
/// If the two AstNodes are not of the same type
///
pub fn compare_ast_nodes(left: &AstNode, right: &AstNode, comparism: &Verb) -> bool {
    // check if both nodes are of the same type
    let same_type = check_same_type(&[left.to_owned(), right.to_owned()]);
    if !same_type {
        panic!("Cannot compare two values of different types");
    }

    match (left, right) {
        (AstNode::Str(left), AstNode::Str(right)) => compare_values(left, right, comparism),
        (AstNode::Number(left), AstNode::Number(right)) => compare_values(left, right, comparism),
        (AstNode::Boolean(left), AstNode::Boolean(right)) => match comparism {
            Verb::Equal => left == right,
            Verb::NotEqual => left != right,
            _ => panic!("Invalid comparism for boolean values"),
        },
        (AstNode::Null, AstNode::Null) => match comparism {
            Verb::Equal => true,
            Verb::NotEqual => false,
            _ => panic!("Invalid comparism for null values"),
        },
        _ => false,
    }
}

/// Compare two values of the same type according to a given comparison verb.
///
/// This function supports the following comparisons:
///
/// - `Equal`: Checks if the two values are equal.
/// - `NotEqual`: Checks if the two values are not equal.
/// - `LargerThan`: Checks if the left value is larger than the right value.
/// - `LargerThanOrEqual`: Checks if the left value is larger than or equal to the right value.
/// - `LessThan`: Checks if the left value is less than the right value.
/// - `LessThanOrEqual`: Checks if the left value is less than or equal to the right value.
///
/// # Panics
///
/// This function will panic if an unsupported comparison verb is provided.

fn compare_values<T: PartialEq + PartialOrd + Debug>(
    left: &T,
    right: &T,
    comparism: &Verb,
) -> bool {
    match comparism {
        Verb::Equal => left == right,
        Verb::NotEqual => left != right,
        Verb::LargerThan => left > right,
        Verb::LargerThanOrEqual => left >= right,
        Verb::LessThan => left < right,
        Verb::LessThanOrEqual => left <= right,
        _ => unreachable!(),
    }
}

pub fn get_left_value_truty(node: &AstNode) -> bool {
    match node {
        AstNode::Str(data) => !data.is_empty(),
        AstNode::Number(data) => *data > 0.0,
        AstNode::Boolean(data) => *data,
        AstNode::Null => false,
        _ => false,
    }
}
