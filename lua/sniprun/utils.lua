ts_utils=require'nvim-treesitter.ts_utils'
locals=require'nvim-treesitter.locals'


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
      line_begin, column_begin, line_end, column_end = ts_utils.get_node_range(function_scope)
      return line_begin, line_end
  elseif kind == 'var' then
      var_definition = def 
      def_statement = var_definition:parent():parent()
      line_begin, column_begin, line_end, column_end = ts_utils.get_node_range(def_statement)
      return line_begin, line_end
  elseif kind == 'method' then
      function_scope = locals.containing_scope(def, bufnr)
      print(ts_utils.get_node_range(function_scope))
      line_begin, column_begin, line_end, column_end = ts_utils.get_node_range(function_scope)
      return line_begin, line_end
  end
end

return M



