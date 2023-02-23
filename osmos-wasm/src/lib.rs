#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hello() -> String {
    "你好，杀破狼，杀杀杀".to_string()
}

#[derive(Default)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct Simulator {
    simulator: osmos_sim::simulator::Simulator,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Simulator {
    #[wasm_bindgen::prelude::wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    #[wasm_bindgen::prelude::wasm_bindgen(js_name=getObjectList)]
    pub fn get_object_list(&self) -> wasm_bindgen::JsValue {
        let value = self
            .simulator
            .object_list
            .iter()
            .map(Object::from)
            .collect::<Vec<Object>>();
        // serde_wasm_bindgen::to_value(value).unwrap()
        serde_wasm_bindgen::to_value(&value).unwrap()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Object {
    pub x: f64,
    pub y: f64,
    pub energy: usize,
}

impl From<&osmos_sim::object::Object> for Object {
    fn from(object: &osmos_sim::object::Object) -> Self {
        Self {
            x: object.cell.position.x,
            y: object.cell.position.y,
            energy: object.cell.energy,
        }
    }
}
