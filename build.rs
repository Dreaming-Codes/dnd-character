fn main() {
    #[cfg(feature = "api")]
    cynic_codegen::register_schema("dnd5eapi")
        .from_sdl_file("schemas/dnd5eapi.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
