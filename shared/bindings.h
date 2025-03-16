#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
namespace Tracks {
namespace ffi {
#endif  // __cplusplus

typedef enum Functions {
  EaseLinear,
  EaseStep,
  EaseInQuad,
  EaseOutQuad,
  EaseInOutQuad,
  EaseInCubic,
  EaseOutCubic,
  EaseInOutCubic,
  EaseInQuart,
  EaseOutQuart,
  EaseInOutQuart,
  EaseInQuint,
  EaseOutQuint,
  EaseInOutQuint,
  EaseInSine,
  EaseOutSine,
  EaseInOutSine,
  EaseInCirc,
  EaseOutCirc,
  EaseInOutCirc,
  EaseInExpo,
  EaseOutExpo,
  EaseInOutExpo,
  EaseInElastic,
  EaseOutElastic,
  EaseInOutElastic,
  EaseInBack,
  EaseOutBack,
  EaseInOutBack,
  EaseInBounce,
  EaseOutBounce,
  EaseInOutBounce,
} Functions;

/**
 * JSON FFI
 */
typedef enum JsonValueType {
  Number,
  Null,
  String,
  Array,
} JsonValueType;

typedef enum WrapBaseValueType {
  Vec3 = 0,
  Quat = 1,
  Vec4 = 2,
  Float = 3,
} WrapBaseValueType;

typedef struct BaseFFIProviderValues BaseFFIProviderValues;

typedef struct BasePointDefinition BasePointDefinition;

typedef struct BaseProviderContext BaseProviderContext;

typedef struct CoroutineManager CoroutineManager;

typedef struct EventData EventData;

typedef struct FloatPointDefinition FloatPointDefinition;

typedef struct GameObject GameObject;

typedef struct Option_BaseValue Option_BaseValue;

typedef struct PointDefinitionInterpolation PointDefinitionInterpolation;

typedef struct QuaternionPointDefinition QuaternionPointDefinition;

typedef struct Track Track;

typedef struct TracksContext TracksContext;

typedef struct Vector3PointDefinition Vector3PointDefinition;

typedef struct Vector4PointDefinition Vector4PointDefinition;

typedef struct JsonArray {
  const struct FFIJsonValue *elements;
  uintptr_t length;
} JsonArray;

typedef union JsonValueData {
  double number_value;
  const char *string_value;
  const struct JsonArray *array;
} JsonValueData;

typedef struct FFIJsonValue {
  enum JsonValueType value_type;
  union JsonValueData data;
} FFIJsonValue;

typedef struct WrappedValues {
  const float *values;
  uintptr_t length;
} WrappedValues;

typedef struct WrappedValues (*BaseFFIProvider)(const struct BaseProviderContext*, void*);

typedef struct FloatInterpolationResult {
  float value;
  bool is_last;
} FloatInterpolationResult;

typedef struct WrapVec3 {
  float x;
  float y;
  float z;
} WrapVec3;

typedef struct WrapQuat {
  float x;
  float y;
  float z;
  float w;
} WrapQuat;

typedef struct WrapVec4 {
  float x;
  float y;
  float z;
  float w;
} WrapVec4;

typedef union WrapBaseValueUnion {
  struct WrapVec3 vec3;
  struct WrapQuat quat;
  struct WrapVec4 vec4;
  float float_v;
} WrapBaseValueUnion;

typedef struct WrapBaseValue {
  enum WrapBaseValueType ty;
  union WrapBaseValueUnion value;
} WrapBaseValue;

typedef struct Vector3InterpolationResult {
  struct WrapVec3 value;
  bool is_last;
} Vector3InterpolationResult;

typedef struct Vector4InterpolationResult {
  struct WrapVec4 value;
  bool is_last;
} Vector4InterpolationResult;

typedef struct QuaternionInterpolationResult {
  struct WrapQuat value;
  bool is_last;
} QuaternionInterpolationResult;

typedef struct Option_BaseValue ValueProperty;

typedef struct PointDefinitionInterpolation PathProperty;

typedef struct CValueProperty {
  bool has_value;
  struct WrapBaseValue value;
} CValueProperty;



#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct FFIJsonValue tracks_create_json_number(double value);

struct FFIJsonValue tracks_create_json_string(const char *value);

struct FFIJsonValue tracks_create_json_array(const struct FFIJsonValue *elements, uintptr_t length);

void tracks_free_json_value(struct FFIJsonValue *json_value);

struct BaseFFIProviderValues *tracks_make_base_ffi_provider(const BaseFFIProvider *func,
                                                            void *user_value);

/**
 * Dispose the base provider. Consumes
 */
void tracks_dipose_base_ffi_provider(struct BaseFFIProviderValues *func);

/**
 * CONTEXT
 */
struct BaseProviderContext *tracks_make_base_provider_context(void);

void tracks_set_base_provider(struct BaseProviderContext *context,
                              const char *base,
                              float *values,
                              uintptr_t count,
                              bool quat);

/**
 *FLOAT POINT DEFINITION
 */
const struct FloatPointDefinition *tracks_make_float_point_definition(const struct FFIJsonValue *json,
                                                                      struct BaseProviderContext *context);

struct FloatInterpolationResult tracks_interpolate_float(const struct FloatPointDefinition *point_definition,
                                                         float time,
                                                         struct BaseProviderContext *context);

uintptr_t tracks_float_count(const struct FloatPointDefinition *point_definition);

bool tracks_float_has_base_provider(const struct FloatPointDefinition *point_definition);

/**
 *BASE POINT DEFINITION
 */
struct BasePointDefinition *tracks_make_base_point_definition(const struct FFIJsonValue *json,
                                                              enum WrapBaseValueType ty,
                                                              struct BaseProviderContext *context);

struct WrapBaseValue tracks_interpolate_base_point_definition(const struct BasePointDefinition *point_definition,
                                                              float time,
                                                              bool *is_last_out,
                                                              struct BaseProviderContext *context);

uintptr_t tracks_base_point_definition_count(const struct BasePointDefinition *point_definition);

bool tracks_base_point_definition_has_base_provider(const struct BasePointDefinition *point_definition);

/**
 *VECTOR3 POINT DEFINITION
 */
const struct Vector3PointDefinition *tracks_make_vector3_point_definition(const struct FFIJsonValue *json,
                                                                          struct BaseProviderContext *context);

struct Vector3InterpolationResult tracks_interpolate_vector3(const struct Vector3PointDefinition *point_definition,
                                                             float time,
                                                             struct BaseProviderContext *context);

uintptr_t tracks_vector3_count(const struct Vector3PointDefinition *point_definition);

bool tracks_vector3_has_base_provider(const struct Vector3PointDefinition *point_definition);

/**
 *VECTOR4 POINT DEFINITION
 */
const struct Vector4PointDefinition *tracks_make_vector4_point_definition(const struct FFIJsonValue *json,
                                                                          struct BaseProviderContext *context);

struct Vector4InterpolationResult tracks_interpolate_vector4(const struct Vector4PointDefinition *point_definition,
                                                             float time,
                                                             struct BaseProviderContext *context);

uintptr_t tracks_vector4_count(const struct Vector4PointDefinition *point_definition);

bool tracks_vector4_has_base_provider(const struct Vector4PointDefinition *point_definition);

/**
 *QUATERNION POINT DEFINITION
 */
const struct QuaternionPointDefinition *tracks_make_quat_point_definition(const struct FFIJsonValue *json,
                                                                          struct BaseProviderContext *context);

struct QuaternionInterpolationResult tracks_interpolate_quat(const struct QuaternionPointDefinition *point_definition,
                                                             float time,
                                                             struct BaseProviderContext *context);

uintptr_t tracks_quat_count(const struct QuaternionPointDefinition *point_definition);

bool tracks_quat_has_base_provider(const struct QuaternionPointDefinition *point_definition);

/**
 * Creates a new CoroutineManager instance and returns a raw pointer to it.
 * The caller is responsible for freeing the memory using destroy_coroutine_manager.
 */
struct CoroutineManager *create_coroutine_manager(void);

/**
 * Destroys a CoroutineManager instance, freeing its memory.
 */
void destroy_coroutine_manager(struct CoroutineManager *manager);

/**
 * Starts an event coroutine in the manager. Consumes event_data
 */
void start_event_coroutine(struct CoroutineManager *manager,
                           float bpm,
                           float song_time,
                           const struct BaseProviderContext *context,
                           struct EventData *event_data);

/**
 * Polls all events in the manager, updating their state based on the current song time.
 */
void poll_events(struct CoroutineManager *manager,
                 float song_time,
                 const struct BaseProviderContext *context);

struct Track *track_create(void);

/**
 * Consumes the track and frees its memory.
 */
void track_destroy(struct Track *track);

void track_set_name(struct Track *track, const char *name);

const char *track_get_name(const struct Track *track);

void track_register_game_object(struct Track *track, struct GameObject *game_object);

void track_register_property(struct Track *track, const char *id, ValueProperty *property);

const ValueProperty *track_get_property(const struct Track *track, const char *id);

PathProperty *track_get_path_property(struct Track *track, const char *id);

void track_mark_updated(struct Track *track);

PathProperty *path_property_create(void);

void path_property_finish(PathProperty *ptr);

/**
 * Consumes the path property and frees its memory.
 */
void path_property_free(PathProperty *ptr);

float path_property_get_time(const PathProperty *ptr);

void path_property_set_time(PathProperty *ptr, float time);

struct CValueProperty path_property_interpolate(PathProperty *ptr,
                                                float time,
                                                struct BaseProviderContext *context);

enum WrapBaseValueType property_get_type(const ValueProperty *ptr);

enum WrapBaseValueType path_property_get_type(const PathProperty *ptr);

struct TracksContext *tracks_context_create(void);

/**
 * Consumes the context and frees its memory.
 */
void tracks_context_destroy(struct TracksContext *context);

/**
 * Consumes the track and moves
 * it into the context. Returns a const pointer to the track.
 */
const struct Track *tracks_context_add_track(struct TracksContext *context, struct Track *track);

/**
 * Consumes the point definition and moves it into the context.
 * Returns a const pointer to the point definition.
 */
const struct BasePointDefinition *tracks_context_add_point_definition(struct TracksContext *context,
                                                                      struct BasePointDefinition *point_def);

struct Track *tracks_context_get_track_by_name(struct TracksContext *context, const char *name);

struct Track *tracks_context_get_track(struct TracksContext *context, uintptr_t index);

struct CoroutineManager *tracks_context_get_coroutine_manager(struct TracksContext *context);

struct BaseProviderContext *tracks_context_get_base_provider_context(struct TracksContext *context);

/**
 * C-compatible wrapper for easing functions
 */
float interpolate_easing(enum Functions easing_function, float t);

/**
 * Gets an easing function by index (useful for FFI where enums might be troublesome)
 * Returns Functions::EaseLinear if the index is out of bounds
 */
enum Functions get_easing_function_by_index(int32_t index);

/**
 * Gets the total number of available easing functions
 */
int32_t get_easing_function_count(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#ifdef __cplusplus
}  // namespace ffi
}  // namespace Tracks
#endif  // __cplusplus
