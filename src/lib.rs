use koan::{error::handle_err, lexer::lex, parser::parse, state};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Output {
    stdout: String,
    result: String,
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
pub fn run_line(line: String, state: WState) -> Result<Output, String> {
    let mut statements = lex(&line).and_then(parse).map_err(handle_err)?;
    let mut stdout: Vec<u8> = vec![];
    let mut state = state.0;

    let st = statements.pop().unwrap();
    let val = st.eval(&mut state, &mut stdout).map_err(handle_err)?;
    let result = format!("{val}");

    Ok(Output {
        stdout: String::from_utf8(stdout)
            .map_err(|_| "Value could not be converted to valid UTF-8")?,
        result,
    })
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, koan-wasm-wrapper!");
}
