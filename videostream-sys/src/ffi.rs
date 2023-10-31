/* automatically generated by rust-bindgen 0.64.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vsl_host {
    _unused: [u8; 0],
}
#[doc = " The VSLHost object manages a connection point at the user-defined path and\n allows frames to be registered for client use."]
pub type VSLHost = vsl_host;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vsl_client {
    _unused: [u8; 0],
}
#[doc = " The VSLClient object manages a single connection to a VSLHost."]
pub type VSLClient = vsl_client;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vsl_frame {
    _unused: [u8; 0],
}
#[doc = " The VSLFrame object represents a single video frame from either the host\n or client perspective.  Certain API are only available to the host or client."]
pub type VSLFrame = vsl_frame;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vsl_encoder {
    _unused: [u8; 0],
}
#[doc = " The VSLEncoder object represents encoder instance.\n"]
pub type VSLEncoder = vsl_encoder;
#[doc = " The VSLRect structure represents a rectangle region of a frame and is used to\n define cropping regions for sub-frames."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vsl_rect {
    #[doc = " The left-most pixel offset for the rectangle."]
    pub x: ::std::os::raw::c_int,
    #[doc = " The top-most pixel offset for the rectangle."]
    pub y: ::std::os::raw::c_int,
    #[doc = " The width in pixels of the rectangle.  The end position is x+width."]
    pub width: ::std::os::raw::c_int,
    #[doc = " The height in pixels of the rectangle.  The end position is y+height."]
    pub height: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_vsl_rect() {
    const UNINIT: ::std::mem::MaybeUninit<vsl_rect> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<vsl_rect>(),
        16usize,
        concat!("Size of: ", stringify!(vsl_rect))
    );
    assert_eq!(
        ::std::mem::align_of::<vsl_rect>(),
        4usize,
        concat!("Alignment of ", stringify!(vsl_rect))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vsl_rect),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(vsl_rect),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).width) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vsl_rect),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).height) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(vsl_rect),
            "::",
            stringify!(height)
        )
    );
}
#[doc = " The VSLRect structure represents a rectangle region of a frame and is used to\n define cropping regions for sub-frames."]
pub type VSLRect = vsl_rect;
pub const vsl_encode_profile_VSL_ENCODE_PROFILE_AUTO: vsl_encode_profile = 0;
pub type vsl_encode_profile = ::std::os::raw::c_uint;
pub use self::vsl_encode_profile as VSLEncoderProfile;
#[doc = " Function pointer definition which will be called as part of\n @ref vsl_frame_unregister.  This is typically used to free resources\n associated with the frame on either client or host side."]
pub type vsl_frame_cleanup = ::std::option::Option<unsafe extern "C" fn(frame: *mut VSLFrame)>;
extern "C" {
    #[doc = " Returns the VideoStream Library version."]
    pub fn vsl_version() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn vsl_timestamp() -> i64;
}
extern "C" {
    #[doc = " Creates a host on the requested path.  If the path is unavailable because\n of permissions or already exists then NULL is returned and errno is set.\n\n @memberof VSLHost"]
    pub fn vsl_host_init(path: *const ::std::os::raw::c_char) -> *mut VSLHost;
}
extern "C" {
    #[doc = " Releases the host, disconnecting all clients and releasing any allocated\n memory.\n\n @memberof VSLHost"]
    pub fn vsl_host_release(host: *mut VSLHost);
}
extern "C" {
    #[doc = " Returns the bound path of the host.\n\n @memberof VSLHost"]
    pub fn vsl_host_path(host: *const VSLHost) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Polls the list of available connections in our pool.  If @param wait is >0\n then poll will timeout after @param wait milliseconds.  Note frames are only\n expired by the @ref vsl_host_process function so the @param wait parameter\n should be some value no greater than the desired expiration time.\n\n @memberof VSLHost"]
    pub fn vsl_host_poll(host: *mut VSLHost, wait: i64) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Services a single client socket.  Note this does not accept new sockets for\n that you must call @ref vsl_host_process().  The main advantage over calling\n this function is to see if individual client servicing resulted in an error.\n\n @since 1.0\n @memberof VSLHost"]
    pub fn vsl_host_service(
        host: *mut VSLHost,
        sock: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Process the host tasks by first expiring old frames and then servicing the\n first available connection in our pool.  This function should be called in a\n loop, generally blocked by @ref vsl_host_poll.\n\n @memberof VSLHost"]
    pub fn vsl_host_process(host: *mut VSLHost) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Request a copy of the sockets managed by the host.  There will always be at\n least one socket which is the connection socket which accepts new\n connections.  Up to n_sockets socket descriptors will be copied into the\n sockets buffer, if n_sockets is fewer than the number of available sockets\n errno will be set to ENOBUFS. The n_socket parameter, if provided, will be\n populated with a value of n_clients+1 which can be used to query required\n space for the sockets buffer.  It is suggested to provide a buffer which is\n larger than max_sockets to avoid race conditions where the number of sockets\n changes between calls to this function.\n\n Note that the array of sockets should be refreshed often as once the function\n returns they may be stale.  The API is implemented in such as way as to allow\n thread-safe operations where one thread may-be using the vsl sockets to send\n messages while another is polling for a read.\n\n @memberof VSLHost"]
    pub fn vsl_host_sockets(
        host: *mut VSLHost,
        n_sockets: usize,
        sockets: *mut ::std::os::raw::c_int,
        max_sockets: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Registers the frame with the host and publishes it to subscribers.\n\n @note A frame posted to this function transfers ownership to the host and\n should not have @ref vsl_frame_release called on it.  This will be managed\n by the host on frame expiry."]
    pub fn vsl_host_post(
        host: *mut VSLHost,
        frame: *mut VSLFrame,
        expires: i64,
        duration: i64,
        pts: i64,
        dts: i64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Drops the frame from the host.  This is meant to be called from the frame\n but can also be used to remove the host association of the frame and return\n ownership to the caller.\n\n @since 1.3\n @memberof VSLHost"]
    pub fn vsl_host_drop(host: *mut VSLHost, frame: *mut VSLFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Creates a client and connects to the host at the provided path.  If the\n connection cannot be made NULL is returned and errno is set.\n\n @memberof VSLClient"]
    pub fn vsl_client_init(
        path: *const ::std::os::raw::c_char,
        userptr: *mut ::std::os::raw::c_void,
        reconnect: bool,
    ) -> *mut VSLClient;
}
extern "C" {
    #[doc = " Releases the client, disconnecting from the host and releasing allocated\n memory.\n\n @memberof VSLClient"]
    pub fn vsl_client_release(client: *mut VSLClient);
}
extern "C" {
    #[doc = " Disconnects from the VSLHost and stops all reconnection attempts.  This\n should be called as part of closing down a VSL client session.  It is\n thread-safe unlike vsl_client_release which disposes of the client object.\n\n @memberof VSLClient\n @since 1.1"]
    pub fn vsl_client_disconnect(client: *mut VSLClient);
}
extern "C" {
    #[doc = " Returns the optional userptr associated with this client connection.\n\n @memberof VSLClient"]
    pub fn vsl_client_userptr(client: *mut VSLClient) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Returns the path on which the client has connected to the host.\n\n @memberof VSLClient"]
    pub fn vsl_client_path(client: *const VSLClient) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Sets the socket timeout for this client.\n\n @memberof VSLClient"]
    pub fn vsl_client_set_timeout(client: *mut VSLClient, timeout: f32);
}
extern "C" {
    #[doc = " Creates and posts the video frame along with optional user pointer to any\n arbitrary data.  Typically it would be used for holding a reference to\n the host's view of the frame handle.\n\n @deprecated The vsl_frame_register function is deprecated in favour of using\n the @ref vsl_frame_init(), @ref vsl_frame_alloc() or @ref vsl_frame_attach(),\n and @ref vsl_host_post() functions which separate frame creation from posting\n to the host for publishing to subscribers.\n\n @note A frame created through this function is owned by the host and should\n not have @ref vsl_frame_release called on it.  This will be managed by the\n host on frame expiry.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_register(
        host: *mut VSLHost,
        serial: i64,
        handle: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        fourcc: u32,
        size: usize,
        offset: usize,
        expires: i64,
        duration: i64,
        pts: i64,
        dts: i64,
        cleanup: vsl_frame_cleanup,
        userptr: *mut ::std::os::raw::c_void,
    ) -> *mut VSLFrame;
}
extern "C" {
    #[doc = " Initializes a VSLFrame without underlying frame buffer.  To create the\n backing memory either call @ref vsl_frame_alloc() or to attach to an existing\n bufer use @ref vsl_frame_attach().\n\n @since 1.3\n @memberof VSLFrame"]
    pub fn vsl_frame_init(
        width: u32,
        height: u32,
        stride: u32,
        fourcc: u32,
        userptr: *mut ::std::os::raw::c_void,
        cleanup: vsl_frame_cleanup,
    ) -> *mut VSLFrame;
}
extern "C" {
    #[doc = " Allocates the underlying memory for the frame.  This function will prefer to\n allocate using dmabuf and fallback to shared memory if dmabuf is not\n available, unless the frame has a path defined in which case shared memory is\n assumed.  If the path begins with /dev then it assumed to point to a\n dmabuf-heap device.  If path is NULL then the allocator will first attempt to\n create a dmabuf then fallback to shared memory.\n\n Allocations will be based on a buffer large enough to hold height*stride\n bytes.  If using a compressed fourcc such as JPEG the actual data will be\n smaller, this size can be captured when calling @ref vsl_frame_copy() as the\n function returns the number of bytes copied into the target frame.  There is\n currently no method to capture the actual compressed size when receiving an\n already compressed frame.  This limitation is because the size varies from\n frame to frame while the underlying buffer is of a fixed size.  When the\n actual encoded size is important, the @ref vsl_frame_copy() should be called\n directly or the reported size communicated to the client through a separate\n channel.\n\n @since 1.3\n @memberof VSLFrame"]
    pub fn vsl_frame_alloc(
        frame: *mut VSLFrame,
        path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Frees the allocated buffer for this frame.  Does not release the frame itself\n for that use @ref vsl_frame_release().\n\n @param frame\n @since 1.3\n @memberof VSLFrame"]
    pub fn vsl_frame_unalloc(frame: *mut VSLFrame);
}
extern "C" {
    #[doc = " Attach the provided file descriptor to the VSLFrame.  If size is not provided\n it is assumed to be stride*height bytes.  If offset is provided then size\n *MUST* be provided, the offset is in bytes to the start of the frame.\n\n @since 1.3\n @memberof VSLFrame"]
    pub fn vsl_frame_attach(
        frame: *mut VSLFrame,
        fd: ::std::os::raw::c_int,
        size: usize,
        offset: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the path to the underlying VSLFrame buffer.  Note it will not always\n be available, such as when the frame was externally created.  When no path is\n available NULL is returned.\n\n @note This function is not thread-safe and you must use the string\n immediately.\n\n @since 1.3\n @memberof VSLFrame"]
    pub fn vsl_frame_path(frame: *const VSLFrame) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Unregisters the frame, removing it from the host pool.\n\n @deprecated This function is deprecated in favour of calling\n @ref vsl_frame_release() which will handle the required cleanup.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_unregister(frame: *mut VSLFrame);
}
extern "C" {
    #[doc = " Copy the source frame into the target frame, with optional source crop. The\n copy handles format conversion, rescaling to fit the target frame.  Resize\n happens after the crop, if required.\n\n Copy can happen between any frames, regardless of whether they are parented\n or not or have differing parents.  The copy happens through the underlying\n buffers and will attempt to use available hardware accelerators.\n\n The function will attempt to lock target and source.  Since lock is a no-op\n when not a client frame it is safe even for free-standing frames.  Copying to\n or from a posted frame is safe but is likely to cause visual corruption such\n as tearing.\n\n @since 1.3\n @memberof VSLFrame"]
    pub fn vsl_frame_copy(
        target: *mut VSLFrame,
        source: *mut VSLFrame,
        crop: *const VSLRect,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the user pointer associated with this frame.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_userptr(frame: *mut VSLFrame) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Associate userptr with this frame.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_set_userptr(frame: *mut VSLFrame, userptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    #[doc = " Waits for a frame to arrive and returns a new frame object.  Frames who's\n timestamp is less than @param until will be ignored.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_wait(client: *mut VSLClient, until: i64) -> *mut VSLFrame;
}
extern "C" {
    #[doc = " Releases the frame, performing required cleanup.  If the frame was mapped it\n will be unmapped.  If the frame was posted to a host it will be removed, if\n this is a client frame it will be unlocked.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_release(frame: *mut VSLFrame);
}
extern "C" {
    #[doc = " Attempts to lock the video frame.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_trylock(frame: *mut VSLFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Attempts to unlock the video frame.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_unlock(frame: *mut VSLFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the serial frame count of the video frame.\n\n Note this frame serial tracks the count of frames registered on the host and\n does not necessarily equal the actual frame number from the camera.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_serial(frame: *const VSLFrame) -> i64;
}
extern "C" {
    #[doc = " Returns the timestamp for this frame in nanoseconds.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_timestamp(frame: *const VSLFrame) -> i64;
}
extern "C" {
    #[doc = " Returns the duration for this frame in nanoseconds.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_duration(frame: *const VSLFrame) -> i64;
}
extern "C" {
    #[doc = " Returns the presentation timestamp for this frame in nanoseconds.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_pts(frame: *const VSLFrame) -> i64;
}
extern "C" {
    #[doc = " Returns the decode timestamp for this frame in nanoseconds.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_dts(frame: *const VSLFrame) -> i64;
}
extern "C" {
    #[doc = " Returns the epiration time for this frame in milliseconds.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_expires(frame: *const VSLFrame) -> i64;
}
extern "C" {
    #[doc = " Returns the FOURCC code for the video frame.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_fourcc(frame: *const VSLFrame) -> u32;
}
extern "C" {
    #[doc = " Returns the width in pixels of the video frame.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_width(frame: *const VSLFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the height in pixels of the video frame.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_height(frame: *const VSLFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the stride in bytes of the video frame, to go from one row to the\n next.\n\n @since 1.3\n @memberof VSLFrame"]
    pub fn vsl_frame_stride(frame: *const VSLFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the size in bytes of the video frame.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_size(frame: *const VSLFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the file descriptor for this frame or -1 if none is associated.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_handle(frame: *const VSLFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the physical address of the frame.  If the frame does not support\n DMA then MMAP_FAILED is returned.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_paddr(frame: *const VSLFrame) -> isize;
}
extern "C" {
    #[doc = " Maps the frame into the process' memory space, optionally also sets the\n size of the frame if @param size is non-NULL.  Ensure the frame is\n unmapped when no longer needed using @ref nn_frame_munmap().\n\n Note that a frame must be locked for the duration of the mapping.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_mmap(frame: *mut VSLFrame, size: *mut usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Maps the frame into the process' memory space, optionally also sets the\n size of the frame if @param size is non-NULL.\n\n @memberof VSLFrame"]
    pub fn vsl_frame_munmap(frame: *mut VSLFrame);
}
extern "C" {
    #[doc = " Cache synchronization session control for when using DMA-backed buffers.\n This happens automatically on mmap/munmap but the API is also available for\n cases where the frame is updated in-place during a mapping.\n\n @param frame the frame object to synchronize\n @param enable whether the sync session is being enabled or disabled\n @param mode the synchronization mode controls READ, WRITE, or both.\n @since 1.3\n @memberof VSLFrame"]
    pub fn vsl_frame_sync(
        frame: *const VSLFrame,
        enable: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns a fourcc integer code from the string.  If the fourcc code is invalid\n or unsupported then 0 is returned."]
    pub fn vsl_fourcc_from_string(fourcc: *const ::std::os::raw::c_char) -> u32;
}
extern "C" {
    #[doc = " @brief Creates VSLEncoder instance\n\n @param profile VSLEncoderProfile determining encode quality\n @param outputFourcc fourcc code defining the codec\n @param fps output stream fps\n @return VSLEncoder* new encoder instance\n\n Every encoder instance must be released using vsl_encoder_release\n\n For Hantro VC8000e encoder initialization is performed when vsl_encode_frame\n is called for a first time"]
    pub fn vsl_encoder_create(
        profile: VSLEncoderProfile,
        outputFourcc: u32,
        fps: ::std::os::raw::c_int,
    ) -> *mut VSLEncoder;
}
extern "C" {
    #[doc = " @brief Destroys VSLEncoder instance\n\n @param encoder VSLEncoder* instance to destroy"]
    pub fn vsl_encoder_release(encoder: *mut VSLEncoder);
}
extern "C" {
    #[doc = " @brief Encode frame\n @param encoder VSLEncoder instance\n @param source VSLFrame source\n @param destination VSLFrame destination\n @param cropRegion (optional) VSLRect that defines the crop region, NULL when\n destination and source sizes match\n @param keyframe (optional) VSL sets this to 1 if the encoded frame is a\n keyframe, otherwise 0. User can set to NULL to ignore param.\n @retval 0 on success\n @retval -1 on falure (check errno for details)\n\n For Hantro VC8000e encoder initialization is performed when this function is\n called for a first time For Hantro VC8000e encoder source width, height and\n fourcc; destination width, height and fourcc; cropRegion parameters must\n match for all function calls throughout the lifetime of the encoder instance"]
    pub fn vsl_encode_frame(
        encoder: *mut VSLEncoder,
        source: *mut VSLFrame,
        destination: *mut VSLFrame,
        cropRegion: *const VSLRect,
        keyframe: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsl_encoder_new_output_frame(
        encoder: *const VSLEncoder,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        duration: i64,
        pts: i64,
        dts: i64,
    ) -> *mut VSLFrame;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vsl_camera_buffer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vsl_camera {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " Opens the camera device specified by the @param filename and allocates\n device memory. If the device was not found or could not be recognized\n\n Return NULL if the device was not found or could not be recognized.\n Otherwise returns a vsl_camera context which can be used in other vsl_camera\n functions.\n\n @memberof VSLCamera"]
    pub fn vsl_camera_open_device(filename: *const ::std::os::raw::c_char) -> *mut vsl_camera;
}
extern "C" {
    #[doc = " Initialized the camera device in @param ctx for streaming\n and allocate camera buffers.\n\n Then requests the camera to stream at the requested @param width\n and @param height using the requested @param fourcc code.\n\n The @param width, @param height, @param fourcc parameters\n will be set to the actual width and height and fourcc that\n the camera driver sets the device to.\n\n Returns -1 if an error is encountered when initializing the camera to stream,\n otherwise returns 0\n\n @memberof VSLCamera"]
    pub fn vsl_camera_init_device(
        ctx: *mut vsl_camera,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
        buf_count: *mut ::std::os::raw::c_int,
        fourcc: *mut u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Requests the camera in @param ctx to mirror the image leftside right\n\n Returns -1 if a mirror was requested but the camera driver refused\n the request, otherwise 0.\n\n @memberof VSLCamera"]
    pub fn vsl_camera_mirror(ctx: *const vsl_camera, mirror: bool) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Requests the camera in @param ctx to mirror the image upside down\n\n Returns -1 if a mirror was requested but the camera driver refused\n the request, otherwise 0.\n\n @memberof VSLCamera"]
    pub fn vsl_camera_mirror_v(ctx: *const vsl_camera, mirror: bool) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Starts the camera stream.\n\n Must be called after @ref vsl_camera_init_device\n\n @memberof VSLCamera"]
    pub fn vsl_camera_start_capturing(ctx: *mut vsl_camera) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Attempts to read a frame from the camera.\n\n Must be called after @ref vsl_camera_start_capturing.\n\n Ensure to call @ref vsl_camera_release_buffer after the buffer is done being\n used and allow the buffer to be reused for frame capture.\n\n @memberof VSLCamera"]
    pub fn vsl_camera_get_data(ctx: *mut vsl_camera) -> *mut vsl_camera_buffer;
}
extern "C" {
    #[doc = " Enqueues a buffer to be reused for frame capture.\n\n @memberof VSLCamera"]
    pub fn vsl_camera_release_buffer(
        ctx: *mut vsl_camera,
        buffer: *const vsl_camera_buffer,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Stops the camera stream.\n\n @memberof VSLCamera"]
    pub fn vsl_camera_stop_capturing(ctx: *const vsl_camera) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Uninitializes the camera buffers and frees the buffer memory\n\n Ensure that the device is not streaming. If\n @ref vsl_camera_start_capturing was called, ensure that\n @ref vsl_camera_stop_capturing is called before this function\n\n @memberof VSLCamera"]
    pub fn vsl_camera_uninit_device(ctx: *mut vsl_camera);
}
extern "C" {
    #[doc = " Closes the camera device and frees the device memory\n\n Ensure that the device is not streaming. If\n @ref vsl_camera_start_capturing was called, ensure that\n @ref vsl_camera_stop_capturing is called before this function\n\n @memberof VSLCamera"]
    pub fn vsl_camera_close_device(ctx: *mut vsl_camera);
}
extern "C" {
    #[doc = " Checks if dma buffers are supported on the camera\n\n Ensure that this is called after\n @ref vsl_camera_init_device\n\n @memberof VSLCamera"]
    pub fn vsl_camera_is_dmabuf_supported(ctx: *const vsl_camera) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the number of queued buffers for the camera.\n @ref vsl_camera_get_data will timeout if there are 0 queued buffers.\n\n The user can send buffers back to the buffer queue using\n @ref vsl_camera_release_buffer\n\n @memberof VSLCamera"]
    pub fn vsl_camera_get_queued_buf_count(ctx: *const vsl_camera) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the mmap memory of the camera buffer\n\n @memberof VSLCamera"]
    pub fn vsl_camera_buffer_mmap(buffer: *mut vsl_camera_buffer) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Returns the dmabuf file descriptor of the camera buffer\n\n If the device does not support dmabuf, returns -1\n\n @memberof VSLCamera"]
    pub fn vsl_camera_buffer_dma_fd(buffer: *const vsl_camera_buffer) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the phys addr of the camera buffer\n\n If the device does not support dmabuf, returns 0\n\n @memberof VSLCamera"]
    pub fn vsl_camera_buffer_phys_addr(buffer: *const vsl_camera_buffer) -> u64;
}
extern "C" {
    #[doc = " Returns the length of the camera buffer in bytes\n\n @memberof VSLCamera"]
    pub fn vsl_camera_buffer_length(buffer: *const vsl_camera_buffer) -> u32;
}
extern "C" {
    #[doc = " Returns the fourcc code of the camera buffer\n\n @memberof VSLCamera"]
    pub fn vsl_camera_buffer_fourcc(buffer: *const vsl_camera_buffer) -> u32;
}
extern "C" {
    #[doc = " Lists the supported single planar formats of\n the camera into @param codes as fourcc codes"]
    pub fn vsl_camera_enum_fmts(
        ctx: *const vsl_camera,
        codes: *mut u32,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Lists the supported multi planar formats of\n the camera into @param codes as fourcc codes"]
    pub fn vsl_camera_enum_mplane_fmts(
        ctx: *const vsl_camera,
        codes: *mut u32,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
