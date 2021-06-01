initSidebarItems({"constant":[["ATOM_NONE",""],["BUTTON_MIDDLE","The middle button. The middle button."],["BUTTON_PRIMARY","The primary button. This is typically the left mouse button, or the right button in a left-handed setup. The primary button. This is typically the left mouse button, or the right button in a left-handed setup."],["BUTTON_SECONDARY","The secondary button. This is typically the right mouse button, or the left button in a left-handed setup. The secondary button. This is typically the right mouse button, or the left button in a left-handed setup."],["EVENT_PROPAGATE","Use this macro as the return value for continuing the propagation of an event handler."],["EVENT_STOP","Use this macro as the return value for stopping the propagation of an event handler."],["NONE_DEVICE_PAD",""],["SELECTION_CLIPBOARD",""],["SELECTION_PRIMARY",""],["SELECTION_SECONDARY",""],["SELECTION_TYPE_ATOM",""],["SELECTION_TYPE_BITMAP",""],["SELECTION_TYPE_COLORMAP",""],["SELECTION_TYPE_DRAWABLE",""],["SELECTION_TYPE_INTEGER",""],["SELECTION_TYPE_PIXMAP",""],["SELECTION_TYPE_STRING",""],["SELECTION_TYPE_WINDOW",""],["TARGET_BITMAP",""],["TARGET_COLORMAP",""],["TARGET_DRAWABLE",""],["TARGET_PIXMAP",""],["TARGET_STRING",""]],"enum":[["AxisUse","An enumeration describing the way in which a device axis (valuator) maps onto the predefined valuator types that GTK+ understands."],["ByteOrder","A set of values describing the possible byte-orders for storing pixel values in memory."],["ChangeData",""],["CrossingMode","Specifies the crossing mode for [`EventCrossing`][crate::EventCrossing]."],["CursorType","Predefined cursors."],["DevicePadFeature","A pad feature."],["DeviceToolType","Indicates the specific type of tool being used being a tablet. Such as an airbrush, pencil, etc."],["DeviceType","Indicates the device type. See [above][GdkDeviceManager.description] for more information about the meaning of these device types."],["DragCancelReason","Used in [`DragContext`][crate::DragContext] to the reason of a cancelled DND operation."],["DragProtocol","Used in [`DragContext`][crate::DragContext] to indicate the protocol according to which DND is done."],["EventType","Specifies the type of the event."],["FullscreenMode","Indicates which monitor (in a multi-head setup) a window should span over when in fullscreen mode."],["GLError","Error enumeration for [`GLContext`][crate::GLContext]."],["GrabOwnership","Defines how device grabs interact with other devices."],["GrabStatus","Returned by [`Device::grab()`][crate::Device::grab()], `gdk_pointer_grab()` and `gdk_keyboard_grab()` to indicate success or the reason for the failure of the grab attempt."],["Gravity","Defines the reference point of a window and the meaning of coordinates passed to `gtk_window_move()`. See `gtk_window_move()` and the “implementation notes” section of the Extended Window Manager Hints specification for more details."],["InputMode","An enumeration that describes the mode of an input device."],["InputSource","An enumeration describing the type of an input device in general terms."],["ModifierIntent","This enum is used with [`Keymap::modifier_mask()`][crate::Keymap::modifier_mask()] in order to determine what modifiers the currently used windowing system backend uses for particular purposes. For example, on X11/Windows, the Control key is used for invoking menu shortcuts (accelerators), whereas on Apple computers it’s the Command key (which correspond to [`ModifierType::CONTROL_MASK`][crate::ModifierType::CONTROL_MASK] and [`ModifierType::MOD2_MASK`][crate::ModifierType::MOD2_MASK], respectively)."],["NotifyType","Specifies the kind of crossing for [`EventCrossing`][crate::EventCrossing]."],["OwnerChange","Specifies why a selection ownership was changed."],["PropMode","Describes how existing data is combined with new data when using `gdk_property_change()`."],["PropertyState","Specifies the type of a property change for a [`EventProperty`][crate::EventProperty]."],["ScrollDirection","Specifies the direction for [`EventScroll`][crate::EventScroll]."],["SettingAction","Specifies the kind of modification applied to a setting in a [`EventSetting`][crate::EventSetting]."],["SubpixelLayout","This enumeration describes how the red, green and blue components of physical pixels on an output device are laid out."],["VisibilityState","Specifies the visiblity status of a window for a [`EventVisibility`][crate::EventVisibility]."],["VisualType","A set of values that describe the manner in which the pixel values for a visual are converted into RGB values for display."],["WindowEdge","Determines a window edge or corner."],["WindowType","Describes the kind of window."],["WindowTypeHint","These are hints for the window manager that indicate what type of function the window has. The window manager can use this when determining decoration and behaviour of the window. The hint must be set before mapping the window."],["WindowWindowClass","[`InputOutput`][Self::InputOutput] windows are the standard kind of window you might expect. Such windows receive events and are also displayed on screen. [`InputOnly`][Self::InputOnly] windows are invisible; they are usually placed above other windows in order to trap or filter the events. You can’t draw on [`InputOnly`][Self::InputOnly] windows."]],"fn":[["beep","Emits a short beep on the default display."],["display_arg_name","Gets the display name specified in the command line arguments passed to `gdk_init()` or [`parse_args()`][crate::parse_args()], if any."],["error_trap_pop","Removes an error trap pushed with [`error_trap_push()`][crate::error_trap_push()]. May block until an error has been definitively received or not received from the X server. [`error_trap_pop_ignored()`][crate::error_trap_pop_ignored()] is preferred if you don’t need to know whether an error occurred, because it never has to block. If you don’t need the return value of [`error_trap_pop()`][crate::error_trap_pop()], use [`error_trap_pop_ignored()`][crate::error_trap_pop_ignored()]."],["error_trap_pop_ignored","Removes an error trap pushed with [`error_trap_push()`][crate::error_trap_push()], but without bothering to wait and see whether an error occurred. If an error arrives later asynchronously that was triggered while the trap was pushed, that error will be ignored."],["error_trap_push","This function allows X errors to be trapped instead of the normal behavior of exiting the application. It should only be used if it is not possible to avoid the X error in any other way. Errors are ignored on all [`Display`][crate::Display] currently known to the [`DisplayManager`][crate::DisplayManager]. If you don’t care which error happens and just want to ignore everything, pop with [`error_trap_pop_ignored()`][crate::error_trap_pop_ignored()]. If you need the error code, use [`error_trap_pop()`][crate::error_trap_pop()] which may have to block and wait for the error to arrive from the X server."],["events_get_angle","If both events contain X/Y information, this function will return [`true`] and return in `angle` the relative angle from `event1` to `event2`. The rotation direction for positive angles is from the positive X axis towards the positive Y axis."],["events_get_center","If both events contain X/Y information, the center of both coordinates will be returned in `x` and `y`."],["events_get_distance","If both events have X/Y information, the distance between both coordinates (as in a straight line going from `event1` to `event2`) will be returned."],["events_pending","Checks if any events are ready to be processed for any display."],["flush","Flushes the output buffers of all display connections and waits until all requests have been processed. This is rarely needed by applications."],["init",""],["list_visuals","Lists the available visuals for the default screen. (See [`Screen::list_visuals()`][crate::Screen::list_visuals()]) A visual describes a hardware image data format. For example, a visual might support 24-bit color, or 8-bit color, and might expect pixels to be in a certain format."],["notify_startup_complete","Indicates to the GUI environment that the application has finished loading. If the applications opens windows, this function is normally called after opening the application’s initial set of windows."],["notify_startup_complete_with_id","Indicates to the GUI environment that the application has finished loading, using a given identifier."],["pango_context_get","Creates a [`pango::Context`][crate::pango::Context] for the default GDK screen."],["pango_context_get_for_display","Creates a [`pango::Context`][crate::pango::Context] for `display`."],["pango_context_get_for_screen","Creates a [`pango::Context`][crate::pango::Context] for `screen`."],["pango_layout_get_clip_region","Obtains a clip region which contains the areas where the given ranges of text would be drawn. `x_origin` and `y_origin` are the top left point to center the layout. `index_ranges` should contain ranges of bytes in the layout’s text."],["pango_layout_line_get_clip_region","Obtains a clip region which contains the areas where the given ranges of text would be drawn. `x_origin` and `y_origin` are the top left position of the layout. `index_ranges` should contain ranges of bytes in the layout’s text. The clip region will include space to the left or right of the line (to the layout bounding box) if you have indexes above or below the indexes contained inside the line. This is to draw the selection all the way to the side of the layout. However, the clip region is in line coordinates, not layout coordinates."],["pixbuf_get_from_surface","Transfers image data from a [`cairo::Surface`][crate::cairo::Surface] and converts it to an RGB(A) representation inside a [`gdk_pixbuf::Pixbuf`][crate::gdk_pixbuf::Pixbuf]. This allows you to efficiently read individual pixels from cairo surfaces. For `GdkWindows`, use `gdk_pixbuf_get_from_window()` instead."],["program_class","Gets the program class. Unless the program class has explicitly been set with [`set_program_class()`][crate::set_program_class()] or with the `--class` commandline option, the default value is the program name (determined with `g_get_prgname()`) with the first character converted to uppercase."],["property_change","Changes the contents of a property on a window."],["property_delete","Deletes a property from a window."],["property_get","Retrieves a portion of the contents of a property. If the property does not exist, then the function returns [`false`], and `GDK_NONE` will be stored in `actual_property_type`."],["query_depths","This function returns the available bit depths for the default screen. It’s equivalent to listing the visuals ([`list_visuals()`][crate::list_visuals()]) and then looking at the depth field in each visual, removing duplicates."],["selection_convert","Retrieves the contents of a selection in a given form."],["selection_owner_get","Determines the owner of the given selection."],["selection_owner_get_for_display","Determine the owner of the given selection."],["selection_owner_set","Sets the owner of the given selection."],["selection_owner_set_for_display","Sets the [`Window`][crate::Window] `owner` as the current owner of the selection `selection`."],["selection_send_notify","Sends a response to SelectionRequest event."],["selection_send_notify_for_display","Send a response to SelectionRequest event."],["set_allowed_backends","Sets a list of backends that GDK should try to use."],["set_double_click_time","Set the double click time for the default display. See [`Display::set_double_click_time()`][crate::Display::set_double_click_time()]. See also [`Display::set_double_click_distance()`][crate::Display::set_double_click_distance()]. Applications should not set this, it is a global user-configured setting."],["set_initialized","Informs this crate that GDK has been initialized and the current thread is the main one."],["set_program_class","Sets the program class. The X11 backend uses the program class to set the class name part of the `WM_CLASS` property on toplevel windows; see the ICCCM."],["set_show_events","Sets whether a trace of received events is output. Note that GTK+ must be compiled with debugging (that is, configured using the `--enable-debug` option) to use this option."],["setting_get","Obtains a desktop-wide setting, such as the double-click time, for the default screen. See [`Screen::is_setting()`][crate::Screen::is_setting()]."],["shows_events","Gets whether event debugging output is enabled."],["synthesize_window_state",""],["test_render_sync","Retrieves a pixel from `window` to force the windowing system to carry out any pending rendering commands."],["test_simulate_button","This function is intended to be used in GTK+ test programs. It will warp the mouse pointer to the given (`x`,`y`) coordinates within `window` and simulate a button press or release event. Because the mouse pointer needs to be warped to the target location, use of this function outside of test programs that run in their own virtual windowing system (e.g. Xvfb) is not recommended."],["test_simulate_key","This function is intended to be used in GTK+ test programs. If (`x`,`y`) are > (-1,-1), it will warp the mouse pointer to the given (`x`,`y`) coordinates within `window` and simulate a key press or release event."],["text_property_to_utf8_list_for_display","Converts a text property in the given encoding to a list of UTF-8 strings."],["utf8_to_string_target","Converts an UTF-8 string into the best possible representation as a STRING. The representation of characters not in STRING is not specified; it may be as pseudo-escape sequences \\x{ABCD}, or it may be in some other form of approximation."]],"mod":[["functions",""],["keys",""],["prelude","Traits intended for blanket imports."]],"struct":[["AnchorHints","Positioning hints for aligning a window relative to a rectangle."],["AppLaunchContext","GdkAppLaunchContext is an implementation of [`gio::AppLaunchContext`][crate::gio::AppLaunchContext] that handles launching an application in a graphical context. It provides startup notification and allows to launch applications on a specific screen or workspace."],["Atom","An opaque type representing a string as an index into a table of strings on the X server."],["AxisFlags","Flags describing the current capabilities of a device/tool."],["Color",""],["Cursor","A [`Cursor`][crate::Cursor] represents a cursor. Its contents are private."],["Device","The [`Device`][crate::Device] object represents a single input device, such as a keyboard, a mouse, a touchpad, etc."],["DeviceManager","In addition to a single pointer and keyboard for user interface input, GDK contains support for a variety of input devices, including graphics tablets, touchscreens and multiple pointers/keyboards interacting simultaneously with the user interface. Such input devices often have additional features, such as sub-pixel positioning information and additional device-dependent information."],["DevicePad","[`DevicePad`][crate::DevicePad] is an interface implemented by devices of type [`InputSource::TabletPad`][crate::InputSource::TabletPad], it allows querying the features provided by the pad device."],["DeviceTool",""],["Display","[`Display`][crate::Display] objects purpose are two fold:"],["DisplayManager","The purpose of the [`DisplayManager`][crate::DisplayManager] singleton object is to offer notification when displays appear or disappear or the default display changes."],["DragAction","Used in [`DragContext`][crate::DragContext] to indicate what the destination should do with the dropped data."],["DragContext",""],["DrawingContext","[`DrawingContext`][crate::DrawingContext] is an object that represents the current drawing state of a [`Window`][crate::Window]."],["Event","A generic GDK event."],["EventButton","Used for button press and button release events. The `type` field will be one of [`EventType::ButtonPress`][crate::EventType::ButtonPress], [`EventType::_2buttonPress`][crate::EventType::_2buttonPress], [`EventType::_3buttonPress`][crate::EventType::_3buttonPress] or [`EventType::ButtonRelease`][crate::EventType::ButtonRelease],"],["EventConfigure","Generated when a window size or position has changed."],["EventCrossing","Generated when the pointer enters or leaves a window."],["EventDND","Generated during DND operations."],["EventExpose","Generated when all or part of a window becomes visible and needs to be redrawn."],["EventFocus","Describes a change of keyboard focus."],["EventGrabBroken","Generated when a pointer or keyboard grab is broken. On X11, this happens when the grab window becomes unviewable (i.e. it or one of its ancestors is unmapped), or if the same application grabs the pointer or keyboard again. Note that implicit grabs (which are initiated by button presses) can also cause [`EventGrabBroken`][crate::EventGrabBroken] events."],["EventKey","Describes a key press or key release event."],["EventMask","A set of bit-flags to indicate which events a window is to receive. Most of these masks map onto one or more of the [`EventType`][crate::EventType] event types above."],["EventMotion","Generated when the pointer moves."],["EventOwnerChange","Generated when the owner of a selection changes. On X11, this information is only available if the X server supports the XFIXES extension."],["EventPadAxis","Generated during [`InputSource::TabletPad`][crate::InputSource::TabletPad] interaction with tactile sensors."],["EventPadButton","Generated during [`InputSource::TabletPad`][crate::InputSource::TabletPad] button presses and releases."],["EventPadGroupMode","Generated during [`InputSource::TabletPad`][crate::InputSource::TabletPad] mode switches in a group."],["EventProperty","Describes a property change on a window."],["EventProximity","Proximity events are generated when using GDK’s wrapper for the XInput extension. The XInput extension is an add-on for standard X that allows you to use nonstandard devices such as graphics tablets. A proximity event indicates that the stylus has moved in or out of contact with the tablet, or perhaps that the user’s finger has moved in or out of contact with a touch screen."],["EventScroll","Generated from button presses for the buttons 4 to 7. Wheel mice are usually configured to generate button press events for buttons 4 and 5 when the wheel is turned."],["EventSelection","Generated when a selection is requested or ownership of a selection is taken over by another client application."],["EventSequence",""],["EventSetting","Generated when a setting is modified."],["EventTouch","Used for touch events. `type` field will be one of [`EventType::TouchBegin`][crate::EventType::TouchBegin], [`EventType::TouchUpdate`][crate::EventType::TouchUpdate], [`EventType::TouchEnd`][crate::EventType::TouchEnd] or [`EventType::TouchCancel`][crate::EventType::TouchCancel]."],["EventTouchpadPinch","Generated during touchpad swipe gestures."],["EventTouchpadSwipe","Generated during touchpad swipe gestures."],["EventVisibility",""],["EventWindowState","Generated when the state of a toplevel window changes."],["FrameClock","A [`FrameClock`][crate::FrameClock] tells the application when to update and repaint a window. This may be synced to the vertical refresh rate of the monitor, for example. Even when the frame clock uses a simple timer rather than a hardware-based vertical sync, the frame clock helps because it ensures everything paints at the same time (reducing the total number of frames). The frame clock can also automatically stop painting when it knows the frames will not be visible, or scale back animation framerates."],["FrameClockPhase","[`FrameClockPhase`][crate::FrameClockPhase] is used to represent the different paint clock phases that can be requested. The elements of the enumeration correspond to the signals of [`FrameClock`][crate::FrameClock]."],["FrameTimings","A [`FrameTimings`][crate::FrameTimings] object holds timing information for a single frame of the application’s displays. To retrieve [`FrameTimings`][crate::FrameTimings] objects, use [`FrameClock::timings()`][crate::FrameClock::timings()] or [`FrameClock::current_timings()`][crate::FrameClock::current_timings()]. The information in [`FrameTimings`][crate::FrameTimings] is useful for precise synchronization of video with the event or audio streams, and for measuring quality metrics for the application’s display, such as latency and jitter."],["GLContext","[`GLContext`][crate::GLContext] is an object representing the platform-specific OpenGL drawing context."],["GRange",""],["Geometry","The [`Geometry`][crate::Geometry] struct gives the window manager information about a window’s geometry constraints. Normally you would set these on the GTK+ level using `gtk_window_set_geometry_hints()`. `GtkWindow` then sets the hints on the [`Window`][crate::Window] it creates."],["Keymap","A [`Keymap`][crate::Keymap] defines the translation from keyboard state (including a hardware key, a modifier mask, and active keyboard group) to a keyval. This translation has two phases. The first phase is to determine the effective keyboard group and level for the keyboard state; the second phase is to look up the keycode/group/level triplet in the keymap and see what keyval it corresponds to."],["KeymapKey","A [`KeymapKey`][crate::KeymapKey] is a hardware key that can be mapped to a keyval."],["ModifierType","A set of bit-flags to indicate the state of modifier keys and mouse buttons in various event types. Typical modifier keys are Shift, Control, Meta, Super, Hyper, Alt, Compose, Apple, CapsLock or ShiftLock."],["Monitor","GdkMonitor objects represent the individual outputs that are associated with a [`Display`][crate::Display]. GdkDisplay has APIs to enumerate monitors with [`Display::n_monitors()`][crate::Display::n_monitors()] and [`Display::monitor()`][crate::Display::monitor()], and to find particular monitors with [`Display::primary_monitor()`][crate::Display::primary_monitor()] or [`Display::monitor_at_window()`][crate::Display::monitor_at_window()]."],["RGBA","A [`RGBA`][crate::RGBA] is used to represent a (possibly translucent) color, in a way that is compatible with cairo’s notion of color."],["Rectangle","Defines the position and size of a rectangle. It is identical to `cairo_rectangle_int_t`."],["RgbaParseError",""],["Screen","[`Screen`][crate::Screen] objects are the GDK representation of the screen on which windows can be displayed and on which the pointer moves. X originally identified screens with physical screens, but nowadays it is more common to have a single [`Screen`][crate::Screen] which combines several physical monitors (see [`n_monitors()`][Self::n_monitors()])."],["Seat","The [`Seat`][crate::Seat] object represents a collection of input devices that belong to a user."],["SeatCapabilities","Flags describing the seat capabilities."],["TimeCoord","A [`TimeCoord`][crate::TimeCoord] stores a single event in a motion history."],["Visual","A [`Visual`][crate::Visual] contains information about a particular visual."],["WMDecoration","These are hints originally defined by the Motif toolkit. The window manager can use them when determining how to decorate the window. The hint must be set before mapping the window."],["WMFunction","These are hints originally defined by the Motif toolkit. The window manager can use them when determining the functions to offer for the window. The hint must be set before mapping the window."],["Window","This is an Abstract Base Class, you cannot instantiate it."],["WindowAttr",""],["WindowHints","Used to indicate which fields of a [`Geometry`][crate::Geometry] struct should be paid attention to. Also, the presence/absence of [`POS`][Self::POS], [`USER_POS`][Self::USER_POS], and [`USER_SIZE`][Self::USER_SIZE] is significant, though they don’t directly refer to [`Geometry`][crate::Geometry] fields. [`USER_POS`][Self::USER_POS] will be set automatically by `GtkWindow` if you call `gtk_window_move()`. [`USER_POS`][Self::USER_POS] and [`USER_SIZE`][Self::USER_SIZE] should be set if the user specified a size/position using a –geometry command-line argument; `gtk_window_parse_geometry()` automatically sets these flags."],["WindowState","Specifies the state of a toplevel window."]],"trait":[["FromEvent","A helper trait implemented by all event subtypes."]],"type":[["key",""]]});