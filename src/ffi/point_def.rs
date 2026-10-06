use crate::{base_provider_context::BaseProviderContext, point_definition::PointDefinitionLike};

mod base;
mod float;
mod quat;
mod vec3;
mod vec4;

/// Shared body of the `tracks_interpolate_*_batch` functions: writes the value at `times[i]` to `out[i]`.
/// Null pointers or `len == 0` are a no-op.
///
/// # Safety
/// - `point_definition` and `context` must be valid pointers when non-null.
/// - `times` must point to `len` readable `f32`s, and `out` to `len` writable `W`s. `out` may be uninitialised.
unsafe fn interpolate_batch<D, T, W>(
    point_definition: *const D,
    times: *const f32,
    out: *mut W,
    len: usize,
    context: *const BaseProviderContext,
) where
    D: PointDefinitionLike<T>,
    T: Default + Clone,
    W: From<T>,
{
    if point_definition.is_null()
        || times.is_null()
        || out.is_null()
        || context.is_null()
        || len == 0
    {
        return;
    }

    let (point_definition, context) = unsafe { (&*point_definition, &*context) };
    let times = unsafe { std::slice::from_raw_parts(times, len) };
    for (i, &time) in times.iter().enumerate() {
        let value = point_definition.interpolate(time, context).0;
        // write instead of assigning through a slice, since `out` may be uninitialised
        unsafe { out.add(i).write(value.into()) };
    }
}

#[cfg(all(test, feature = "json"))]
mod tests {
    use std::mem::MaybeUninit;

    use glam::{Quat, Vec3, Vec4};
    use serde_json::json;

    use super::{float, quat, vec3, vec4};
    use crate::{
        animation::property::PathProperty,
        base_value::{BaseValue, WrapBaseValueType},
        ffi::{
            property::path_property_interpolate_batch,
            types::{WrapBaseValue, WrapQuat, WrapVec3, WrapVec4},
        },
        prelude::BaseProviderContext,
        test_helpers::*,
    };

    const TIMES: [f32; 8] = [-1.0, 0.0, 0.1, 0.4, 0.5, 0.75, 1.0, 2.0];

    /// Runs an FFI batch function into an uninitialised buffer, like a host would.
    fn run_ffi<W>(f: impl FnOnce(*const f32, *mut W, usize)) -> Vec<W> {
        let mut out: Vec<MaybeUninit<W>> =
            (0..TIMES.len()).map(|_| MaybeUninit::uninit()).collect();
        f(TIMES.as_ptr(), out.as_mut_ptr().cast(), TIMES.len());
        out.into_iter()
            .map(|v| unsafe { v.assume_init() })
            .collect()
    }

    #[test]
    fn float_batch_matches_single() {
        let mut ctx = BaseProviderContext::new();
        let def = parse_float_point_definition(
            json!([[0.0, 0.0], [10.0, 0.5, "easeInQuad"], [4.0, 1.0]]),
            &mut ctx,
        );
        let expected: Vec<f32> = TIMES
            .iter()
            .map(|&t| interpolate_float_point_definition(&def, t, &ctx).0)
            .collect();

        let mut rust_out = [0.0; TIMES.len()];
        crate::prelude::PointDefinitionLike::interpolate_batch(&def, &TIMES, &mut rust_out, &ctx);
        assert_eq!(rust_out.as_slice(), expected.as_slice());

        let ffi_out = run_ffi(|t, o, n| unsafe {
            float::tracks_interpolate_float_batch(&def, t, o, n, &ctx)
        });
        assert_eq!(ffi_out, expected);
    }

    #[test]
    fn vector3_batch_matches_single() {
        let mut ctx = BaseProviderContext::new();
        // spline points use their neighbours, so check them too
        let def = parse_vector3_point_definition(
            json!([
                [0.0, 0.0, 0.0, 0.0],
                [1.0, 2.0, 3.0, 0.5, "splineCatmullRom"],
                [4.0, 0.0, -1.0, 1.0, "easeOutSine"]
            ]),
            &mut ctx,
        );
        let expected: Vec<Vec3> = TIMES
            .iter()
            .map(|&t| interpolate_vector3_point_definition(&def, t, &ctx).0)
            .collect();

        let ffi_out = run_ffi::<WrapVec3>(|t, o, n| unsafe {
            vec3::tracks_interpolate_vector3_batch(&def, t, o, n, &ctx)
        });
        let ffi_out: Vec<Vec3> = ffi_out.iter().map(|v| Vec3::new(v.x, v.y, v.z)).collect();
        assert_eq!(ffi_out, expected);
    }

    #[test]
    fn vector4_and_quat_batch_match_single() {
        let mut ctx = BaseProviderContext::new();
        let v4 = parse_vector4_point_definition(
            json!([[0.0, 0.0, 0.0, 0.0, 0.0], [1.0, 2.0, 3.0, 4.0, 1.0]]),
            &mut ctx,
        );
        let expected: Vec<Vec4> = TIMES
            .iter()
            .map(|&t| interpolate_vector4_point_definition(&v4, t, &ctx).0)
            .collect();
        let out = run_ffi::<WrapVec4>(|t, o, n| unsafe {
            vec4::tracks_interpolate_vector4_batch(&v4, t, o, n, &ctx)
        });
        assert_eq!(
            out.iter()
                .map(|v| Vec4::new(v.x, v.y, v.z, v.w))
                .collect::<Vec<_>>(),
            expected
        );

        let q = parse_quaternion_point_definition(
            json!([[0.0, 0.0, 0.0, 0.0], [0.0, 90.0, 45.0, 1.0]]),
            &mut ctx,
        );
        let expected: Vec<Quat> = TIMES
            .iter()
            .map(|&t| interpolate_quaternion_point_definition(&q, t, &ctx).0)
            .collect();
        let out = run_ffi::<WrapQuat>(|t, o, n| unsafe {
            quat::tracks_interpolate_quat_batch(&q, t, o, n, &ctx)
        });
        assert_eq!(
            out.iter()
                .map(|v| Quat::from_xyzw(v.x, v.y, v.z, v.w))
                .collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn path_batch_matches_single() {
        let mut ctx = BaseProviderContext::new();
        let prev = parse_vector3_point_definition(
            json!([[0.0, 0.0, 0.0, 0.0], [1.0, 1.0, 1.0, 1.0]]),
            &mut ctx,
        );
        let point = parse_vector3_point_definition(
            json!([[5.0, 0.0, 0.0, 0.0], [0.0, 5.0, 0.0, 1.0]]),
            &mut ctx,
        );

        let mut path = PathProperty::empty(WrapBaseValueType::Vec3);
        let empty = run_ffi::<WrapBaseValue>(|t, o, n| {
            assert!(!unsafe { path_property_interpolate_batch(&path, t, o.cast(), n, &ctx) });
            // nothing was written, so fill the buffer to keep `run_ffi` sound
            for i in 0..n {
                unsafe { o.add(i).write(BaseValue::Float(0.0).into()) };
            }
        });
        assert_eq!(empty.len(), TIMES.len());

        path.init(Some(prev.into()));
        path.init(Some(point.into()));
        path.interpolate_time = 0.25;
        let expected: Vec<Option<BaseValue>> =
            TIMES.iter().map(|&t| path.interpolate(t, &ctx)).collect();

        let mut rust_out = [BaseValue::default(); TIMES.len()];
        assert!(path.interpolate_batch(&TIMES, &mut rust_out, &ctx));
        assert_eq!(rust_out.map(Some).as_slice(), expected.as_slice());

        let ffi_out = run_ffi::<WrapBaseValue>(|t, o, n| {
            assert!(unsafe { path_property_interpolate_batch(&path, t, o, n, &ctx) });
        });
        let ffi_out: Vec<Option<BaseValue>> = ffi_out.into_iter().map(|v| Some(v.into())).collect();
        assert_eq!(ffi_out, expected);
    }
}
