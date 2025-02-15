return function(ui)
    ui.vertical {
        cross_align = CrossAlign.fill,
        gap = 1,
        ui.horizontal {
            justify = Justify.space_evenly,
            ui.label "one",
            ui.label "two",
            ui.label "three",
        },
        ui.label(string.format("hello, world: %d", 2)),
        ui.label(string.format("hello, world: %d", 3)),
        ui.label(string.format("hello, world: %d", 4)),
    }
end
