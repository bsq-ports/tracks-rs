use super::{Modifier, ModifierBase, operation::Operation, shared_has_base_provider};
use crate::{
    base_provider_context::BaseProviderContext,
    values::{AbstractValueProvider, ValueProvider},
};
use glam::{EulerRot, Quat, Vec3};

/// Is it ZXY or XYZ???
/// Is it just XYZ because it's the default?
pub const TRACKS_EULER_ROT: EulerRot = EulerRot::XYZ;

#[derive(Debug)]
pub enum QuaternionValues {
    // equivalents but different repr
    Static(Vec3, Quat),
    Dynamic(Vec<ValueProvider>),
}

#[derive(Debug)]
pub struct QuaternionModifier {
    values: QuaternionValues,
    has_base_provider: bool,
    modifiers: Vec<Modifier>,
    operation: Operation,
}

impl QuaternionModifier {
    pub fn new(point: QuaternionValues, modifiers: Vec<Modifier>, operation: Operation) -> Self {
        let has_base_provider =
            shared_has_base_provider(matches!(point, QuaternionValues::Dynamic(_)), &modifiers);
        Self {
            values: point,
            has_base_provider,
            modifiers,
            operation,
        }
    }

    fn translate_euler(values: &[ValueProvider], context: &BaseProviderContext) -> Vec3 {
        let mut vec3 = Vec3::ZERO;

        values
            .iter()
            .flat_map(|x| x.values(context).iter().copied().collect::<Vec<_>>())
            .take(Self::VALUE_COUNT)
            .enumerate()
            .for_each(|(i, v)| vec3[i] = v);

        vec3
    }

    pub fn get_vector_point(&self, context: &BaseProviderContext) -> Vec3 {
        let original_point = match &self.values {
            QuaternionValues::Static(s, _) => *s,
            QuaternionValues::Dynamic(value_providers) => {
                Self::translate_euler(value_providers, context)
            }
        };
        self.modifiers.iter().fold(original_point, |acc, x| {
            let Modifier::Quaternion(quat_point) = x else {
                panic!("Invalid modifier type");
            };
            match x.get_operation() {
                Operation::Add => acc + quat_point.get_vector_point(context),
                Operation::Sub => acc - quat_point.get_vector_point(context),
                Operation::Mul => acc * quat_point.get_vector_point(context),
                Operation::Div => acc / quat_point.get_vector_point(context),
                Operation::None => quat_point.get_vector_point(context),
            }
        })
    }
}

impl ModifierBase for QuaternionModifier {
    type Value = Quat;
    const VALUE_COUNT: usize = 3;

    fn get_point(&self, context: &BaseProviderContext) -> Quat {
        if self.modifiers.is_empty() && matches!(self.values, QuaternionValues::Static(_, _)) {
            return self.get_raw_point();
        }
        // modifiers applied to the point
        let vector_point = self.get_vector_point(context);

        Quat::from_euler(
            TRACKS_EULER_ROT,
            vector_point.x.to_radians(),
            vector_point.y.to_radians(),
            vector_point.z.to_radians(),
        )
    }

    fn get_raw_point(&self) -> Quat {
        match self.values {
            QuaternionValues::Static(_, q) => q,
            _ => Quat::IDENTITY,
        }
    }

    fn translate(&self, values: &[f32]) -> Quat {
        Quat::from_euler(
            TRACKS_EULER_ROT,
            values[0].to_radians(),
            values[1].to_radians(),
            values[2].to_radians(),
        )
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }

    fn has_base_provider(&self) -> bool {
        self.has_base_provider
    }
}
