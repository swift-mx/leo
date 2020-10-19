// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{Expression, ExpressionError, Frame};
use leo_static_check::Type;
use leo_typed::SpreadOrExpression as UnresolvedSpreadOrExpression;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpreadOrExpression {
    Spread(Expression),
    Expression(Expression),
}

impl SpreadOrExpression {
    pub(crate) fn type_(&self) -> &Type {
        match self {
            SpreadOrExpression::Spread(expression) => expression.type_(),
            SpreadOrExpression::Expression(expression) => expression.type_(),
        }
    }

    ///
    /// Returns a new `SpreadOrExpression` from a given `UnresolvedSpreadOrExpression`.
    ///
    /// Performs a lookup in the given function body's variable table if the expression contains
    /// user-defined variables.
    ///
    pub fn new(
        frame: &Frame,
        type_: &Type,
        unresolved_expression: UnresolvedSpreadOrExpression,
    ) -> Result<Self, ExpressionError> {
        Ok(match unresolved_expression {
            UnresolvedSpreadOrExpression::Spread(spread) => {
                let spread_resolved = Expression::new(frame, type_, spread)?;
                // TODO: add check for array type or array element type
                SpreadOrExpression::Spread(spread_resolved)
            }
            UnresolvedSpreadOrExpression::Expression(expression) => {
                let expression_resolved = Expression::new(frame, type_, expression)?;
                // TODO: add check for array type or array element type
                SpreadOrExpression::Expression(expression_resolved)
            }
        })
    }
}
