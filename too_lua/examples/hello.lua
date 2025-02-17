-- local function children(ui)
--     for i = 0, 100 do
--         ui.label(string.format("hello: %d", i))
--     end
-- end

local checked = Value.new(false)

---@param ui ui
return function(ui)
    -- ui.vertical {
    --     scrollable = true,
    --     cross_align = CrossAlign.fill,
    --     children(ui)
    -- }

    ui.center {
        -- I don't like this name, at all
        ui.constrained {
            constraint = Constraint.min_width(20),
            ui.toggle_switch {
                class = Toggle.large,
                value = checked,
            }
        }
    }
end
