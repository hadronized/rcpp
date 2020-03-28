# A Rust implementation of the C Preprocessor

This crate is a Rust implementation of _cpp_, following the official specification of
the preprocessor [here](https://gcc.gnu.org/onlinedocs/cpp).

## Motivation

Original need occurred in the [glsl] crate, that needed a pure Rust implementation of a slightly
different _cpp_ implementation. That motivation is the reason why _rcpp_ is designed to be
extensible around the preprocessor directives.

[glsl]: https://github.com/phaazon/glsl
