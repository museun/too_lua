local something = Value.persist("test", 0.1)

rt.spawn(function()
    repeat
        something.value = something.value + 0.001
        rt.sleep_ms(10)
    until false
end)

---@param ui ui
return function(ui)
    ui.center {
        ui.vertical {
            gap = 1,
            cross_align = CrossAlign.center,
            justify = Justify.space_between,
            ui.label(lazy { function() return string.format("%0.2f", something.value) end }),
            ui.slider {
                value = something,
            },
            ui.progress {
                value = something,
            }
        }
    }
end
