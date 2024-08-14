mod spec;

/// Must be applied to an inline mod which will be hidden.
///
/// Outputs every struct like the original along with a `spec` module which
/// contain structs used to derialize with the pkpass specification names.
///
/// # Example
///
/// ```
/// #[pkpass_derive::spec]
/// mod _dummy {
///   /// bla, bla, desc
///   #[derive(Debug, Derserialize, Serialize)]
///   struct Pass {
///     #[serde(default, skip_serializing_if = "...")]
///     #[spec(fieldName)]
///     field: Type,
///   }
/// }
/// ```
///
/// get expanded to:
///
/// ```
/// /// bla, bla, desc
/// #[derive(Debug, Derserialize, Serialize)]
/// struct Pass {
///   #[serde(default, skip_serializing_if = "...")]
///   field: Type
/// }
///
/// mod spec {
///   #[derive(Derserialize, Serialize)]
///   struct Pass {
///     #[serde(default, skip_serializing_if = "...")]
///     #[serde(rename = "fieldName")]
///     field_name: Type
///   }
/// }
/// ```
#[proc_macro_attribute]
pub fn spec(
	args: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	if !args.is_empty() {
		let args: proc_macro2::TokenStream = args.into();
		let error =
			syn::Error::new_spanned(args, "`spec` doesn't take arguments").to_compile_error();
		return error.into();
	}

	spec::spec(input.into()).into()
}
