use serenity::prelude::*;
use std::marker::PhantomData;

pub struct DataContainerKey<T> {
    value_type: PhantomData<T>,
}

impl<T: 'static> TypeMapKey for DataContainerKey<T>
where
    T: 'static,
{
    type Value = T;
}

impl<T> DataContainerKey<T>
where
    Self: TypeMapKey,
    <Self as TypeMapKey>::Value: Send + Sync,
{
    #[allow(dead_code)]
    pub fn insert(data: &mut ShareMap, value: <Self as TypeMapKey>::Value) {
        data.insert::<Self>(value);
    }

    #[allow(dead_code)]
    pub fn remove(data: &mut ShareMap) -> Option<<Self as TypeMapKey>::Value> {
        data.remove::<Self>()
    }

    #[allow(dead_code)]
    pub fn contains(data: &mut ShareMap) -> bool {
        data.contains::<Self>()
    }

    #[allow(dead_code)]
    pub fn get(data: &ShareMap) -> Option<&<Self as TypeMapKey>::Value> {
        data.get::<Self>()
    }

    #[allow(dead_code)]
    pub fn get_mut(data: &mut ShareMap) -> Option<&mut <Self as TypeMapKey>::Value> {
        data.get_mut::<Self>()
    }
}
