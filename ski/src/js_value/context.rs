use super::JsFunc;
use crate::repository::{clear_history, get_context, push_history_def, push_history_del};
use tuber::{parse_update_or_delete, Command, Context};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Context)]
pub struct JsContext(Context);

#[wasm_bindgen(js_class = Context)]
impl JsContext {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        JsContext(get_context().unwrap())
    }

    pub fn default() -> Self {
        JsContext(Context::default())
    }

    pub fn def(&mut self, func: &str) -> bool {
        let command = parse_update_or_delete(func).unwrap();
        match command {
            Command::Update(func) => {
                push_history_def(&func).unwrap();
                self.0.def(func);
                true
            }
            Command::Del(id) => {
                push_history_del(&id).unwrap();
                self.0.del(&id);
                true
            }
            _ => false,
        }
    }

    pub fn get(&self, id: &str) -> Option<JsFunc> {
        self.0.get(&id.into()).map(|func| func.to_owned().into())
    }

    #[wasm_bindgen(js_name = getAll)]
    pub fn get_all(&self) -> Box<[JsFunc]> {
        self.0
            .clone()
            .to_vec()
            .into_iter()
            .map(|func| func.into())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn reset(&self) -> Result<(), JsError> {
        clear_history().map_err(|err| JsError::new(err.to_string().as_str()))
    }

    #[wasm_bindgen(js_name = deleteAll)]
    pub fn delete_all(&mut self) -> Result<(), JsError> {
        for (id, _) in self.0.iter() {
            push_history_del(id).map_err(|err| JsError::new(err.to_string().as_str()))?;
        }
        Ok(())
    }
}

impl From<Context> for JsContext {
    fn from(context: Context) -> JsContext {
        JsContext(context)
    }
}

impl From<JsContext> for Context {
    fn from(js_context: JsContext) -> Context {
        js_context.0
    }
}
