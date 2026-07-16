use anyhow::Result;
use std::sync::Arc;
use swc_common::comments::SingleThreadedComments;
use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

use crate::UraiContext;
use crate::ast::CodeAnalyze;

impl CodeAnalyze {
    pub fn new(ctx: Arc<UraiContext>, content: &str, file_name: &str) -> Self {
        Self {
            ctx,
            file_name: file_name.to_string(),
            content: content.to_string(),
        }
    }
    pub fn parse_file(&self) -> Result<(Module, SingleThreadedComments, Lrc<SourceMap>)> {
        let file_name = &self.file_name;
        let content = &self.content;
        let code_map: Lrc<SourceMap> = Default::default();
        let file_map = code_map.new_source_file(
            FileName::Custom(file_name.to_string()).into(),
            content.to_string(),
        );

        let is_ts = file_name.ends_with(".ts") || file_name.ends_with(".tsx");
        let is_jsx = file_name.ends_with(".jsx") || file_name.ends_with(".tsx");

        let syntax = if is_ts {
            Syntax::Typescript(TsSyntax {
                tsx: is_jsx,
                ..Default::default()
            })
        } else {
            Syntax::Es(EsSyntax {
                jsx: is_jsx,
                ..Default::default()
            })
        };

        let comments = SingleThreadedComments::default();

        let lexer = Lexer::new(
            syntax,
            Default::default(),
            StringInput::from(&*file_map),
            Some(&comments),
        );
        let mut parser = Parser::new_from(lexer);
        let module = parser
            .parse_module()
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;

        Ok((module, comments, code_map))
    }
}

pub fn parse_file(
    content: &str,
    file_name: &str,
) -> Result<(Module, SingleThreadedComments, Lrc<SourceMap>)> {
    let code_map: Lrc<SourceMap> = Default::default();
    let file_map = code_map.new_source_file(
        FileName::Custom(file_name.to_string()).into(),
        content.to_string(),
    );

    let is_ts = file_name.ends_with(".ts") || file_name.ends_with(".tsx");
    let is_jsx = file_name.ends_with(".jsx") || file_name.ends_with(".tsx");

    let syntax = if is_ts {
        Syntax::Typescript(TsSyntax {
            tsx: is_jsx,
            ..Default::default()
        })
    } else {
        Syntax::Es(EsSyntax {
            jsx: is_jsx,
            ..Default::default()
        })
    };

    let comments = SingleThreadedComments::default();

    let lexer = Lexer::new(
        syntax,
        Default::default(),
        StringInput::from(&*file_map),
        Some(&comments),
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser
        .parse_module()
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;

    Ok((module, comments, code_map))
}
