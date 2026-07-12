object_definition {
    path: str, // will be a key
    layer: u8,
    size: f32..f32 | f32
    tags: [],
    biomes: [],
    modifiers: [fibalbe]
    tune: {
        brightness: f32,
        contract: f32,
        opacity: f32,
    }
}

object_placed {
    definition: object_definition,
    x: f32,
    y: f32,
}
