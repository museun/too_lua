---@param ui ui
local function show_nodes(ui)
    for i = 0, 10000 do
        ui.label(string.format("hello, world. #%d", i))
    end
end

---@param ui ui
return function(ui)
    ui.vertical {
        scrollable = true,
        cross_align = CrossAlign.fill,
        show_nodes(ui)
    }
end
