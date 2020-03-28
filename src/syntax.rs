//! The C Preprocessor syntax tree.

/// All the currently available preprocessor directives.
#[derive(Clone, Debug, PartialEq)]
pub enum Directive {
  /// The `#define` directive.
  Define(DefineDirective),
  /// The `#undef` directive.
  Undef(UndefDirective),
  /// The `#if` directive.
  If(IfDirective),
  /// The `#ifdef` directive.
  IfDef(IfDefDirective),
  /// The `#ifndef` directive.
  IfNDef(IfNDefDirective),
  /// The `#else` directive.
  ElseDirective,
  /// The `#elseif` directive.
  ElseIf(ElseIfDirective),
  /// The `#endif` directive.
  EndIf,
  /// The `#error` directive.
  Error(ErrorDirective),
  /// The `#include` directive.
  Include(IncludeDirective),
  /// The `#line` directive.
  Line(LineDirective),
  /// The `#pragma` directive.
  Pragma(PragmaDirective),
  /// The `#version` directive.
  Version(VersionDirective),
  /// The `#extension` directive.
  Extension(ExtensionDirective),
}

/// A `#define` preprocessor directive.
///
/// Allows any expression but only Integer and Float literals make sense
#[derive(Clone, Debug, PartialEq)]
pub enum DefineDirective {
  /// An object macro.
  ///
  /// # Example
  ///
  /// ```ignore
  /// #define foo 123
  /// ```
  Object { ident: String, value: String },

  /// A function macro.
  ///
  /// # Example
  ///
  /// ```ignore
  /// #define foo(x) (123 + (x))
  /// ```
  Function {
    ident: String,
    args: Vec<String>,
    body: String,
  },
}

/// An `#else` preprocessor directive.
#[derive(Clone, Debug, PartialEq)]
pub struct ElseIfDirective {
  /// Carried condition.
  pub condition: String,
}

/// An `#error` preprocessor directive.
#[derive(Clone, Debug, PartialEq)]
pub struct ErrorDirective {
  /// Message to display at compile-time.
  ///
  /// # Example
  ///
  /// ```ignore
  /// #error "Error at compile-time."
  /// ```
  pub message: String,
}

/// An `#if` preprocessor directive.
#[derive(Clone, Debug, PartialEq)]
pub struct IfDirective {
  /// Carried condition.
  pub condition: String,
}

/// An `#ifdef` preprocessor directive.
#[derive(Clone, Debug, PartialEq)]
pub struct IfDefDirective {
  /// Identifier that must be defined for the condition to be true.
  pub ident: String,
}

/// A `#ifndef` preprocessor directive.
#[derive(Clone, Debug, PartialEq)]
pub struct IfNDefDirective {
  /// Identifier that must not be defined for the condition to be true.
  pub ident: String,
}

/// An `#include` name annotation.
#[derive(Clone, Debug, PartialEq)]
pub struct IncludeDirective {
  /// Path to the file.
  pub path: Path,
}

/// A `#line` preprocessor directive.
#[derive(Clone, Debug, PartialEq)]
pub struct LineDirective {
  /// Line number.
  pub line: u32,
  /// “Source string number.”
  pub source_string_number: Option<u32>,
}

/// A `#pragma` preprocessor directive.
/// Holds compiler-specific command.
#[derive(Clone, Debug, PartialEq)]
pub struct PragmaDirective {
  /// Pragma command.
  pub command: String,
}

/// A `#undef` preprocessor directive.
#[derive(Clone, Debug, PartialEq)]
pub struct UndefDirective {
  /// Name to unset.
  pub name: String,
}

/// A `#version` preprocessor directive.
#[derive(Clone, Debug, PartialEq)]
pub struct VersionDirective {
  /// Version to use.
  pub version: u16,
  /// Profile to use.
  pub profile: Option<VersionProfile>,
}

/// A `#version` profile annotation.
#[derive(Clone, Debug, PartialEq)]
pub enum VersionProfile {
  /// The core profile.
  Core,
  /// The compatibility profile.
  Compatibility,
  /// The OpenGL ES profile.
  ES,
}

/// An `#extension` preprocessor directive.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionDirective {
  /// Name of the extension.
  pub name: ExtensionName,
  /// Extension behavior.
  pub behavior: Option<ExtensionBehavior>,
}

/// An `#extension` name annotation.
#[derive(Clone, Debug, PartialEq)]
pub enum ExtensionName {
  /// All extensions you could ever imagine in your whole lifetime (how crazy is that!).
  All,
  /// A specific extension.
  Specific(String),
}

/// An #extension behavior annotation.
#[derive(Clone, Debug, PartialEq)]
pub enum ExtensionBehavior {
  /// The extension is required.
  Require,
  /// The extension is enabled.
  Enable,
  /// Warn if the extension is not available.
  Warn,
  /// Disable the extension.
  Disable,
}

/// A path literal.
#[derive(Clone, Debug, PartialEq)]
pub enum Path {
  /// Specified with angle brackets.
  Absolute(String),
  /// Specified with double quotes.
  Relative(String),
}

/// A function macro representation.
#[derive(Clone, Debug, PartialEq)]
pub struct FnMacro {
  /// Name of the macro.
  name: String,
  /// Name arguments
  args: Vec<String>,
  /// Content of the macro.
  body: String,
}
