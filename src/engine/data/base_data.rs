use serde::Serialize;

use super::BaseDataBuilder;

#[derive(Debug, Default, Serialize)]
pub struct BaseData<T: Serialize> {
    #[serde(flatten)]
    pub base_data: BaseDataBuilder,
    #[serde(flatten)]
    pub child_data: T,
}

impl<T> BaseData<T>
where
    T: Serialize,
{
    pub fn into_base_data(self) -> BaseDataBuilder {
        self.base_data
    }
}
