use crate::{
    base_provider_context::BaseProviderContext,
    easings::functions::Functions,
    modifiers::{ModifierLike, ModifierValues, modifier::BasicModifier, operation::Operation},
    value_types::ValueType,
};

use super::PointDataLike;

#[derive(Debug, Clone)]
pub struct BasicPointData<T: ValueType> {
    base_modifier: BasicModifier<T>,
    pub smooth: bool,
    easing: Functions,
    time: f32,
}

impl<T: ValueType> BasicPointData<T>
where
    [(); T::VALUE_COUNT]:,
{
    pub fn new(
        point: ModifierValues<T>,
        time: f32,
        smooth: bool,
        modifiers: Vec<BasicModifier<T>>,
        easing: Functions,
    ) -> Self {
        Self {
            base_modifier: BasicModifier::new(point, modifiers, Operation::None),
            smooth,
            easing,
            time,
        }
    }
}

// impl<T: ValueType> ModifierLike for BasicPointData<T> {
//     const VALUE_COUNT: usize = T::VALUE_COUNT;

//     fn get_modified_point(&self, context: &BaseProviderContext) -> T {
//         self.base_modifier.get_modified_point(context)
//     }

//     fn get_raw_point(&self) -> T {
//         self.base_modifier.get_raw_point()
//     }

//     fn translate(&self, values: &[f32]) -> T {
//         self.base_modifier.translate(values)
//     }

//     fn get_operation(&self) -> Operation {
//         self.base_modifier.get_operation()
//     }

//     fn has_base_provider(&self) -> bool {
//         self.base_modifier.has_base_provider()
//     }
// }

impl<T: ValueType> PointDataLike<T> for BasicPointData<T>
where
    [(); T::VALUE_COUNT]:,
{
    fn get_easing(&self) -> Functions {
        self.easing
    }

    fn get_time(&self) -> f32 {
        self.time
    }
    fn has_base_provider(&self) -> bool {
        self.base_modifier.has_base_provider()
    }

    fn get_point(&self, context: &BaseProviderContext) -> T {
        self.base_modifier.get_modified_point(context)
    }
}
