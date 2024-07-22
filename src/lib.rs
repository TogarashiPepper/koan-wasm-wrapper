use koan::{error::handle_err, interpreter::IntrpCtx, lexer::lex, parser::parse, state};
use wasm_bindgen::prelude::*;
use std::panic;

#[wasm_bindgen]
pub struct Output {
    stdout: String,
    result: String,
}

#[wasm_bindgen]
impl Output {
    #[wasm_bindgen]
    pub fn stdout(&self) -> String {
        self.stdout.clone()
    }

    #[wasm_bindgen]
    pub fn result(&self) -> String {
        self.result.clone()
    }
}

#[wasm_bindgen]
pub struct WState(state::State);

#[wasm_bindgen]
impl WState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(state::State::new())
    }
}

#[wasm_bindgen]
pub fn run_line(line: String, state: &mut WState) -> Result<Output, String> {
    console_error_panic_hook::set_once();

    let (mut statements, pool) = lex(&line).and_then(parse).map_err(handle_err)?;
    let mut stdout: Vec<u8> = vec![];
    let mut ctx = IntrpCtx {
        writer: &mut stdout,
        state: &mut state.0,
        pool: &pool,
    };

    let st = statements.pop().unwrap();
    let val = ctx.eval_ast(st).map_err(handle_err)?;
    let result = format!("{val}");

    Ok(Output {
        stdout: String::from_utf8(stdout)
            .map_err(|_| "Value could not be converted to valid UTF-8")?,
        result,
    })
}
