use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, spanned::Spanned, LitStr, Visibility};

#[derive(Default)]
struct Errors {
	list: Vec<syn::Error>,
}

impl Errors {
	fn error(&mut self, span: &impl Spanned, message: impl Into<String>) {
		self.list.push(syn::Error::new(span.span(), message.into()));
	}
}

pub fn spec(input: TokenStream) -> TokenStream {
	let (mut output, errors) = match symbols_with_errors(input) {
		Ok(ok) => ok,
		Err(err) => return err.to_compile_error(),
	};

	// If we generated any errors, then report them as compiler_error!() macro calls.
	// This lets the errors point back to the most relevant span. It also allows us
	// to report as many errors as we can during a single run.
	output.extend(errors.into_iter().map(|e| e.to_compile_error()));

	output
}

fn symbols_with_errors(input: TokenStream) -> syn::Result<(TokenStream, Vec<syn::Error>)> {
	let mut errors = Errors::default();

	let inner_mod: syn::ItemMod = syn::parse2(input)?;

	let mod_name = inner_mod.ident.clone();

	let (_, original_items) = inner_mod
		.content
		.ok_or_else(|| syn::Error::new_spanned(inner_mod.ident, "target mod needs to be inline"))?;

	let (lib_items, spec_items) = process_items(original_items, &mut errors)?;

	let output = quote! {
		pub use self::#mod_name::*;
		#[doc(hidden)]
		mod #mod_name {
			#(#lib_items)*
			pub mod spec {
				#(#spec_items)*
			}
		}
	};

	Ok((output, errors.list))
}

fn process_items(
	items: Vec<syn::Item>,
	errors: &mut Errors,
) -> syn::Result<(Vec<syn::Item>, Vec<syn::Item>)> {
	let mut lib_items = vec![];
	let mut spec_items = vec![];

	for item in items {
		match item {
			// Required for hygiene? Pass to both lib and spec
			syn::Item::Use(use_) => {
				lib_items.push(syn::Item::Use(use_.clone()));
				spec_items.push(syn::Item::Use(use_));
			}

			// These items implement `Serialize`/`Deserialize`
			syn::Item::Struct(struct_) => {
				let model = Model::new(struct_);
				lib_items.push(model.clone().into_lib_item()?);
				spec_items.push(model.into_spec_item()?);
			}
			syn::Item::Enum(enum_) => {
				let union = Union::new(enum_);
				lib_items.push(union.clone().into_lib_item()?);
				spec_items.push(union.into_spec_item()?);
			}

			// Only pass custom `Serialize`/`Deserialize` impls to `spec`.
			syn::Item::Impl(impl_) => {
				lib_items.push(syn::Item::Impl(impl_.clone()));

				if let Some((_, trait_path, _)) = &impl_.trait_ {
					// `std::foo::Bar` gets stored vec![Bar, foo, std]
					if let Some(path_seg) = trait_path.segments.get(0) {
						if path_seg.ident == "Deserialize" || path_seg.ident == "Serialize" {
							spec_items.push(syn::Item::Impl(impl_));
						}
					};
				}
			}

			item => errors.error(&item, "item type not supported"),
		}
	}

	Ok((lib_items, spec_items))
}

#[derive(Clone)]
struct Model {
	struct_: syn::ItemStruct,
}

impl Model {
	const fn new(struct_: syn::ItemStruct) -> Self {
		Self { struct_ }
	}

	fn into_lib_item(mut self) -> syn::Result<syn::Item> {
		let attrs = self.struct_.attrs.drain(..).collect::<Vec<_>>();
		for attr in attrs {
			match &attr.meta {
				syn::Meta::NameValue(name_value)
					if name_value.path.get_ident().unwrap() == "doc" =>
				{
					self.struct_.attrs.push(attr);
				}

				syn::Meta::List(list) if list.path.get_ident().unwrap() == "spec" => {}

				syn::Meta::List(list) if list.path.get_ident().unwrap() == "serde" => {
					self.struct_.attrs.push(attr);
				}

				syn::Meta::List(list) if list.path.get_ident().unwrap() == "derive" => {
					self.struct_.attrs.push(attr);
				}

				_ => return Err(syn::Error::new_spanned(attr, "unsupported attr")),
			}
		}

		for field in &mut self.struct_.fields {
			let field_attrs = field.attrs.drain(..).collect::<Vec<_>>();
			for field_attr in field_attrs {
				match &field_attr.meta {
					syn::Meta::NameValue(name_value)
						if name_value.path.get_ident().unwrap() == "doc" =>
					{
						field.attrs.push(field_attr);
					}

					syn::Meta::List(list) if list.path.get_ident().unwrap() == "spec" => {}

					syn::Meta::List(list) if list.path.get_ident().unwrap() == "serde" => {
						field.attrs.push(field_attr.clone());
					}

					_ => return Err(syn::Error::new_spanned(field_attr, "unsupported attr")),
				}
			}
		}

		Ok(syn::Item::Struct(self.struct_))
	}

	fn into_spec_item(mut self) -> syn::Result<syn::Item> {
		self.struct_.vis = Visibility::Inherited;

		let attrs = self.struct_.attrs.drain(..).collect::<Vec<_>>();
		for attr in attrs {
			match &attr.meta {
				syn::Meta::NameValue(name_value)
					if name_value.path.get_ident().unwrap() == "doc" =>
				{
					// Don't need to document spec items
				}

				syn::Meta::List(list) if list.path.get_ident().unwrap() == "spec" => {
					list.parse_nested_meta(|meta| {
						if meta.path.get_ident().unwrap() == "pub" {
							self.struct_.vis =
								Visibility::Public((syn::Token![pub])(Span::mixed_site()));
						}

						Ok(())
					})?;
				}

				syn::Meta::List(list) if list.path.get_ident().unwrap() == "serde" => {
					// TODO: check for presence of serde `rename_all`

					self.struct_.attrs.push(attr.clone());
				}

				syn::Meta::List(list) if list.path.get_ident().unwrap() == "derive" => {
					let mut derives = vec![];

					list.parse_nested_meta(|meta| {
						if meta.path.is_ident("Serialize") || meta.path.is_ident("Deserialize") {
							derives.push(meta.path.get_ident().cloned().unwrap());
						}

						Ok(())
					})?;

					self.struct_
						.attrs
						.push(parse_quote!(#[derive(#(#derives),*)]));
				}

				_ => return Err(syn::Error::new_spanned(attr, "unsupported attr")),
			}
		}

		for field in &mut self.struct_.fields {
			field.vis = Visibility::Inherited;

			let field_attrs = field.attrs.drain(..).collect::<Vec<_>>();
			for field_attr in field_attrs {
				match &field_attr.meta {
					syn::Meta::NameValue(name_value)
						if name_value.path.get_ident().unwrap() == "doc" =>
					{
						// Don't need to document spec items
					}

					syn::Meta::List(list) if list.path.get_ident().unwrap() == "spec" => {
						list.parse_nested_meta(|meta| {
							let key = meta.path.get_ident().unwrap();
							let key = LitStr::new(&key.to_string(), Span::call_site());
							field.attrs.push(parse_quote! { #[serde(rename = #key)] });
							Ok(())
						})?;
					}

					syn::Meta::List(list) if list.path.get_ident().unwrap() == "serde" => {
						// TODO: check for presence of serde `rename`

						field.attrs.push(field_attr.clone());
					}

					_ => return Err(syn::Error::new_spanned(field_attr, "unsupported attr")),
				}
			}
		}

		Ok(syn::Item::Struct(self.struct_))
	}
}

#[derive(Clone)]
struct Union {
	enum_: syn::ItemEnum,
}

impl Union {
	const fn new(enum_: syn::ItemEnum) -> Self {
		Self { enum_ }
	}

	fn into_lib_item(mut self) -> syn::Result<syn::Item> {
		let attrs = self.enum_.attrs.drain(..).collect::<Vec<_>>();
		for attr in attrs {
			match &attr.meta {
				syn::Meta::NameValue(name_value)
					if name_value.path.get_ident().unwrap() == "doc" =>
				{
					self.enum_.attrs.push(attr);
				}

				syn::Meta::List(list) if list.path.get_ident().unwrap() == "spec" => {}

				syn::Meta::List(list) if list.path.get_ident().unwrap() == "serde" => {
					self.enum_.attrs.push(attr);
				}

				syn::Meta::List(list) if list.path.get_ident().unwrap() == "derive" => {
					self.enum_.attrs.push(attr);
				}

				_ => return Err(syn::Error::new_spanned(attr, "unsupported attr")),
			}
		}

		Ok(syn::Item::Enum(self.enum_))
	}

	fn into_spec_item(mut self) -> syn::Result<syn::Item> {
		self.enum_.vis = Visibility::Inherited;

		let attrs = self.enum_.attrs.drain(..).collect::<Vec<_>>();
		for attr in attrs {
			match &attr.meta {
				syn::Meta::NameValue(name_value)
					if name_value.path.get_ident().unwrap() == "doc" =>
				{
					// Don't need to document spec items
				}

				syn::Meta::List(list) if list.path.get_ident().unwrap() == "spec" => {
					list.parse_nested_meta(|meta| {
						if meta.path.get_ident().unwrap() == "pub" {
							self.enum_.vis =
								Visibility::Public((syn::Token![pub])(Span::mixed_site()));
						}

						Ok(())
					})?;
				}

				syn::Meta::List(list) if list.path.get_ident().unwrap() == "serde" => {
					// TODO: check for presence of serde `rename_all`

					self.enum_.attrs.push(attr.clone());
				}

				syn::Meta::List(list) if list.path.get_ident().unwrap() == "derive" => {
					let mut derives = vec![];

					list.parse_nested_meta(|meta| {
						if meta.path.is_ident("Serialize") || meta.path.is_ident("Deserialize") {
							derives.push(meta.path.get_ident().cloned().unwrap());
						}

						Ok(())
					})?;

					self.enum_
						.attrs
						.push(parse_quote!(#[derive(#(#derives),*)]));
				}

				_ => return Err(syn::Error::new_spanned(attr, "unsupported attr")),
			}
		}

		Ok(syn::Item::Enum(self.enum_))
	}
}
