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
  --TODO if def = node abort
  if definition == node then
    print("oupsie")
    return
  end
  -- print("def=",definition)
  -- print("defname=", ts_utils.get_node_text(definition)[1])
  -- print("defscope=", ts_utils.get_node_range(definition))
  --

  local containing_scope = locals.containing_scope(definition, bufnr)

  return ts_utils.get_node_range(containing_scope)
end



function M.get_capture_matches(bufnr, capture_string, query_group, start_row, end_row)
    if not string.sub(capture_string, 1,2) == '@' then
      print('capture_string must start with "@"')
    return
  end

  --remove leading "@"
  capture_string = string.sub(capture_string, 2)

  local matches = {}
  for match in M.iter_group_results(bufnr, query_group, start_row, end_row) do
    local insert = utils.get_at_path(match, capture_string)

    if insert then
      table.insert(matches, insert)
    end
  end
  return matches
end

function M.iter_group_results(bufnr, query_group,start_row, end_row)
  local lang = parsers.get_buf_lang(bufnr)
  if not lang then return function() end end

  local query =query_module.get_query(lang, query_group)
  if not query then return function() end end

  local parser = parsers.get_parser(bufnr, lang)
  if not parser then return function() end end

  local root = parser:parse():root()
  -- local start_row, _, end_row, _ = root:range()

  -- The end row is exclusive so we need to add 1 to it.
  return query_module.iter_prepared_matches(query, root, bufnr, start_row, end_row + 1)
end



-- ! the end row is exclusive so you'll often need to add 1
function M.list_nodes_in_range(start_row, end_row, bufnr)
  local bufnr = bufnr or api.nvim_get_current_buf()
  for _,node in ipairs(M.get_capture_matches(bufnr,"@function","code_deps",start_row, end_row)) do
    -- print("node found:", node.node)
    -- print("node's name:" , ts_utils.get_node_text(node.node)[1])
    local sr,sc,er,ec = M.get_definition_scope_of_function_node(node.node,bufnr)
    print(sr,sc,er,ec)

  end

end





return M

