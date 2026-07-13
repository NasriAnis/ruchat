use serde::Deserialize;

// Deserialize an instance of type T from bytes of JSON text
pub fn json_from_slice<'a, T: Deserialize<'a>>(_t: T, json: &'a [u8]) -> Result<T, serde_json::Error> {
    let t = match serde_json::from_slice(json){
        Ok(t) => t,
        Err(e) => return Err(e),
    };
    Ok(t)
}
