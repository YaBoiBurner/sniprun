ts_utils=require'nvim-treesitter.ts_utils'
locals=require'nvim-treesitter.locals'



function get_nodes_in_range()
  a = 1
  b = a +2
  return a
end

c=get_nodes_in_range()


function get_scope_of_definition(node,bufnr)
  local def,scope,kind = locals.find_definition(node,bufnr)

  print(ts_utils.get_node_text(node)[1])
  print(ts_utils.get_node_range(def))

  if kind == 'function' then
      function_scope = locals.containing_scope(def, bufnr)
      print("function scope: ",ts_utils.get_node_range(function_scope))
  else
    print("not a function: ", kind)
  end
end




