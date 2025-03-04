local M={}
M.term={}
M.fw_handle=0
M.term.opened = 0
M.term.buffer = -1
M.term.window_handle = 0
M.term.current_line = -1
M.term.chan = -1

function M.fw_open(row, column, message, ok, temp)
  M.fw_close()

  hl_ok = "SniprunFloatingWinOk"
  hl_err = "SniprunFloatingWinErr"
  if ok then
    hl = hl_ok
  else
    hl = hl_err
  end

  namespace_id = vim.api.nvim_create_namespace("")

  buf = 0 -- buffer 
  w = 0
  h = -1
  bp = {row , column}
  message_map = {}
  bufnr = vim.api.nvim_create_buf(false, true)
  for line in message:gmatch("([^\n]*)\n?") do
    h = h + 1
    w = math.max(w,string.len(line)) 
    vim.api.nvim_buf_set_lines(bufnr,h,h+1,false,{line}) 
    vim.api.nvim_buf_add_highlight(bufnr, namespace_id, hl, h,0,-1) -- highlight lines in floating window
  end
  M.fw_handle = vim.api.nvim_open_win(bufnr, false, {relative='win', width=w+1, height=h, bufpos=bp, focusable=false, style='minimal', border='single'})
end

function M.term_open()
  if M.term.opened ~= 0 then return end
  vim.cmd(':rightb45vsplit')
  local buf = vim.api.nvim_create_buf(false,true)
  local win = vim.api.nvim_get_current_win()
  vim.api.nvim_win_set_buf(win,buf)
  local chan = vim.api.nvim_open_term(buf, {})
  vim.cmd("set scrollback=1")

  vim.cmd("wincmd p")
  M.term.opened = 1
  M.term.window_handle = win
  M.term.buffer = buf
  M.term.chan = chan
end

function M.write_to_term(message, ok)
  M.term_open()

  h = M.term.current_line or -1

  status = "------"
  if ok then
    status = "--OK--"
  else
    status = "ERROR-"
  end
  
  local width = vim.api.nvim_win_get_width(M.term.window_handle)  
  half_width = (width - 6) / 2
  message = string.rep("-",half_width)..status..string.rep("-", half_width).."\n"..message

  for line in message:gmatch("([^\n]*)\n?") do
    h = h +1
    vim.api.nvim_chan_send(M.term.chan, line)     
    vim.api.nvim_chan_send(M.term.chan, "\n\r");
  end
  vim.api.nvim_chan_send(M.term.chan, "\n\r");
  M.term.current_line = h 

end


function M.close_all()
  M.fw_close()
  M.clear_virtual_text()
  M.term_close()
end


function M.fw_close()
  if M.fw_handle == 0 then return end
  vim.api.nvim_win_close(M.fw_handle, true)
  M.fw_handle = 0
end


function M.clear_virtual_text()
  vim.cmd("let sniprun_namespace_id = nvim_create_namespace('sniprun')\n call nvim_buf_clear_namespace(0,sniprun_namespace_id, 0 ,-1)")
end

function M.term_close()
  if M.term.window_handle == 0 then return end
  vim.api.nvim_win_close(M.term.window_handle, true)
  M.term.opened = 0
  M.term.window_handle = 0
  M.term.buffer = -1
  M.term.current_line = 0
  M.term.chan=-1
end




return M
