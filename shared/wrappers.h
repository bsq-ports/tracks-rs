#pragma once

#include "bindings.h"
#include <optional>
#include <stdexcept>

namespace Tracks {

struct CoroutineManager {
  ffi::CoroutineManager *ptr;

  CoroutineManager(ffi::CoroutineManager *ptr) : ptr(ptr) {}

  // copies allowed, as this is owned by TracksContext
  CoroutineManager(const CoroutineManager &) = default;
  CoroutineManager(CoroutineManager &&o) noexcept = default;

  operator ffi::CoroutineManager *() { return ptr; }
};

struct BaseProviderContext {
  ffi::BaseProviderContext *ptr;

  BaseProviderContext(ffi::BaseProviderContext *ptr) : ptr(ptr) {}
  // copies allowed, as this is owned by TracksContext
  BaseProviderContext(const BaseProviderContext &) = default;
  BaseProviderContext(BaseProviderContext &&o) noexcept = default;
  operator ffi::BaseProviderContext *() { return ptr; }
  operator ffi::BaseProviderContext const *() const { return ptr; }
};

struct Track {
  // can be owned or not
  // we therefore disable copies
  ffi::Track *ptr;
  bool owned;

  Track() {
    ptr = ffi::track_create();
    owned = true;
  }
  Track(ffi::Track *ptr, bool owned) : ptr(ptr), owned(owned) {}

  Track(const Track &) = delete;
  Track(Track &&o) noexcept : ptr(o.ptr), owned(o.owned) {
    o.ptr = nullptr;
    o.owned = false;
  }

  ~Track() {
    if (!owned) {
      return;
    }
    if (!ptr) {
      return;
    }
    ffi::track_destroy(ptr);
  }
};

struct PointDefinition {
  // can be owned or not
  // we therefore disable copies
  ffi::BasePointDefinition const *ptr;
  bool owned = false;

  PointDefinition(ffi::FFIJsonValue *json, ffi::WrapBaseValueType ty,
                  ffi::BaseProviderContext *context) {
    ptr = ffi::tracks_make_base_point_definition(json, ty, context);
  }

  PointDefinition(ffi::BasePointDefinition const *ptr, bool owned)
      : ptr(ptr), owned(owned) {}

  PointDefinition(const PointDefinition &) = delete;
  PointDefinition(PointDefinition &&o) noexcept : ptr(o.ptr), owned(o.owned) {
    o.ptr = nullptr;
    o.owned = false;
  }

  ~PointDefinition() {
    if (!owned) {
      return;
    }
    if (!ptr) {
      return;
    }
    ffi::base_point_definition_free(
        const_cast<ffi::BasePointDefinition *>(ptr));
  }

  uintptr_t count() const {
    return Tracks::ffi::tracks_base_point_definition_count(ptr);
  }

  bool hasBaseProvider() const {
    return Tracks::ffi::tracks_base_point_definition_has_base_provider(ptr);
  }
};

struct TracksContext {
  ffi::TracksContext *ptr;

  TracksContext() { ptr = ffi::tracks_context_create(); }
  TracksContext(ffi::TracksContext *ptr) : ptr(ptr) {}

  TracksContext(const TracksContext &) = delete;
  TracksContext(TracksContext &&o) noexcept : ptr(o.ptr) { o.ptr = nullptr; }

  ~TracksContext() {
    if (!ptr) {
      return;
    }
    ffi::tracks_context_destroy(ptr);
  }

  CoroutineManager GetCoroutineManager() const {
    if (!ptr) {
      throw std::runtime_error("TracksContext is null");
    }
    return ffi::tracks_context_get_coroutine_manager(ptr);
  }

  BaseProviderContext GetBaseProviderContext() const {
    if (!ptr) {
      throw std::runtime_error("TracksContext is null");
    }
    return ffi::tracks_context_get_base_provider_context(ptr);
  }

  PointDefinition
  AddPointDefinition(std::optional<std::string_view> id,
                     ffi::BasePointDefinition *pointDefinition) const {
    if (!ptr) {
      throw std::runtime_error("TracksContext is null");
    }
    auto p = ffi::tracks_context_add_point_definition(
        ptr, id.value_or("").data(), pointDefinition);

    return PointDefinition(p, false);
  }

  std::optional<PointDefinition>
  GetPointDefinition(std::string_view name, ffi::WrapBaseValueType ty) const {
    if (!ptr) {
      throw std::runtime_error("TracksContext is null");
    }
    auto pointDefinition =
        ffi::tracks_context_get_point_definition(ptr, name.data(), ty);
    if (!pointDefinition) {
      return std::nullopt;
    }

    return PointDefinition(pointDefinition, false);
  }

  ffi::TrackKeyFFI AddTrack(Track &&track) const {
    if (!ptr) {
      throw std::runtime_error("TracksContext is null");
    }
    auto key = ffi::tracks_context_add_track(ptr, track.ptr);
    // track is now owned by context
    track.owned = false;
    return key;
  }

  Track GetTrack(const ffi::TrackKeyFFI &index) const {
    if (!ptr) {
      throw std::runtime_error("TracksContext is null");
    }
    auto trackPtr = ffi::tracks_context_get_track(ptr, index);
    return Track(trackPtr, false);
  }

  operator ffi::TracksContext const *() const { return ptr; }
  operator ffi::TracksContext *() { return ptr; }
};

} // namespace Tracks