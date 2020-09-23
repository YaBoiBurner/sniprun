;; From tree-sitter-python licensed under MIT License
; Copyright (c) 2016 Max Brunsfeld


; Function calls

(decorator) @function
((decorator (dotted_name (identifier) @function))
 (#vim-match? @function "^([A-Z])@!.*$"))

(call
  function: (identifier) @function)

(call
  function: (attribute
              attribute: (identifier) @method))

((call
   function: (identifier) @constructor)
 (#match? @constructor "^[A-Z]"))

((call
  function: (attribute
              attribute: (identifier) @constructor))
 (#match? @constructor "^[A-Z]"))

