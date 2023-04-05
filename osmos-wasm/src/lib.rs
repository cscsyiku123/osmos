#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hello() -> String {
    "Hello, world!".to_string()
}

#[derive(Default)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct Simulator {
    simulator: osmos_sim::simulator::Simulator,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Simulator {
    #[wasm_bindgen::prelude::wasm_bindgen(constructor)]
    pub fn new() -> Simulator {
        Default::default()
    }
    #[wasm_bindgen::prelude::wasm_bindgen]
    pub fn get_object_list(&self) -> wasm_bindgen::JsValue {
        let value = self.simulator.object_list.iter().map(Object::from).collect::<Vec<Object>>();
        let value1 = serde_wasm_bindgen::to_value(&value).unwrap();
        println!("value1: {:?}", value1);
        return value1;
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Object {
    pub x: f64,
    pub y: f64,
    pub energy: usize,
}

impl From<&osmos_sim::object::Object> for Object {
    fn from(object: &osmos_sim::object::Object) -> Object {
        Object {
            x: object.cell.position.x,
            y: object.cell.position.y,
            energy: object.cell.energy,
        }
    }
}
