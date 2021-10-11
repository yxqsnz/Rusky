#![feature(proc_macro_diagnostic)]
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result as ParseResult},
    parse_macro_input,
    spanned::Spanned,
    AttributeArgs,
    ItemFn,
    Lit,
    Meta,
    NestedMeta,
};
/// parse attribute
macro_rules! parse_attr {
    ($meta:ident,
    $(
        $attr_name:expr => $target:ident$(,)?
    )*
    ) => {
        match $meta {
            Meta::List(ref meta) => {
                match meta
                    .path
                    .get_ident()
                    .expect("expected a ident")
                    .to_string()
                    .as_str()
                {
                    $(
                        $attr_name => {
                            parse_attr!(?parse_list meta, $target);
                        }
                    )*
                    attr => panic!("unexpected attr: {}", attr),
                }
            }
            Meta::NameValue(ref meta) => match meta
                .path
                .get_ident()
                .expect("expected a ident.")
                .to_string()
                .as_str()
            {
                $(
                    $attr_name => {
                        parse_attr!(?parse_name_value, meta, $target);
                    }
                )*
                attr => {
                    panic!("unknown attribute: {}", attr)
                }
            },
            _ => (),
        }
    };
    (?parse_name_value, $meta:ident, $i:ident) => {
        let lit = &$meta.lit;
        match lit {
            Lit::Str(s) => $i = Some(s.value()),
            lit => {
                panic!("unexpected literal: {:?}", lit);
            }
        }
    };
    (?parse_list $meta:ident, $i:ident) => {
        for meta in &$meta.nested {
            match meta {
                NestedMeta::Lit(lit) => match lit {
                    Lit::Str(str) => {
                        $i = Some(str.value());
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    };
}
struct CommandMacro {
    pub name: String,
    pub description: Option<String>,
    pub function: ItemFn,
}
impl Parse for CommandMacro {
    fn parse(tokens: ParseStream) -> ParseResult<Self> {
        let function: ItemFn = tokens.parse()?;
        let f = function.clone();
        let mut name = Some(function.sig.ident.to_string());

        let mut description: Option<String> = None;
        for attr in function.attrs {
            let meta = attr.parse_meta()?;
            parse_attr!(meta,
                "description" => description,
                "name" => name,
            );
        }

        Ok(Self {
            name: name.unwrap(),
            description,
            function: f.clone(),
        })
    }
}

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    let mut command = parse_macro_input!(input as CommandMacro);
    for arg in args {
        match arg {
            NestedMeta::Lit(Lit::Str(s)) => {
                command.name = s.value();
            }
            _ => (),
        }
    }

    let mut f = command.function;
    f.attrs.clear();
    let name = command.name;
    let d = command.description;
    let f_name = f.sig.ident.clone();
    let f_name_s = f_name.to_string();
    let s_command_name = format_ident!("{}_COMMAND", name.to_uppercase());
    if matches!(name.to_lowercase().as_str(), "test" | "foo") {
        f.sig
            .span()
            .unwrap()
            .warning("come on, pick a more creative name")
            .help("consider using a more creative name")
            .emit()
    }
    quote! {
     // yes.
     macro_rules! __convert_os_baguios_pra_option_pq_o_caralho_do_quote_nao_mantem_o_caralho_do_option {
        (()) => (None);
        ($e:expr) => (Some($e.to_string()));
     }
     pub struct #s_command_name;
     #[serenity::async_trait]
     impl crate::commands::SlashCommand for #s_command_name {
        fn data(&self) -> crate::commands::SlashCommandData {
            crate::commands::SlashCommandData {
                name: #name.to_string(),
                description: __convert_os_baguios_pra_option_pq_o_caralho_do_quote_nao_mantem_o_caralho_do_option!(#d),
                // TODO: options attribute.
                options: None,
            }
        }
        async fn execute(&self, c: &crate::commands::CommandContext) -> crate::Result<()> {
            tracing::info!("executing function: {} at command {}", #f_name_s,self.data().name);
            let r = #f_name(c).await?;
            Ok(r)
        }   
     }
     // vo deixar aq quieto mesmo.
     #f
    }
    .into()
}
