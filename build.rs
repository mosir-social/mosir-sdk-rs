fn main() {
    println!("cargo:rerun-if-changed=public.graphqls");
    println!("cargo:rerun-if-changed=public.operations.graphql");

    cynic_codegen::register_schema("public")
        .from_sdl_file("public.graphqls")
        .expect("failed to register GraphQL schema")
        .as_default()
        .expect("failed to set default GraphQL schema");
}
