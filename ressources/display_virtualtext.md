Virtual text is permanently displayed at the right of the line (/ last line of bloc), and can be cleared with
`:SnipClose` (or a shortcut to `<Plug>SnipClose`).

Output for ok and error results are highlighted with the groups
`SniprunVirtualTextOk` and `SniprunVirtualTextErr`

One can choose to display only the 'ok' results or 'error' results or both in the display configuration via the keys:
- "SniprunVirtualTextOk"
- "SniprunVirtualTextErr"

Multiline output is shortened (...\<last line of output> for ok, \<first line of output>... for errors)

![](visual_assets/virtual_text.png)
