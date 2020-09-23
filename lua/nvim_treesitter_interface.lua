local locals = require'nvim-treesitter.locals'
local parsers = require'nvim-treesitter.parsers'
local ts_utils = require'nvim-treesitter.ts_utils'
local utils = require'nvim-treesitter.utils'
local query_module = require'nvim-treesitter.query'
local api=vim.api

local M = {}

function M.get_definition_scope_of_function_node(node, bufnr)
  -- local node = ts_utils.get_node_at_cursor()
  local bufnr = bufnr or api.nvim_get_current_buf()

  local definition = locals.find_definition(node, bufnr)
  print("def=",definition)
  print("defname=", ts_utils.get_node_text(definition)[1])


  local containing_scope = locals.containing_scope(definition, bufnr)

  return ts_utils.get_node_range(containing_scope)
end



-- ! the end row is exclusive so you'll often need to add 1
function M.list_nodes_in_range(start_row, end_row, bufnr)
  local bufnr = bufnr or api.nvim_get_current_buf()
  for _,node in ipairs(query_module.get_capture_matches(bufnr,"@function","code_deps")) do
    print("node found:", node.node)
    print("node's name:" , ts_utils.get_node_text(node.node)[1])
    local sr,sc,er,ec = M.get_definition_scope_of_function_node(node.node,bufnr)
    print(sr,sc,er,ec)
  end

end





return M

