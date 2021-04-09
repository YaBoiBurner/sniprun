ts_utils=require'nvim-treesitter.ts_utils'
locals=require'nvim-treesitter.locals'


local M ={}
range_table = {}

function M.get_nodes_in_range()
  a = 1
  b = a +2
  return a
end

-- c=get_nodes_in_range()


function M.get_scope_of_definition(node,bufnr)
  local def,scope,kind = locals.find_definition(node,bufnr)

  if kind == 'function' then
      function_scope = locals.containing_scope(def, bufnr)
      -- print("function scope: ",ts_utils.get_node_range(function_scope))
      line_begin, column_begin, line_end, column_end = ts_utils.get_node_range(function_scope)
      return line_begin, line_end
  elseif kind == 'var' then
      var_definition = def 
  else
    print("not a function: ", kind)
  end
end

return M



