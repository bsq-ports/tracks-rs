#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
namespace Tracks {
namespace ffi {
#endif  // __cplusplus

/**
 * JSON FFI
 */
typedef enum JsonValueType {
  Number,
  Null,
  String,
  Array,
} JsonValueType;

typedef struct BaseFFIProviderValues BaseFFIProviderValues;

typedef struct BasePointDefinition BasePointDefinition;

typedef struct BaseProviderContext BaseProviderContext;

typedef struct CoroutineManager CoroutineManager;

typedef struct EventData EventData;

typedef struct FloatPointDefinition FloatPointDefinition;

typedef struct GameObject GameObject;

typedef struct Option_BaseValue Option_BaseValue;

typedef struct PathProperty PathProperty;

typedef struct QuaternionPointDefinition QuaternionPointDefinition;

typedef struct RefCell_BasePointDefinition RefCell_BasePointDefinition;

typedef struct RefCell_Track RefCell_Track;

typedef struct Track Track;

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

typedef struct Vector3InterpolationResult {
  struct WrapVec3 value;
  bool is_last;
} Vector3InterpolationResult;

typedef struct WrapVec4 {
  float x;
  float y;
  float z;
  float w;
} WrapVec4;

typedef struct Vector4InterpolationResult {
  struct WrapVec4 value;
  bool is_last;
} Vector4InterpolationResult;

typedef struct WrapQuat {
  float x;
  float y;
  float z;
  float w;
} WrapQuat;

typedef struct QuaternionInterpolationResult {
  struct WrapQuat value;
  bool is_last;
} QuaternionInterpolationResult;

/**
 * Type that handles converting a Rc type to/from C
 */
typedef struct RcC_RefCell_BasePointDefinition {
  const struct RefCell_BasePointDefinition *rc;
} RcC_RefCell_BasePointDefinition;

typedef struct RcC_RefCell_BasePointDefinition RcCRefCell_BasePointDefinition;

/**
 * Type that handles converting a Rc type to/from C
 */
typedef struct RcC_RefCell_Track {
  const struct RefCell_Track *rc;
} RcC_RefCell_Track;

typedef struct RcC_RefCell_Track RcCRefCell_Track;

typedef struct Option_BaseValue ValueProperty;



#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct FFIJsonValue tracks_create_json_number(double value);

struct FFIJsonValue tracks_create_json_string(const char *value);

struct FFIJsonValue tracks_create_json_array(const struct FFIJsonValue *elements, uintptr_t length);

void tracks_free_json_value(struct FFIJsonValue *json_value);

struct BaseFFIProviderValues *tracks_make_base_ffi_provider(const BaseFFIProvider *func,
                                                            void *user_value);

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

RcCRefCell_BasePointDefinition base_point_definition_into_global(struct BasePointDefinition *ptr);

void base_point_definition_global_dispose(struct RcC_RefCell_BasePointDefinition ptr);

/**
 * Create a new coroutine manager.
 */
struct CoroutineManager *coroutine_manager_new(void);

/**
 * Start an event coroutine.
 */
void coroutine_manager_start_event(struct CoroutineManager *manager,
                                   float bpm,
                                   float song_time,
                                   const struct BaseProviderContext *context,
                                   struct EventData *event_data);

/**
 * Poll events in a coroutine manager.
 */
void coroutine_manager_poll_events(struct CoroutineManager *manager,
                                   float song_time,
                                   const struct BaseProviderContext *context);

/**
 * Free a coroutine manager.
 */
void coroutine_manager_free(struct CoroutineManager *manager);

/**
 * Create a new empty track
 */
const struct Track *track_create(void);

/**
 * Free a track
 */
void track_global_dispose(RcCRefCell_Track track);

/**
 * Register a value property
 */
void track_register_property(struct Track *track, const char *id, ValueProperty *property);

/**
 * Register a path property
 */
void track_register_path_property(struct Track *track,
                                  const char *id,
                                  struct PathProperty *property);

/**
 * Register a game object
 */
void track_register_game_object(struct Track *track, struct GameObject *game_object);

/**
 * Remove a game object
 */
void track_remove_game_object(struct Track *track, const struct GameObject *game_object);

/**
 * Mark the track as updated
 */
void track_mark_updated(struct Track *track);

/**
 * Check if a property exists
 */
int track_has_property(const struct Track *track, const char *id);

/**
 * Check if a path property exists
 */
int track_has_path_property(const struct Track *track, const char *id);

/**
 * Get a property
 */
const ValueProperty *track_get_property(const struct Track *track, const char *id);

/**
 * Get a path property
 */
const struct PathProperty *track_get_path_property(const struct Track *track, const char *id);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#ifdef __cplusplus
}  // namespace ffi
}  // namespace Tracks
#endif  // __cplusplus
