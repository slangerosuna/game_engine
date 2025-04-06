use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro_derive(System)]
pub fn derive_system(input: TokenStream) -> TokenStream {
    todo!()
}

#[proc_macro_derive(Task)]
pub fn derive_task(input: TokenStream) -> TokenStream {
    todo!()
}
