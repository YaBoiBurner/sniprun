local locals = require'nvim-treesitter.locals'
local parsers = require'nvim-treesitter.parsers'
local ts_utils = require'nvim-treesitter.ts_utils'
local utils = require'nvim-treesitter.utils'
local api=vim.api

local M = {}

function M.get_definition_scope_of_function_node(node, bufnr)
  -- local node = ts_utils.get_node_at_cursor()
  local bufnr = bufnr or api.nvim_get_current_buf()

  local definition = locals.find_definition(node, bufnr)


  local containing_scope = locals.containing_scope(definition, bufnr)

  return ts_utils.get_node_range(containing_scope)
end



function M.list_nodes_in_range(start_row, start_col, end_row, end_col, bufnr)
  local bufnr = bufnr or api.nvim_get_current_buf()

  -- get a table of all function call in the scope
    -- get a table of all method calls
    -- get a table of all used variables

  --cheating...
 fnode = ts_utils.get_node_at_cursor()
 function_calls = {}
 table.insert(function_calls, fnode)


 for _,call in ipairs(function_calls) do
   print(M.get_definition_scope_of_function_node(call, bufnr))
   end

end





return M

