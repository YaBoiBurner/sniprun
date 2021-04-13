ts_utils=require'nvim-treesitter.ts_utils'
locals=require'nvim-treesitter.locals'
tsrange=require'nvim-treesitter.tsrange'.TSRange


local M ={}
range_table = {}

function M.get_nodes_in_range()
  a = 1
  b = a +2
  return a
end

c=M.get_nodes_in_range() *
  2




a=c


function M.get_scope_of_definition(node,bufnr)
  local def,scope,kind = locals.find_definition(node,bufnr)
  if kind == 'function' then
      function_scope = locals.containing_scope(def, bufnr)
      return function_scope
  elseif kind == 'var' then
      var_definition = def 
      def_statement = var_definition:parent():parent()
      return def_statement
  elseif kind == 'method' then
      method_scope = locals.containing_scope(def, bufnr)
      return method_scope
  end
end

function M.test(node)
  local up = M.get_scope_of_definition(node, 0)
  line_begin, column_begin, line_end, column_end = ts_utils.get_node_range(up)
  print(line_begin, line_end)
end
return M



