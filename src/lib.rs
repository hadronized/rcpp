//! Preprocessor parser.
//!
//! The preprocessor is responsible in evaluating an input string to produce a translation unit
//! (GLSL code). The preprocessor _recognizes_ several CPP directives and, among them, _interprets_
//! some:
//!
//! - Not interpreted:
//!   - `#include`.
//!   - `#line`.
//!   - `#pragma`.
//!   - `#version`.
//!   - `#extension`.
//! - Interpreted:
//!   - `#define`, `#undef`.
//!   - `#if`, `#ifdef`, `#ifndef`, `#elseif`, `#else`, `#endif`.
//!
//! Non-interpreted CPP directives are meaningful for the rest of the parsing, lexing and
//! compilation stages, thus they are passed along with the rest of the input. It’s especially
//! important for `#include`, for instance — that is not officially recognized by the GLSL
//! preprocessor but can has a wide spread usage. You will be required to resolve those includes by
//! yourself, as it’s not in the scope of this crate.

mod parser;

use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Runtime error while preprocessing.
#[derive(Clone, Debug, PartialEq)]
pub enum PreprocessorError {
  /// A `#define` directive was issued with an identifier that is equal to a previously defined
  /// one and the current define method disallows that.
  NotAuthorizedDefineOverride {
    /// Ident that is already defined.
    ident: String,
    /// Value that is currently defined.
    current_value: Defined,
    /// Candidate value to put instead.
    candidate_value: Defined,
  },
  /// An `#undef` directive was issued with an identifier that is not currently defined.
  UndefineUnknownSymbol { ident: String },
  /// Non-matching conditional, like `#if` vs. `#endif`.
  NonMatchingConditional,
  /// Code-driven error.
  CodeDriven(String),
}

/// Defined content.
#[derive(Clone, Debug, PartialEq)]
pub enum Defined {
  /// Object-like macro.
  Object(String),
  /// Function-like macro.
  Function {
    /// List of arguments.
    arg: Vec<String>,
    /// Body of the macro.
    body: String,
  },
}

/// The preprocessor.
#[derive(Clone, Debug, PartialEq)]
pub struct Preprocessor {
  /// Options to use when preprocessing.
  opt: PreprocessorOpt,
  /// Runtime errors that have occurred while preprocessing.
  runtime_errors: Vec<PreprocessorError>,
  /// Currently defined values; map an identifier to a defined symbol.
  defined_syms: HashMap<String, Defined>,
  /// Currently active conditional code; `true` means that we must continue parsing; `false` that
  /// we should be ignoring code until we meet either a new conditional, or `#endif`.
  conditional_stack: Vec<bool>,
}

impl Preprocessor {
  /// Define a symbol.
  fn define_sym(&mut self, ident: String, value: Defined) {
    match self.defined_syms.entry(ident.clone()) {
      Entry::Vacant(entry) => {
        let _ = entry.insert(value);
      }

      Entry::Occupied(mut entry) => match self.opt.define_method {
        DefineMethod::Override => {
          let _ = entry.insert(value);
        }

        DefineMethod::Preserve => {}

        DefineMethod::FailOnOverride => {
          self
            .runtime_errors
            .push(PreprocessorError::NotAuthorizedDefineOverride {
              ident,
              current_value: entry.get().clone(),
              candidate_value: value,
            });
        }
      },
    }
  }

  /// Undefine a symbol.
  fn undef_sym(&mut self, ident: &str) {
    if self.defined_syms.remove(ident).is_none() {
      self
        .runtime_errors
        .push(PreprocessorError::UndefineUnknownSymbol {
          ident: ident.to_owned(),
        });
    }
  }

  /// Enter a conditional scope.
  fn enter_conditional(&mut self, condition: bool) {
    self.conditional_stack.push(condition);
  }

  /// Leave a conditional scope.
  ///
  /// Return the conditional we were in.
  fn leave_conditional(&mut self) -> Option<bool> {
    let cond = self.conditional_stack.pop();

    if cond.is_none() {
      self
        .runtime_errors
        .push(PreprocessorError::NonMatchingConditional);
    }

    cond
  }

  /// Check whether we should be interpreting the input or just ignore it. Typical cases of ignoring
  /// is inside `#if` where the condition is held false.
  fn is_ignoring(&self) -> bool {
    self.conditional_stack.last().cloned().unwrap_or(true)
  }

  /// Make a preprocessor error.
  fn raise_error(&mut self, error: String) {
    self
      .runtime_errors
      .push(PreprocessorError::CodeDriven(error));
  }

  // /// Run the preprocessor on an input string.
  // pub fn run<I>(self, input: I) -> Result<Output, PreprocessorError>
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
struct PreprocessorOpt {
  /// [`DefineMethod`] to use everytime a `#define` is encountered.
  define_method: DefineMethod,
}

/// Method to apply when running the `#define` directive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DefineMethod {
  /// Override a symbol if it already exists.
  ///
  /// # Example
  ///
  /// ```ignore
  /// #define foo 1
  /// #define foo 2
  ///
  /// bool v = foo == 2; // true
  /// ```
  Override,
  /// Do not define if a symbol already exists.
  ///
  /// # Example
  ///
  /// ```ignore
  /// #define foo 1
  /// #define foo 2
  ///
  /// bool v = foo == 1; // true
  /// ```
  Preserve,
  /// Make the preprocessor fail.
  ///
  /// # Example
  ///
  /// ```ignore
  /// #define foo 1
  /// #define foo 2
  ///
  /// bool v = foo == 1; // doesn’t compile
  /// ```
  FailOnOverride,
}
