use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Api<T> {
    pub code: Option<u16>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> Api<T> {
    pub fn get_obj( &self) -> Option<T>
    where
        T: Serialize + Deserialize<'static> + Clone,
    {
        self.data.clone()
    }
    pub fn get_code(&self)-> u16 {
        self.code.unwrap()
    }

    pub fn response_err() ->Self{
        Api{ code: None, msg: Some("request failed".to_string()), data: None }
    }
}

impl<T> From<String> for Api<T>
where
    T: Serialize + DeserializeOwned + ToString,
{
    fn from(value: String) -> Self {
        serde_json::from_str::<Api<T>>(&value).unwrap()
    }
}
