/*
 * Copyright 2019 The Starlark in Rust Authors.
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Call-related bytecode interpreter code.

use std::{
    fmt,
    fmt::{Display, Formatter},
};

use crate::{
    collections::symbol_map::Symbol,
    eval::{
        bc::{instr_arg::BcInstrArg, stack_ptr::BcStackPtr},
        Arguments,
    },
    values::FrozenStringValue,
};

/// Call arguments.
pub(crate) trait BcCallArgs: BcInstrArg {
    fn pop_from_stack<'a, 'v>(&'a self, stack: &'a BcStackPtr<'v, '_>) -> Arguments<'v, 'a>;
}

/// Full call arguments: positional, named, star and star-star. All taken from the stack.
#[derive(Debug)]
pub(crate) struct BcCallArgsFull {
    pub(crate) pos_named: u32,
    pub(crate) names: Box<[(Symbol, FrozenStringValue)]>,
    pub(crate) args: bool,
    pub(crate) kwargs: bool,
}

/// Positional-only call arguments, from stack.
#[derive(Debug)]
pub(crate) struct BcCallArgsPos {
    /// Number of positional arguments.
    pub(crate) pos: u32,
}

impl BcCallArgsFull {
    fn pos(&self) -> u32 {
        assert!(self.pos_named as usize >= self.names.len());
        self.pos_named - (self.names.len() as u32)
    }
}

impl Display for BcCallArgsFull {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut first = true;
        let mut write_sep = |f: &mut Formatter| {
            if !first {
                write!(f, " ")?;
            }
            first = false;
            Ok(())
        };
        // Number of positional arguments.
        if self.pos() != 0 {
            write_sep(f)?;
            write!(f, "{}", self.pos())?;
        }
        // Named arguments.
        for (_, name) in &*self.names {
            write_sep(f)?;
            write!(f, "{}", name.as_str())?;
        }
        // Star argument?
        if self.args {
            write_sep(f)?;
            write!(f, "*")?;
        }
        // Star-star argument?
        if self.kwargs {
            write_sep(f)?;
            write!(f, "**")?;
        }
        Ok(())
    }
}

impl BcCallArgs for BcCallArgsFull {
    fn pop_from_stack<'a, 'v>(&'a self, stack: &'a BcStackPtr<'v, '_>) -> Arguments<'v, 'a> {
        stack.pop_args(self)
    }
}

impl BcCallArgs for BcCallArgsPos {
    fn pop_from_stack<'a, 'v>(&'a self, stack: &'a BcStackPtr<'v, '_>) -> Arguments<'v, 'a> {
        stack.pop_args_pos(self)
    }
}
