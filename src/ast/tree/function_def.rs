use log::debug;
use proc_macro2::TokenStream;
use pyo3::FromPyObject;
use quote::{ format_ident, quote };
use serde::{ Deserialize, Serialize };

use crate::{
    CodeGen,
    CodeGenContext,
    ExprType,
    Object,
    ParameterList,
    PythonOptions,
    Statement,
    StatementType,
    SymbolTableNode,
    SymbolTableScopes,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FunctionDef {
    pub name: String,
    pub args: ParameterList,
    pub body: Vec<Statement>,
    pub decorator_list: Vec<String>,
}

impl<'py> ::pyo3::FromPyObject<'py> for FunctionDef {
    fn extract_bound(obj: &::pyo3::Bound<'py, ::pyo3::PyAny>) -> ::pyo3::PyResult<Self> {
        ::std::result::Result::Ok(FunctionDef {
            name: ::pyo3::impl_::frompyobject::extract_struct_field(
                &::pyo3::types::PyAnyMethods::getattr(obj, ::pyo3::intern!(obj.py(), "name"))?,
                "FunctionDef",
                "name"
            )?,
            args: ::pyo3::impl_::frompyobject::extract_struct_field(
                &::pyo3::types::PyAnyMethods::getattr(obj, ::pyo3::intern!(obj.py(), "args"))?,
                "FunctionDef",
                "args"
            )?,
            body: ::pyo3::impl_::frompyobject::extract_struct_field(
                &::pyo3::types::PyAnyMethods::getattr(obj, ::pyo3::intern!(obj.py(), "body"))?,
                "FunctionDef",
                "body"
            )?,
            decorator_list: vec![],
        })
    }
}

impl CodeGen for FunctionDef {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        let mut symbols = symbols;
        symbols.insert(self.name.clone(), SymbolTableNode::FunctionDef(self.clone()));
        symbols
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: SymbolTableScopes
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut streams = TokenStream::new();
        let fn_name = format_ident!("{}", self.name);

        // The Python convention is that functions that begin with a single underscore,
        // it's private. Otherwise, it's public. We formalize that by default.
        let visibility = if self.name.starts_with("_") && !self.name.starts_with("__") {
            format_ident!("")
        } else if self.name.starts_with("__") && self.name.ends_with("__") {
            format_ident!("pub(crate)")
        } else {
            format_ident!("pub")
        };

        let is_async = match ctx.clone() {
            CodeGenContext::Async(_) => { quote!(async) }
            _ => quote!(),
        };

        let parameters = self.args
            .clone()
            .to_rust(ctx.clone(), options.clone(), symbols.clone())
            .expect(format!("parsing arguments {:?}", self.args).as_str());

        for s in self.body.iter() {
            streams.extend(
                s
                    .clone()
                    .to_rust(ctx.clone(), options.clone(), symbols.clone())
                    .expect(format!("parsing statement {:?}", s).as_str())
            );
            streams.extend(quote!(;));
        }

        let docstring = if let Some(d) = self.get_docstring() {
            format!("{}", d)
        } else {
            "".to_string()
        };

        let function =
            quote! {
            #[doc = #docstring]
            #visibility #is_async fn #fn_name(#parameters) {
                #streams
            }
        };

        debug!("function: {}", function);
        Ok(function)
    }

    fn get_docstring(&self) -> Option<String> {
        let expr = self.body[0].clone();
        match expr.statement {
            StatementType::Expr(e) =>
                match e.value {
                    ExprType::Constant(c) => Some(c.to_string()),
                    _ => None,
                }
            _ => None,
        }
    }
}

impl Object for FunctionDef {}
