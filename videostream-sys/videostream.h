/**
 * Copyright 2020 by Au-Zone Technologies.  All Rights Reserved.
 */
#ifndef VIDEOSTREAM_H
#define VIDEOSTREAM_H

#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define VSL_VERSION_ENCODE(major, minor, revision) \
    (((major) *1000000) + ((minor) *1000) + (revision))
#define VSL_VERSION_DECODE_MAJOR(version) ((version) / 1000000)
#define VSL_VERSION_DECODE_MINOR(version) (((version) % 1000000) / 1000)
#define VSL_VERSION_DECODE_REVISION(version) ((version) % 1000)

#if defined(__GNUC__) && defined(__GNUC_PATCHLEVEL__)
#define VSL_GNUC_VERSION \
    VSL_VERSION_ENCODE(__GNUC__, __GNUC_MINOR__, __GNUC_PATCHLEVEL__)
#elif defined(__GNUC__)
#define VSL_GNUC_VERSION VSL_VERSION_ENCODE(__GNUC__, __GNUC_MINOR__, 0)
#endif

#if defined(VSL_GNUC_VERSION)
#define VSL_GNUC_VERSION_CHECK(major, minor, patch) \
    (VSL_GNUC_VERSION >= VSL_VERSION_ENCODE(major, minor, patch))
#else
#define VSL_GNUC_VERSION_CHECK(major, minor, patch) (0)
#endif

#if defined(__CC_ARM) && defined(__ARMCOMPILER_VERSION)
#define VSL_ARM_VERSION                                           \
    VSL_VERSION_ENCODE(__ARMCOMPILER_VERSION / 1000000,           \
                       (__ARMCOMPILER_VERSION % 1000000) / 10000, \
                       (__ARMCOMPILER_VERSION % 10000) / 100)
#elif defined(__CC_ARM) && defined(__ARMCC_VERSION)
#define VSL_ARM_VERSION                                     \
    VSL_VERSION_ENCODE(__ARMCC_VERSION / 1000000,           \
                       (__ARMCC_VERSION % 1000000) / 10000, \
                       (__ARMCC_VERSION % 10000) / 100)
#endif

#if defined(VSL_ARM_VERSION)
#define VSL_ARM_VERSION_CHECK(major, minor, patch) \
    (VSL_ARM_VERSION >= VSL_VERSION_ENCODE(major, minor, patch))
#else
#define VSL_ARM_VERSION_CHECK(major, minor, patch) (0)
#endif

#if defined(__IAR_SYSTEMS_ICC__)
#if __VER__ > 1000
#define VSL_IAR_VERSION                           \
    VSL_VERSION_ENCODE((__VER__ / 1000000),       \
                       ((__VER__ / 1000) % 1000), \
                       (__VER__ % 1000))
#else
#define VSL_IAR_VERSION VSL_VERSION_ENCODE(VER / 100, __VER__ % 100, 0)
#endif
#endif

#if defined(VSL_IAR_VERSION)
#define VSL_IAR_VERSION_CHECK(major, minor, patch) \
    (VSL_IAR_VERSION >= VSL_VERSION_ENCODE(major, minor, patch))
#else
#define VSL_IAR_VERSION_CHECK(major, minor, patch) (0)
#endif

#if defined(VSL_GNUC_VERSION) && !defined(__clang) && !defined(VSL_ARM_VERSION)
#define VSL_GCC_VERSION VSL_GNUC_VERSION
#endif

#if defined(VSL_GCC_VERSION)
#define VSL_GCC_VERSION_CHECK(major, minor, patch) \
    (VSL_GCC_VERSION >= VSL_VERSION_ENCODE(major, minor, patch))
#else
#define VSL_GCC_VERSION_CHECK(major, minor, patch) (0)
#endif

#if defined(__cplusplus) && (__cplusplus >= 201402L)
#define VSL_DEPRECATED(since) [[deprecated("Since " #since)]]
#define VSL_DEPRECATED_FOR(since, replacement) \
    [[deprecated("Since " #since "; use " #replacement)]]
#elif _MSC_VER >= 1400
#define VSL_DEPRECATED(since) __declspec(deprecated("Since " #since))
#define VSL_DEPRECATED_FOR(since, replacement) \
    __declspec(deprecated("Since " #since "; use " #replacement))
#elif _MSC_VER >= 1310
#define VSL_DEPRECATED(since) _declspec(deprecated)
#define VSL_DEPRECATED_FOR(since, replacement) __declspec(deprecated)
#elif VSL_IAR_VERSION_CHECK(8, 0, 0)
#define VSL_DEPRECATED(since) _Pragma("deprecated")
#define VSL_DEPRECATED_FOR(since, replacement) _Pragma("deprecated")
#elif defined(_GHS_MULTI)
#define VSL_DEPRECATED(since)
#define VSL_DEPRECATED_FOR(since, replacement)
#else
#define VSL_DEPRECATED(since) __attribute__((__deprecated__("Since " #since)))
#define VSL_DEPRECATED_FOR(since, replacement) \
    __attribute__((__deprecated__("Since " #since "; use " #replacement)))
#endif

#if VSL_GCC_VERSION_CHECK(4, 3, 0)
#define VSL_UNAVAILABLE(available_since) \
    __attribute__((__warning__("Not available until " #available_since)))
#else
#define VSL_UNAVAILABLE(available_since)
#endif

#if defined(__cplusplus) && (__cplusplus >= 201703L)
#define VSL_WARN_UNUSED_RESULT [[nodiscard]]
#elif defined(_Check_return_) /* SAL */
#define VSL_WARN_UNUSED_RESULT _Check_return_
#else
#define VSL_WARN_UNUSED_RESULT __attribute__((__warn_unused_result__))
#endif

#if defined(_WIN32) || defined(__CYGWIN__)
#define VSL_PRIVATE
#define VSL_PUBLIC __declspec(dllexport)
#define VSL_IMPORT __declspec(dllimport)
#else
#define VSL_PRIVATE __attribute__((__visibility__("hidden")))
#define VSL_PUBLIC __attribute__((__visibility__("default")))
#define VSL_IMPORT extern
#endif

#if !defined(__cplusplus) &&                                         \
    ((defined(__STDC_VERSION__) && (__STDC_VERSION__ >= 201112L)) || \
     defined(_Static_assert))
#define VSL_STATIC_ASSERT(expr, message) _Static_assert(expr, message)
#elif (defined(__cplusplus) && (__cplusplus >= 201703L)) || (_MSC_VER >= 1600)
#define VSL_STATIC_ASSERT(expr, message) static_assert(expr, message)
#elif defined(__cplusplus) && (__cplusplus >= 201103L)
#define VSL_STATIC_ASSERT(expr, message) static_assert(expr)
#else
#define VSL_STATIC_ASSERT(expr, message)
#endif

#ifdef VSL_API_STATIC
#define VSL_API
#else
#ifdef VSL_API_EXPORT
#define VSL_API VSL_PUBLIC
#else
#define VSL_API VSL_IMPORT
#endif
#endif

#define VSL_VERSION_1_0 VSL_VERSION_ENCODE(1, 0, 0)
#define VSL_VERSION_1_1 VSL_VERSION_ENCODE(1, 1, 0)
#define VSL_VERSION_1_2 VSL_VERSION_ENCODE(1, 2, 0)
#define VSL_VERSION_1_3 VSL_VERSION_ENCODE(1, 3, 0)

#ifndef VSL_TARGET_VERSION
#define VSL_TARGET_VERSION VSL_VERSION_1_3
#endif

#if VSL_TARGET_VERSION < VSL_VERSION_ENCODE(1, 0, 0)
#define VSL_AVAILABLE_SINCE_1_0 VSL_UNAVAILABLE(1.0)
#define VSL_DEPRECATED_SINCE_1_0
#define VSL_DEPRECATED_SINCE_1_0_FOR(replacement)
#else
#define VSL_AVAILABLE_SINCE_1_0
#define VSL_DEPRECATED_SINCE_1_0 VSL_DEPRECATED(1.0)
#define VSL_DEPRECATED_SINCE_1_0_FOR(replacement) \
    VSL_DEPRECATED_FOR(1.0, replacement)
#endif

#if VSL_TARGET_VERSION < VSL_VERSION_ENCODE(1, 1, 0)
#define VSL_AVAILABLE_SINCE_1_1 VSL_UNAVAILABLE(1.1)
#define VSL_DEPRECATED_SINCE_1_1
#define VSL_DEPRECATED_SINCE_1_1_FOR(replacement)
#else
#define VSL_AVAILABLE_SINCE_1_1
#define VSL_DEPRECATED_SINCE_1_1 VSL_DEPRECATED(1.1)
#define VSL_DEPRECATED_SINCE_1_1_FOR(replacement) \
    VSL_DEPRECATED_FOR(1.1, replacement)
#endif

#if VSL_TARGET_VERSION < VSL_VERSION_ENCODE(1, 2, 0)
#define VSL_AVAILABLE_SINCE_1_2 VSL_UNAVAILABLE(1.2)
#define VSL_DEPRECATED_SINCE_1_2
#define VSL_DEPRECATED_SINCE_1_2_FOR(replacement)
#else
#define VSL_AVAILABLE_SINCE_1_2
#define VSL_DEPRECATED_SINCE_1_2 VSL_DEPRECATED(1.2)
#define VSL_DEPRECATED_SINCE_1_2_FOR(replacement) \
    VSL_DEPRECATED_FOR(1.2, replacement)
#endif

#if VSL_TARGET_VERSION < VSL_VERSION_ENCODE(1, 3, 0)
#define VSL_AVAILABLE_SINCE_1_3 VSL_UNAVAILABLE(1.3)
#define VSL_DEPRECATED_SINCE_1_3
#define VSL_DEPRECATED_SINCE_1_3_FOR(replacement)
#else
#define VSL_AVAILABLE_SINCE_1_3
#define VSL_DEPRECATED_SINCE_1_3 VSL_DEPRECATED(1.3)
#define VSL_DEPRECATED_SINCE_1_3_FOR(replacement) \
    VSL_DEPRECATED_FOR(1.3, replacement)
#endif

#define VSL_FOURCC(a, b, c, d)                                         \
    ((uint32_t) (a) | ((uint32_t) (b) << 8) | ((uint32_t) (c) << 16) | \
     ((uint32_t) (d) << 24))

#ifdef __cplusplus
extern "C" {
#endif

/**
 * The VSLHost object manages a connection point at the user-defined path and
 * allows frames to be registered for client use.
 */
typedef struct vsl_host VSLHost;

/**
 * The VSLClient object manages a single connection to a VSLHost.
 */
typedef struct vsl_client VSLClient;

/**
 * The VSLFrame object represents a single video frame from either the host
 * or client perspective.  Certain API are only available to the host or client.
 */
typedef struct vsl_frame VSLFrame;

/**
 * The VSLRect structure represents a rectangle region of a frame and is used to
 * define cropping regions for sub-frames.
 */
typedef struct vsl_rect {
    /**
     * The left-most pixel offset for the rectangle.
     */
    int x;
    /**
     * The top-most pixel offset for the rectangle.
     */
    int y;
    /**
     * The width in pixels of the rectangle.  The end position is x+width.
     */
    int width;
    /**
     * The height in pixels of the rectangle.  The end position is y+height.
     */
    int height;
} VSLRect;

/**
 * Function pointer definition which will be called as part of
 * @ref vsl_frame_unregister.  This is typically used to free resources
 * associated with the frame on either client or host side.
 */
typedef void (*vsl_frame_cleanup)(VSLFrame* frame);

/**
 * Returns the VideoStream Library version.
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
const char*
vsl_version();

/**
 *
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int64_t
vsl_timestamp();

/**
 * Creates a host on the requested path.  If the path is unavailable because
 * of permissions or already exists then NULL is returned and errno is set.
 *
 * @memberof VSLHost
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
VSLHost*
vsl_host_init(const char* path);

/**
 * Releases the host, disconnecting all clients and releasing any allocated
 * memory.
 *
 * @memberof VSLHost
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
void
vsl_host_release(VSLHost* host);

/**
 * Returns the bound path of the host.
 *
 * @memberof VSLHost
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
const char*
vsl_host_path(const VSLHost* host);

/**
 * Polls the list of available connections in our pool.  If @param wait is >0
 * then poll will timeout after @param wait milliseconds.  Note frames are only
 * expired by the @ref vsl_host_process function so the @param wait parameter
 * should be some value no greater than the desired expiration time.
 *
 * @memberof VSLHost
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int
vsl_host_poll(VSLHost* host, int64_t wait);

/**
 * Services the client socket.
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int
vsl_host_service(VSLHost* host, int sock);

/**
 * Process the host tasks by first expiring old frames and then servicing the
 * first available connection in our pool.  This function should be called in a
 * loop, generally blocked by @ref vsl_host_poll.
 *
 * @memberof VSLHost
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int
vsl_host_process(VSLHost* host);

/**
 * Request a copy of the sockets managed by the host.  There will always be at
 * least one socket which is the connection socket which accepts new
 * connections.  Up to n_sockets socket descriptors will be copied into the
 * sockets buffer, if n_sockets is fewer than the number of available sockets
 * errno will be set to ENOBUFS. The n_socket parameter, if provided, will be
 * populated with a value of n_clients+1 which can be used to query required
 * space for the sockets buffer.  It is suggested to provide a buffer which is
 * larger than max_sockets to avoid race conditions where the number of sockets
 * changes between calls to this function.
 *
 * Note that the array of sockets should be refreshed often as once the function
 * returns they may be stale.  The API is implemented in such as way as to allow
 * thread-safe operations where one thread may-be using the vsl sockets to send
 * messages while another is polling for a read.
 *
 * @memberof VSLHost
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int
vsl_host_sockets(VSLHost* host,
                 size_t   n_sockets,
                 int*     sockets,
                 size_t*  max_sockets);

/**
 * Registers the frame with the host and publishes it to subscribers.
 */
VSL_AVAILABLE_SINCE_1_3
VSL_API
int
vsl_host_post(VSLHost*  host,
              VSLFrame* frame,
              int64_t   expires,
              int64_t   duration,
              int64_t   pts,
              int64_t   dts);

/**
 * Creates a client and connects to the host at the provided path.  If the
 * connection cannot be made NULL is returned and errno is set.
 *
 * @memberof VSLClient
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
VSLClient*
vsl_client_init(const char* path, void* userptr, bool reconnect);

/**
 * Releases the client, disconnecting from the host and releasing allocated
 * memory.
 *
 * @memberof VSLClient
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
void
vsl_client_release(VSLClient* client);

/**
 * Disconnects from the VSLHost and stops all reconnection attempts.  This
 * should be called as part of closing down a VSL client session.  It is
 * thread-safe unlike vsl_client_release which disposes of the client object.
 *
 * @memberof VSLClient
 * @since 1.1
 */
VSL_AVAILABLE_SINCE_1_1
VSL_API
void
vsl_client_disconnect(VSLClient* client);

/**
 * Returns the optional userptr associated with this client connection.
 *
 * @memberof VSLClient
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
void*
vsl_client_userptr(VSLClient* client);

/**
 * Returns the path on which the client has connected to the host.
 *
 * @memberof VSLClient
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
const char*
vsl_client_path(const VSLClient* client);

/**
 * Sets the socket timeout for this client.
 *
 * @memberof VSLClient
 */

VSL_AVAILABLE_SINCE_1_0
VSL_API
void
vsl_client_set_timeout(VSLClient* client, float timeout);

/**
 * Creates and posts the video frame along with optional user pointer to any
 * arbitrary data.  Typically it would be used for holding a reference to
 * the host's view of the frame handle.
 *
 * @deprecated The vsl_frame_register function is deprecated in favour of using
 * the @ref vsl_frame_init(), @ref vsl_frame_alloc() or @ref vsl_frame_attach(),
 * and @ref vsl_host_post() functions which separate frame creation from posting
 * to the host for publishing to subscribers.
 *
 * @memberof VSLFrame
 */
VSL_DEPRECATED_SINCE_1_3
VSL_AVAILABLE_SINCE_1_0
VSL_API
VSLFrame*
vsl_frame_register(VSLHost*          host,
                   int64_t           serial,
                   int               handle,
                   int               width,
                   int               height,
                   uint32_t          fourcc,
                   size_t            size,
                   size_t            offset,
                   int64_t           expires,
                   int64_t           duration,
                   int64_t           pts,
                   int64_t           dts,
                   vsl_frame_cleanup cleanup,
                   void*             userptr);

/**
 * Initializes a VSLFrame without underlying frame buffer.  To create the
 * backing memory either call @ref vsl_frame_alloc() or to attach to an existing
 * bufer use @ref vsl_frame_attach().
 *
 * @since 1.3
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_3
VSL_API
VSLFrame*
vsl_frame_init(uint32_t          width,
               uint32_t          height,
               uint32_t          stride,
               uint32_t          fourcc,
               void*             userptr,
               vsl_frame_cleanup cleanup);

/**
 * Allocates the underlying memory for the frame.  This function will prefer to
 * allocate using dmabuf and fallback to shared memory if dmabuf is not
 * available, unless the frame has a path defined in which case shared memory is
 * assumed.
 *
 * @since 1.3
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_3
VSL_API
int
vsl_frame_alloc(VSLFrame* frame);

/**
 * Attach the provided file descriptor to the VSLFrame.  If size is not provided
 * it is assumed to be stride*height bytes.  If offset is provided then size
 * *MUST* be provided, the offset is in bytes to the start of the frame.
 *
 * @since 1.3
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_3
VSL_API
int
vsl_frame_attach(VSLFrame* frame, int fd, size_t size, size_t offset);

/**
 * Unregisters the frame, removing it from the host pool.
 *
 * @deprecated This function is deprecated in favour of calling
 * @ref vsl_frame_release() which will handle the required cleanup.
 *
 * @memberof VSLFrame
 */
VSL_DEPRECATED_SINCE_1_3_FOR(vsl_frame_release)
VSL_AVAILABLE_SINCE_1_0
VSL_API
void
vsl_frame_unregister(VSLFrame* frame);

/**
 * Copy the source frame into the target frame, with optional source crop. The
 * copy handles format conversion, rescaling to fit the target frame.  Resize
 * happens after the crop, if required.
 *
 * Copy can happen between any frames, regardless of whether they are parented
 * or not or have differing parents.  The copy happens through the underlying
 * buffers and will attempt to use available hardware accelerators.
 *
 * The function will attempt to lock target and source.  Since lock is a no-op
 * when not a client frame it is safe even for free-standing frames.  Copying to
 * or from a posted frame is safe but is likely to cause visual corruption such
 * as tearing.
 *
 * @since 1.3
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_3
VSL_API
int
vsl_frame_copy(VSLFrame* target, VSLFrame* source, const VSLRect* crop);

/**
 * Returns the user pointer associated with this frame.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
void*
vsl_frame_userptr(VSLFrame* frame);

/**
 * Associate userptr with this frame.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
void
vsl_frame_set_userptr(VSLFrame* frame, void* userptr);

/**
 * Waits for a frame to arrive and returns a new frame object.  Frames who's
 * timestamp is less than @param until will be ignored.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
VSLFrame*
vsl_frame_wait(VSLClient* client, int64_t until);

/**
 * Releases the frame, performing required cleanup.  If the frame was mapped it
 * will be unmapped.  If the frame was posted to a host it will be removed, if
 * this is a client frame it will be unlocked.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
void
vsl_frame_release(VSLFrame* frame);

/**
 * Attempts to lock the video frame.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int
vsl_frame_trylock(VSLFrame* frame);

/**
 * Attempts to unlock the video frame.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int
vsl_frame_unlock(VSLFrame* frame);

/**
 * Returns the serial frame count of the video frame.
 *
 * Note this frame serial tracks the count of frames registered on the host and
 * does not necessarily equal the actual frame number from the camera.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int64_t
vsl_frame_serial(const VSLFrame* frame);

/**
 * Returns the timestamp for this frame in nanoseconds.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int64_t
vsl_frame_timestamp(const VSLFrame* frame);

/**
 * Returns the duration for this frame in nanoseconds.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int64_t
vsl_frame_duration(const VSLFrame* frame);

/**
 * Returns the presentation timestamp for this frame in nanoseconds.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int64_t
vsl_frame_pts(const VSLFrame* frame);

/**
 * Returns the decode timestamp for this frame in nanoseconds.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int64_t
vsl_frame_dts(const VSLFrame* frame);

/**
 * Returns the epiration time for this frame in milliseconds.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int64_t
vsl_frame_expires(const VSLFrame* frame);

/**
 * Returns the FOURCC code for the video frame.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
uint32_t
vsl_frame_fourcc(const VSLFrame* frame);

/**
 * Returns the width in pixels of the video frame.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int
vsl_frame_width(const VSLFrame* frame);

/**
 * Returns the height in pixels of the video frame.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int
vsl_frame_height(const VSLFrame* frame);

/**
 * Returns the size in bytes of the video frame.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int
vsl_frame_size(const VSLFrame* frame);

/**
 * Returns the file descriptor for this frame or -1 if none is associated.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
int
vsl_frame_handle(const VSLFrame* frame);

/**
 * Returns the physical address of the frame.  If the frame does not support
 * DMA then MMAP_FAILED is returned.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
intptr_t
vsl_frame_paddr(const VSLFrame* frame);

/**
 * Maps the frame into the process' memory space, optionally also sets the
 * size of the frame if @param size is non-NULL.  Ensure the frame is
 * unmapped when no longer needed using @ref nn_frame_munmap().
 *
 * Note that a frame must be locked for the duration of the mapping.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
void*
vsl_frame_mmap(VSLFrame* frame, size_t* size);

/**
 * Maps the frame into the process' memory space, optionally also sets the
 * size of the frame if @param size is non-NULL.
 *
 * @memberof VSLFrame
 */
VSL_AVAILABLE_SINCE_1_0
VSL_API
void
vsl_frame_munmap(VSLFrame* frame);

#ifdef __cplusplus
}
#endif

#endif /* VIDEOSTREAM_H */
