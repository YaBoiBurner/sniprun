; Forked from https://github.com/tree-sitter/tree-sitter-rust
; Copyright (c) 2017 Maxim Sokolov
; Licensed under the MIT license.

; Identifier conventions

; Function calls
(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function))

(generic_function
  function: (identifier) @function)
(generic_function
  function: (scoped_identifier
    name: (identifier) @function))
(generic_function
  function: (field_expression
    field: (field_identifier) @function))
