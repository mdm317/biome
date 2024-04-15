/// Sorted array of browser builtin
///
/// Source: <https://github.com/sindresorhus/globals/blob/9e2e2598dabdb845ff76c0c3acf5c52c812a64de/globals.json#L368-L1442>
pub const BROWSER: &[&str; 1073] = &[
    "AbortController",
    "AbortSignal",
    "AbsoluteOrientationSensor",
    "AbstractRange",
    "Accelerometer",
    "AnalyserNode",
    "Animation",
    "AnimationEffect",
    "AnimationEvent",
    "AnimationPlaybackEvent",
    "AnimationTimeline",
    "Attr",
    "Audio",
    "AudioBuffer",
    "AudioBufferSourceNode",
    "AudioContext",
    "AudioData",
    "AudioDecoder",
    "AudioDestinationNode",
    "AudioEncoder",
    "AudioListener",
    "AudioNode",
    "AudioParam",
    "AudioParamMap",
    "AudioProcessingEvent",
    "AudioScheduledSourceNode",
    "AudioSinkInfo",
    "AudioWorklet",
    "AudioWorkletNode",
    "AuthenticatorAssertionResponse",
    "AuthenticatorAttestationResponse",
    "AuthenticatorResponse",
    "BackgroundFetchManager",
    "BackgroundFetchRecord",
    "BackgroundFetchRegistration",
    "BarProp",
    "BaseAudioContext",
    "BatteryManager",
    "BeforeUnloadEvent",
    "BiquadFilterNode",
    "Blob",
    "BlobEvent",
    "Bluetooth",
    "BluetoothCharacteristicProperties",
    "BluetoothDevice",
    "BluetoothRemoteGATTCharacteristic",
    "BluetoothRemoteGATTDescriptor",
    "BluetoothRemoteGATTServer",
    "BluetoothRemoteGATTService",
    "BluetoothUUID",
    "BroadcastChannel",
    "BrowserCaptureMediaStreamTrack",
    "ByteLengthQueuingStrategy",
    "CDATASection",
    "CSS",
    "CSSAnimation",
    "CSSConditionRule",
    "CSSContainerRule",
    "CSSCounterStyleRule",
    "CSSFontFaceRule",
    "CSSFontFeatureValuesRule",
    "CSSFontPaletteValuesRule",
    "CSSGroupingRule",
    "CSSImageValue",
    "CSSImportRule",
    "CSSKeyframeRule",
    "CSSKeyframesRule",
    "CSSKeywordValue",
    "CSSLayerBlockRule",
    "CSSLayerStatementRule",
    "CSSMathClamp",
    "CSSMathInvert",
    "CSSMathMax",
    "CSSMathMin",
    "CSSMathNegate",
    "CSSMathProduct",
    "CSSMathSum",
    "CSSMathValue",
    "CSSMatrixComponent",
    "CSSMediaRule",
    "CSSNamespaceRule",
    "CSSNumericArray",
    "CSSNumericValue",
    "CSSPageRule",
    "CSSPerspective",
    "CSSPositionValue",
    "CSSPropertyRule",
    "CSSRotate",
    "CSSRule",
    "CSSRuleList",
    "CSSScale",
    "CSSScopeRule",
    "CSSSkew",
    "CSSSkewX",
    "CSSSkewY",
    "CSSStartingStyleRule",
    "CSSStyleDeclaration",
    "CSSStyleRule",
    "CSSStyleSheet",
    "CSSStyleValue",
    "CSSSupportsRule",
    "CSSTransformComponent",
    "CSSTransformValue",
    "CSSTransition",
    "CSSTranslate",
    "CSSUnitValue",
    "CSSUnparsedValue",
    "CSSVariableReferenceValue",
    "Cache",
    "CacheStorage",
    "CanvasCaptureMediaStream",
    "CanvasCaptureMediaStreamTrack",
    "CanvasGradient",
    "CanvasPattern",
    "CanvasRenderingContext2D",
    "CaptureController",
    "CaretPosition",
    "ChannelMergerNode",
    "ChannelSplitterNode",
    "CharacterBoundsUpdateEvent",
    "CharacterData",
    "Clipboard",
    "ClipboardEvent",
    "ClipboardItem",
    "CloseEvent",
    "Comment",
    "CompositionEvent",
    "CompressionStream",
    "ConstantSourceNode",
    "ContentVisibilityAutoStateChangeEvent",
    "ConvolverNode",
    "CookieChangeEvent",
    "CookieStore",
    "CookieStoreManager",
    "CountQueuingStrategy",
    "Credential",
    "CredentialsContainer",
    "CropTarget",
    "Crypto",
    "CryptoKey",
    "CustomElementRegistry",
    "CustomEvent",
    "CustomStateSet",
    "DOMError",
    "DOMException",
    "DOMImplementation",
    "DOMMatrix",
    "DOMMatrixReadOnly",
    "DOMParser",
    "DOMPoint",
    "DOMPointReadOnly",
    "DOMQuad",
    "DOMRect",
    "DOMRectList",
    "DOMRectReadOnly",
    "DOMStringList",
    "DOMStringMap",
    "DOMTokenList",
    "DataTransfer",
    "DataTransferItem",
    "DataTransferItemList",
    "DecompressionStream",
    "DelayNode",
    "DelegatedInkTrailPresenter",
    "DeviceMotionEvent",
    "DeviceMotionEventAcceleration",
    "DeviceMotionEventRotationRate",
    "DeviceOrientationEvent",
    "Document",
    "DocumentFragment",
    "DocumentPictureInPicture",
    "DocumentPictureInPictureEvent",
    "DocumentTimeline",
    "DocumentType",
    "DragEvent",
    "DynamicsCompressorNode",
    "EditContext",
    "Element",
    "ElementInternals",
    "EncodedAudioChunk",
    "EncodedVideoChunk",
    "ErrorEvent",
    "Event",
    "EventCounts",
    "EventSource",
    "EventTarget",
    "External",
    "EyeDropper",
    "FeaturePolicy",
    "FederatedCredential",
    "File",
    "FileList",
    "FileReader",
    "FileSystem",
    "FileSystemDirectoryEntry",
    "FileSystemDirectoryHandle",
    "FileSystemDirectoryReader",
    "FileSystemEntry",
    "FileSystemFileEntry",
    "FileSystemFileHandle",
    "FileSystemHandle",
    "FileSystemWritableFileStream",
    "FocusEvent",
    "FontData",
    "FontFace",
    "FontFaceSet",
    "FontFaceSetLoadEvent",
    "FormData",
    "FormDataEvent",
    "FragmentDirective",
    "GPU",
    "GPUAdapter",
    "GPUAdapterInfo",
    "GPUBindGroup",
    "GPUBindGroupLayout",
    "GPUBuffer",
    "GPUBufferUsage",
    "GPUCanvasContext",
    "GPUColorWrite",
    "GPUCommandBuffer",
    "GPUCommandEncoder",
    "GPUCompilationInfo",
    "GPUCompilationMessage",
    "GPUComputePassEncoder",
    "GPUComputePipeline",
    "GPUDevice",
    "GPUDeviceLostInfo",
    "GPUError",
    "GPUExternalTexture",
    "GPUInternalError",
    "GPUMapMode",
    "GPUOutOfMemoryError",
    "GPUPipelineError",
    "GPUPipelineLayout",
    "GPUQuerySet",
    "GPUQueue",
    "GPURenderBundle",
    "GPURenderBundleEncoder",
    "GPURenderPassEncoder",
    "GPURenderPipeline",
    "GPUSampler",
    "GPUShaderModule",
    "GPUShaderStage",
    "GPUSupportedFeatures",
    "GPUSupportedLimits",
    "GPUTexture",
    "GPUTextureUsage",
    "GPUTextureView",
    "GPUUncapturedErrorEvent",
    "GPUValidationError",
    "GainNode",
    "Gamepad",
    "GamepadAxisMoveEvent",
    "GamepadButton",
    "GamepadButtonEvent",
    "GamepadEvent",
    "GamepadHapticActuator",
    "GamepadPose",
    "Geolocation",
    "GeolocationCoordinates",
    "GeolocationPosition",
    "GeolocationPositionError",
    "GravitySensor",
    "Gyroscope",
    "HID",
    "HIDConnectionEvent",
    "HIDDevice",
    "HIDInputReportEvent",
    "HTMLAllCollection",
    "HTMLAnchorElement",
    "HTMLAreaElement",
    "HTMLAudioElement",
    "HTMLBRElement",
    "HTMLBaseElement",
    "HTMLBodyElement",
    "HTMLButtonElement",
    "HTMLCanvasElement",
    "HTMLCollection",
    "HTMLDListElement",
    "HTMLDataElement",
    "HTMLDataListElement",
    "HTMLDetailsElement",
    "HTMLDialogElement",
    "HTMLDirectoryElement",
    "HTMLDivElement",
    "HTMLDocument",
    "HTMLElement",
    "HTMLEmbedElement",
    "HTMLFieldSetElement",
    "HTMLFontElement",
    "HTMLFormControlsCollection",
    "HTMLFormElement",
    "HTMLFrameElement",
    "HTMLFrameSetElement",
    "HTMLHRElement",
    "HTMLHeadElement",
    "HTMLHeadingElement",
    "HTMLHtmlElement",
    "HTMLIFrameElement",
    "HTMLImageElement",
    "HTMLInputElement",
    "HTMLLIElement",
    "HTMLLabelElement",
    "HTMLLegendElement",
    "HTMLLinkElement",
    "HTMLMapElement",
    "HTMLMarqueeElement",
    "HTMLMediaElement",
    "HTMLMenuElement",
    "HTMLMetaElement",
    "HTMLMeterElement",
    "HTMLModElement",
    "HTMLOListElement",
    "HTMLObjectElement",
    "HTMLOptGroupElement",
    "HTMLOptionElement",
    "HTMLOptionsCollection",
    "HTMLOutputElement",
    "HTMLParagraphElement",
    "HTMLParamElement",
    "HTMLPictureElement",
    "HTMLPreElement",
    "HTMLProgressElement",
    "HTMLQuoteElement",
    "HTMLScriptElement",
    "HTMLSelectElement",
    "HTMLSlotElement",
    "HTMLSourceElement",
    "HTMLSpanElement",
    "HTMLStyleElement",
    "HTMLTableCaptionElement",
    "HTMLTableCellElement",
    "HTMLTableColElement",
    "HTMLTableElement",
    "HTMLTableRowElement",
    "HTMLTableSectionElement",
    "HTMLTemplateElement",
    "HTMLTextAreaElement",
    "HTMLTimeElement",
    "HTMLTitleElement",
    "HTMLTrackElement",
    "HTMLUListElement",
    "HTMLUnknownElement",
    "HTMLVideoElement",
    "HashChangeEvent",
    "Headers",
    "Highlight",
    "HighlightRegistry",
    "History",
    "IDBCursor",
    "IDBCursorWithValue",
    "IDBDatabase",
    "IDBFactory",
    "IDBIndex",
    "IDBKeyRange",
    "IDBObjectStore",
    "IDBOpenDBRequest",
    "IDBRequest",
    "IDBTransaction",
    "IDBVersionChangeEvent",
    "IIRFilterNode",
    "IdentityCredential",
    "IdentityCredentialError",
    "IdentityProvider",
    "IdleDeadline",
    "IdleDetector",
    "Image",
    "ImageBitmap",
    "ImageBitmapRenderingContext",
    "ImageCapture",
    "ImageData",
    "ImageDecoder",
    "ImageTrack",
    "ImageTrackList",
    "Ink",
    "InputDeviceCapabilities",
    "InputDeviceInfo",
    "InputEvent",
    "IntersectionObserver",
    "IntersectionObserverEntry",
    "Iterator",
    "Keyboard",
    "KeyboardEvent",
    "KeyboardLayoutMap",
    "KeyframeEffect",
    "LargestContentfulPaint",
    "LaunchParams",
    "LaunchQueue",
    "LayoutShift",
    "LayoutShiftAttribution",
    "LinearAccelerationSensor",
    "Location",
    "Lock",
    "LockManager",
    "MIDIAccess",
    "MIDIConnectionEvent",
    "MIDIInput",
    "MIDIInputMap",
    "MIDIMessageEvent",
    "MIDIOutput",
    "MIDIOutputMap",
    "MIDIPort",
    "MathMLElement",
    "MediaCapabilities",
    "MediaCapabilitiesInfo",
    "MediaDeviceInfo",
    "MediaDevices",
    "MediaElementAudioSourceNode",
    "MediaEncryptedEvent",
    "MediaError",
    "MediaKeyError",
    "MediaKeyMessageEvent",
    "MediaKeySession",
    "MediaKeyStatusMap",
    "MediaKeySystemAccess",
    "MediaKeys",
    "MediaList",
    "MediaMetadata",
    "MediaQueryList",
    "MediaQueryListEvent",
    "MediaRecorder",
    "MediaRecorderErrorEvent",
    "MediaSession",
    "MediaSource",
    "MediaSourceHandle",
    "MediaStream",
    "MediaStreamAudioDestinationNode",
    "MediaStreamAudioSourceNode",
    "MediaStreamEvent",
    "MediaStreamTrack",
    "MediaStreamTrackAudioSourceNode",
    "MediaStreamTrackEvent",
    "MediaStreamTrackGenerator",
    "MediaStreamTrackProcessor",
    "MediaStreamTrackVideoStats",
    "MessageChannel",
    "MessageEvent",
    "MessagePort",
    "MimeType",
    "MimeTypeArray",
    "MouseEvent",
    "MutationEvent",
    "MutationObserver",
    "MutationRecord",
    "NamedNodeMap",
    "NavigateEvent",
    "Navigation",
    "NavigationCurrentEntryChangeEvent",
    "NavigationDestination",
    "NavigationHistoryEntry",
    "NavigationPreloadManager",
    "NavigationTransition",
    "Navigator",
    "NavigatorLogin",
    "NavigatorManagedData",
    "NavigatorUAData",
    "NetworkInformation",
    "Node",
    "NodeFilter",
    "NodeIterator",
    "NodeList",
    "Notification",
    "NotifyPaintEvent",
    "OTPCredential",
    "OfflineAudioCompletionEvent",
    "OfflineAudioContext",
    "OffscreenCanvas",
    "OffscreenCanvasRenderingContext2D",
    "Option",
    "OrientationSensor",
    "OscillatorNode",
    "OverconstrainedError",
    "PERSISTENT",
    "PageTransitionEvent",
    "PannerNode",
    "PasswordCredential",
    "Path2D",
    "PaymentAddress",
    "PaymentManager",
    "PaymentMethodChangeEvent",
    "PaymentRequest",
    "PaymentRequestUpdateEvent",
    "PaymentResponse",
    "Performance",
    "PerformanceElementTiming",
    "PerformanceEntry",
    "PerformanceEventTiming",
    "PerformanceLongTaskTiming",
    "PerformanceMark",
    "PerformanceMeasure",
    "PerformanceNavigation",
    "PerformanceNavigationTiming",
    "PerformanceObserver",
    "PerformanceObserverEntryList",
    "PerformancePaintTiming",
    "PerformanceResourceTiming",
    "PerformanceServerTiming",
    "PerformanceTiming",
    "PeriodicSyncManager",
    "PeriodicWave",
    "PermissionStatus",
    "Permissions",
    "PictureInPictureEvent",
    "PictureInPictureWindow",
    "Plugin",
    "PluginArray",
    "PointerEvent",
    "PopStateEvent",
    "Presentation",
    "PresentationAvailability",
    "PresentationConnection",
    "PresentationConnectionAvailableEvent",
    "PresentationConnectionCloseEvent",
    "PresentationConnectionList",
    "PresentationReceiver",
    "PresentationRequest",
    "ProcessingInstruction",
    "Profiler",
    "ProgressEvent",
    "PromiseRejectionEvent",
    "PublicKeyCredential",
    "PushManager",
    "PushSubscription",
    "PushSubscriptionOptions",
    "RTCCertificate",
    "RTCDTMFSender",
    "RTCDTMFToneChangeEvent",
    "RTCDataChannel",
    "RTCDataChannelEvent",
    "RTCDtlsTransport",
    "RTCEncodedAudioFrame",
    "RTCEncodedVideoFrame",
    "RTCError",
    "RTCErrorEvent",
    "RTCIceCandidate",
    "RTCIceTransport",
    "RTCPeerConnection",
    "RTCPeerConnectionIceErrorEvent",
    "RTCPeerConnectionIceEvent",
    "RTCRtpReceiver",
    "RTCRtpScriptTransform",
    "RTCRtpSender",
    "RTCRtpTransceiver",
    "RTCSctpTransport",
    "RTCSessionDescription",
    "RTCStatsReport",
    "RTCTrackEvent",
    "RadioNodeList",
    "Range",
    "ReadableByteStreamController",
    "ReadableStream",
    "ReadableStreamBYOBReader",
    "ReadableStreamBYOBRequest",
    "ReadableStreamDefaultController",
    "ReadableStreamDefaultReader",
    "RelativeOrientationSensor",
    "RemotePlayback",
    "ReportingObserver",
    "Request",
    "ResizeObserver",
    "ResizeObserverEntry",
    "ResizeObserverSize",
    "Response",
    "SVGAElement",
    "SVGAngle",
    "SVGAnimateElement",
    "SVGAnimateMotionElement",
    "SVGAnimateTransformElement",
    "SVGAnimatedAngle",
    "SVGAnimatedBoolean",
    "SVGAnimatedEnumeration",
    "SVGAnimatedInteger",
    "SVGAnimatedLength",
    "SVGAnimatedLengthList",
    "SVGAnimatedNumber",
    "SVGAnimatedNumberList",
    "SVGAnimatedPreserveAspectRatio",
    "SVGAnimatedRect",
    "SVGAnimatedString",
    "SVGAnimatedTransformList",
    "SVGAnimationElement",
    "SVGCircleElement",
    "SVGClipPathElement",
    "SVGComponentTransferFunctionElement",
    "SVGDefsElement",
    "SVGDescElement",
    "SVGElement",
    "SVGEllipseElement",
    "SVGFEBlendElement",
    "SVGFEColorMatrixElement",
    "SVGFEComponentTransferElement",
    "SVGFECompositeElement",
    "SVGFEConvolveMatrixElement",
    "SVGFEDiffuseLightingElement",
    "SVGFEDisplacementMapElement",
    "SVGFEDistantLightElement",
    "SVGFEDropShadowElement",
    "SVGFEFloodElement",
    "SVGFEFuncAElement",
    "SVGFEFuncBElement",
    "SVGFEFuncGElement",
    "SVGFEFuncRElement",
    "SVGFEGaussianBlurElement",
    "SVGFEImageElement",
    "SVGFEMergeElement",
    "SVGFEMergeNodeElement",
    "SVGFEMorphologyElement",
    "SVGFEOffsetElement",
    "SVGFEPointLightElement",
    "SVGFESpecularLightingElement",
    "SVGFESpotLightElement",
    "SVGFETileElement",
    "SVGFETurbulenceElement",
    "SVGFilterElement",
    "SVGForeignObjectElement",
    "SVGGElement",
    "SVGGeometryElement",
    "SVGGradientElement",
    "SVGGraphicsElement",
    "SVGImageElement",
    "SVGLength",
    "SVGLengthList",
    "SVGLineElement",
    "SVGLinearGradientElement",
    "SVGMPathElement",
    "SVGMarkerElement",
    "SVGMaskElement",
    "SVGMatrix",
    "SVGMetadataElement",
    "SVGNumber",
    "SVGNumberList",
    "SVGPathElement",
    "SVGPatternElement",
    "SVGPoint",
    "SVGPointList",
    "SVGPolygonElement",
    "SVGPolylineElement",
    "SVGPreserveAspectRatio",
    "SVGRadialGradientElement",
    "SVGRect",
    "SVGRectElement",
    "SVGSVGElement",
    "SVGScriptElement",
    "SVGSetElement",
    "SVGStopElement",
    "SVGStringList",
    "SVGStyleElement",
    "SVGSwitchElement",
    "SVGSymbolElement",
    "SVGTSpanElement",
    "SVGTextContentElement",
    "SVGTextElement",
    "SVGTextPathElement",
    "SVGTextPositioningElement",
    "SVGTitleElement",
    "SVGTransform",
    "SVGTransformList",
    "SVGUnitTypes",
    "SVGUseElement",
    "SVGViewElement",
    "Scheduler",
    "Scheduling",
    "Screen",
    "ScreenDetailed",
    "ScreenDetails",
    "ScreenOrientation",
    "ScriptProcessorNode",
    "ScrollTimeline",
    "SecurityPolicyViolationEvent",
    "Selection",
    "Sensor",
    "SensorErrorEvent",
    "Serial",
    "SerialPort",
    "ServiceWorker",
    "ServiceWorkerContainer",
    "ServiceWorkerRegistration",
    "ShadowRoot",
    "SharedWorker",
    "SourceBuffer",
    "SourceBufferList",
    "SpeechSynthesis",
    "SpeechSynthesisErrorEvent",
    "SpeechSynthesisEvent",
    "SpeechSynthesisUtterance",
    "SpeechSynthesisVoice",
    "StaticRange",
    "StereoPannerNode",
    "Storage",
    "StorageBucket",
    "StorageBucketManager",
    "StorageEvent",
    "StorageManager",
    "StylePropertyMap",
    "StylePropertyMapReadOnly",
    "StyleSheet",
    "StyleSheetList",
    "SubmitEvent",
    "SubtleCrypto",
    "SyncManager",
    "TEMPORARY",
    "TaskAttributionTiming",
    "TaskController",
    "TaskPriorityChangeEvent",
    "TaskSignal",
    "Text",
    "TextDecoder",
    "TextDecoderStream",
    "TextEncoder",
    "TextEncoderStream",
    "TextEvent",
    "TextFormat",
    "TextFormatUpdateEvent",
    "TextMetrics",
    "TextTrack",
    "TextTrackCue",
    "TextTrackCueList",
    "TextTrackList",
    "TextUpdateEvent",
    "TimeEvent",
    "TimeRanges",
    "ToggleEvent",
    "Touch",
    "TouchEvent",
    "TouchList",
    "TrackEvent",
    "TransformStream",
    "TransformStreamDefaultController",
    "TransitionEvent",
    "TreeWalker",
    "TrustedHTML",
    "TrustedScript",
    "TrustedScriptURL",
    "TrustedTypePolicy",
    "TrustedTypePolicyFactory",
    "UIEvent",
    "URL",
    "URLPattern",
    "URLSearchParams",
    "USB",
    "USBAlternateInterface",
    "USBConfiguration",
    "USBConnectionEvent",
    "USBDevice",
    "USBEndpoint",
    "USBInTransferResult",
    "USBInterface",
    "USBIsochronousInTransferPacket",
    "USBIsochronousInTransferResult",
    "USBIsochronousOutTransferPacket",
    "USBIsochronousOutTransferResult",
    "USBOutTransferResult",
    "UserActivation",
    "VTTCue",
    "VTTRegion",
    "ValidityState",
    "VideoColorSpace",
    "VideoDecoder",
    "VideoEncoder",
    "VideoFrame",
    "VideoPlaybackQuality",
    "ViewTimeline",
    "ViewTransition",
    "VirtualKeyboard",
    "VirtualKeyboardGeometryChangeEvent",
    "VisibilityStateEntry",
    "VisualViewport",
    "WGSLLanguageFeatures",
    "WakeLock",
    "WakeLockSentinel",
    "WaveShaperNode",
    "WebAssembly",
    "WebGL2RenderingContext",
    "WebGLActiveInfo",
    "WebGLBuffer",
    "WebGLContextEvent",
    "WebGLFramebuffer",
    "WebGLProgram",
    "WebGLQuery",
    "WebGLRenderbuffer",
    "WebGLRenderingContext",
    "WebGLSampler",
    "WebGLShader",
    "WebGLShaderPrecisionFormat",
    "WebGLSync",
    "WebGLTexture",
    "WebGLTransformFeedback",
    "WebGLUniformLocation",
    "WebGLVertexArrayObject",
    "WebSocket",
    "WebTransport",
    "WebTransportBidirectionalStream",
    "WebTransportDatagramDuplexStream",
    "WebTransportError",
    "WebTransportReceiveStream",
    "WebTransportSendStream",
    "WheelEvent",
    "Window",
    "WindowControlsOverlay",
    "WindowControlsOverlayGeometryChangeEvent",
    "Worker",
    "Worklet",
    "WritableStream",
    "WritableStreamDefaultController",
    "WritableStreamDefaultWriter",
    "XMLDocument",
    "XMLHttpRequest",
    "XMLHttpRequestEventTarget",
    "XMLHttpRequestUpload",
    "XMLSerializer",
    "XPathEvaluator",
    "XPathExpression",
    "XPathResult",
    "XRAnchor",
    "XRAnchorSet",
    "XRBoundedReferenceSpace",
    "XRCPUDepthInformation",
    "XRCamera",
    "XRDOMOverlayState",
    "XRDepthInformation",
    "XRFrame",
    "XRHitTestResult",
    "XRHitTestSource",
    "XRInputSource",
    "XRInputSourceArray",
    "XRInputSourceEvent",
    "XRInputSourcesChangeEvent",
    "XRLayer",
    "XRLightEstimate",
    "XRLightProbe",
    "XRPose",
    "XRRay",
    "XRReferenceSpace",
    "XRReferenceSpaceEvent",
    "XRRenderState",
    "XRRigidTransform",
    "XRSession",
    "XRSessionEvent",
    "XRSpace",
    "XRSystem",
    "XRTransientInputHitTestResult",
    "XRTransientInputHitTestSource",
    "XRView",
    "XRViewerPose",
    "XRViewport",
    "XRWebGLBinding",
    "XRWebGLDepthInformation",
    "XRWebGLLayer",
    "XSLTProcessor",
    "addEventListener",
    "alert",
    "atob",
    "blur",
    "btoa",
    "caches",
    "cancelAnimationFrame",
    "cancelIdleCallback",
    "clearInterval",
    "clearTimeout",
    "clientInformation",
    "close",
    "closed",
    "confirm",
    "console",
    "cookieStore",
    "createImageBitmap",
    "credentialless",
    "crossOriginIsolated",
    "crypto",
    "customElements",
    "devicePixelRatio",
    "dispatchEvent",
    "document",
    "documentPictureInPicture",
    "event",
    "external",
    "fetch",
    "find",
    "focus",
    "frameElement",
    "frames",
    "getComputedStyle",
    "getScreenDetails",
    "getSelection",
    "history",
    "indexedDB",
    "innerHeight",
    "innerWidth",
    "isSecureContext",
    "launchQueue",
    "length",
    "localStorage",
    "location",
    "locationbar",
    "matchMedia",
    "menubar",
    "moveBy",
    "moveTo",
    "name",
    "navigation",
    "navigator",
    "offscreenBuffering",
    "onabort",
    "onafterprint",
    "onanimationcancel",
    "onanimationend",
    "onanimationiteration",
    "onanimationstart",
    "onappinstalled",
    "onauxclick",
    "onbeforeinput",
    "onbeforeinstallprompt",
    "onbeforematch",
    "onbeforeprint",
    "onbeforetoggle",
    "onbeforeunload",
    "onbeforexrselect",
    "onblur",
    "oncancel",
    "oncanplay",
    "oncanplaythrough",
    "onchange",
    "onclick",
    "onclose",
    "oncontentvisibilityautostatechange",
    "oncontextlost",
    "oncontextmenu",
    "oncontextrestored",
    "oncopy",
    "oncuechange",
    "oncut",
    "ondblclick",
    "ondevicemotion",
    "ondeviceorientation",
    "ondeviceorientationabsolute",
    "ondrag",
    "ondragend",
    "ondragenter",
    "ondragleave",
    "ondragover",
    "ondragstart",
    "ondrop",
    "ondurationchange",
    "onemptied",
    "onended",
    "onerror",
    "onfocus",
    "onformdata",
    "ongamepadconnected",
    "ongamepaddisconnected",
    "ongotpointercapture",
    "onhashchange",
    "oninput",
    "oninvalid",
    "onkeydown",
    "onkeypress",
    "onkeyup",
    "onlanguagechange",
    "onload",
    "onloadeddata",
    "onloadedmetadata",
    "onloadstart",
    "onlostpointercapture",
    "onmessage",
    "onmessageerror",
    "onmousedown",
    "onmouseenter",
    "onmouseleave",
    "onmousemove",
    "onmouseout",
    "onmouseover",
    "onmouseup",
    "onmousewheel",
    "onoffline",
    "ononline",
    "onpagehide",
    "onpageshow",
    "onpaste",
    "onpause",
    "onplay",
    "onplaying",
    "onpointercancel",
    "onpointerdown",
    "onpointerenter",
    "onpointerleave",
    "onpointermove",
    "onpointerout",
    "onpointerover",
    "onpointerrawupdate",
    "onpointerup",
    "onpopstate",
    "onprogress",
    "onratechange",
    "onrejectionhandled",
    "onreset",
    "onresize",
    "onscroll",
    "onscrollend",
    "onsearch",
    "onsecuritypolicyviolation",
    "onseeked",
    "onseeking",
    "onselect",
    "onselectionchange",
    "onselectstart",
    "onslotchange",
    "onstalled",
    "onstorage",
    "onsubmit",
    "onsuspend",
    "ontimeupdate",
    "ontoggle",
    "ontransitioncancel",
    "ontransitionend",
    "ontransitionrun",
    "ontransitionstart",
    "onunhandledrejection",
    "onunload",
    "onvolumechange",
    "onwaiting",
    "onwheel",
    "open",
    "opener",
    "origin",
    "originAgentCluster",
    "outerHeight",
    "outerWidth",
    "pageXOffset",
    "pageYOffset",
    "parent",
    "performance",
    "personalbar",
    "postMessage",
    "print",
    "prompt",
    "queryLocalFonts",
    "queueMicrotask",
    "removeEventListener",
    "reportError",
    "requestAnimationFrame",
    "requestIdleCallback",
    "resizeBy",
    "resizeTo",
    "scheduler",
    "screen",
    "screenLeft",
    "screenTop",
    "screenX",
    "screenY",
    "scroll",
    "scrollBy",
    "scrollTo",
    "scrollX",
    "scrollY",
    "scrollbars",
    "self",
    "sessionStorage",
    "setInterval",
    "setTimeout",
    "showDirectoryPicker",
    "showOpenFilePicker",
    "showSaveFilePicker",
    "speechSynthesis",
    "status",
    "statusbar",
    "stop",
    "structuredClone",
    "styleMedia",
    "toolbar",
    "top",
    "trustedTypes",
    "visualViewport",
    "window",
];

/// Sorted array of browser service worker builtin
///
/// Source: <https://github.com/sindresorhus/globals/blob/9e2e2598dabdb845ff76c0c3acf5c52c812a64de/globals.json#L2212-L2334>
pub const SERVICE_WORKER: &[&str; 121] = &[
    "Blob",
    "BroadcastChannel",
    "ByteLengthQueuingStrategy",
    "Cache",
    "CacheStorage",
    "Client",
    "Clients",
    "CompressionStream",
    "CountQueuingStrategy",
    "Crypto",
    "CryptoKey",
    "CustomEvent",
    "DecompressionStream",
    "ErrorEvent",
    "Event",
    "ExtendableEvent",
    "ExtendableMessageEvent",
    "FetchEvent",
    "File",
    "FileReaderSync",
    "FormData",
    "Headers",
    "IDBCursor",
    "IDBCursorWithValue",
    "IDBDatabase",
    "IDBFactory",
    "IDBIndex",
    "IDBKeyRange",
    "IDBObjectStore",
    "IDBOpenDBRequest",
    "IDBRequest",
    "IDBTransaction",
    "IDBVersionChangeEvent",
    "ImageData",
    "MessageChannel",
    "MessageEvent",
    "MessagePort",
    "Notification",
    "Performance",
    "PerformanceEntry",
    "PerformanceMark",
    "PerformanceMeasure",
    "PerformanceNavigation",
    "PerformanceObserver",
    "PerformanceObserverEntryList",
    "PerformanceResourceTiming",
    "PerformanceTiming",
    "ReadableByteStreamController",
    "ReadableStream",
    "ReadableStreamBYOBReader",
    "ReadableStreamBYOBRequest",
    "ReadableStreamDefaultController",
    "ReadableStreamDefaultReader",
    "Request",
    "Response",
    "ServiceWorker",
    "ServiceWorkerContainer",
    "ServiceWorkerGlobalScope",
    "ServiceWorkerMessageEvent",
    "ServiceWorkerRegistration",
    "SubtleCrypto",
    "TextDecoder",
    "TextDecoderStream",
    "TextEncoder",
    "TextEncoderStream",
    "TransformStream",
    "TransformStreamDefaultController",
    "URL",
    "URLSearchParams",
    "WebAssembly",
    "WebSocket",
    "WindowClient",
    "Worker",
    "WorkerGlobalScope",
    "WritableStream",
    "WritableStreamDefaultController",
    "WritableStreamDefaultWriter",
    "XMLHttpRequest",
    "addEventListener",
    "applicationCache",
    "atob",
    "btoa",
    "caches",
    "clearInterval",
    "clearTimeout",
    "clients",
    "close",
    "console",
    "crypto",
    "fetch",
    "importScripts",
    "indexedDB",
    "location",
    "name",
    "navigator",
    "onclose",
    "onconnect",
    "onerror",
    "onfetch",
    "oninstall",
    "onlanguagechange",
    "onmessage",
    "onmessageerror",
    "onnotificationclick",
    "onnotificationclose",
    "onoffline",
    "ononline",
    "onpush",
    "onpushsubscriptionchange",
    "onrejectionhandled",
    "onsync",
    "onunhandledrejection",
    "performance",
    "postMessage",
    "queueMicrotask",
    "registration",
    "removeEventListener",
    "self",
    "setInterval",
    "setTimeout",
    "skipWaiting",
];

/// Sorted array of browser web worker builtin
///
/// Source: <https://github.com/sindresorhus/globals/blob/9e2e2598dabdb845ff76c0c3acf5c52c812a64de/globals.json#L1443-L1726>
pub const WEB_WORKER: &[&str; 282] = &[
    "AbortController",
    "AbortSignal",
    "AudioData",
    "AudioDecoder",
    "AudioEncoder",
    "BackgroundFetchManager",
    "BackgroundFetchRecord",
    "BackgroundFetchRegistration",
    "Blob",
    "BroadcastChannel",
    "ByteLengthQueuingStrategy",
    "CSSSkewX",
    "CSSSkewY",
    "Cache",
    "CacheStorage",
    "CanvasGradient",
    "CanvasPattern",
    "CloseEvent",
    "CompressionStream",
    "CountQueuingStrategy",
    "CropTarget",
    "Crypto",
    "CryptoKey",
    "CustomEvent",
    "DOMException",
    "DOMMatrix",
    "DOMMatrixReadOnly",
    "DOMPoint",
    "DOMPointReadOnly",
    "DOMQuad",
    "DOMRect",
    "DOMRectReadOnly",
    "DOMStringList",
    "DecompressionStream",
    "DedicatedWorkerGlobalScope",
    "EncodedAudioChunk",
    "EncodedVideoChunk",
    "ErrorEvent",
    "Event",
    "EventSource",
    "EventTarget",
    "File",
    "FileList",
    "FileReader",
    "FileReaderSync",
    "FileSystemDirectoryHandle",
    "FileSystemFileHandle",
    "FileSystemHandle",
    "FileSystemSyncAccessHandle",
    "FileSystemWritableFileStream",
    "FontFace",
    "FormData",
    "GPU",
    "GPUAdapter",
    "GPUAdapterInfo",
    "GPUBindGroup",
    "GPUBindGroupLayout",
    "GPUBuffer",
    "GPUBufferUsage",
    "GPUCanvasContext",
    "GPUColorWrite",
    "GPUCommandBuffer",
    "GPUCommandEncoder",
    "GPUCompilationInfo",
    "GPUCompilationMessage",
    "GPUComputePassEncoder",
    "GPUComputePipeline",
    "GPUDevice",
    "GPUDeviceLostInfo",
    "GPUError",
    "GPUExternalTexture",
    "GPUInternalError",
    "GPUMapMode",
    "GPUOutOfMemoryError",
    "GPUPipelineError",
    "GPUPipelineLayout",
    "GPUQuerySet",
    "GPUQueue",
    "GPURenderBundle",
    "GPURenderBundleEncoder",
    "GPURenderPassEncoder",
    "GPURenderPipeline",
    "GPUSampler",
    "GPUShaderModule",
    "GPUShaderStage",
    "GPUSupportedFeatures",
    "GPUSupportedLimits",
    "GPUTexture",
    "GPUTextureUsage",
    "GPUTextureView",
    "GPUUncapturedErrorEvent",
    "GPUValidationError",
    "Headers",
    "IDBCursor",
    "IDBCursorWithValue",
    "IDBDatabase",
    "IDBFactory",
    "IDBIndex",
    "IDBKeyRange",
    "IDBObjectStore",
    "IDBOpenDBRequest",
    "IDBRequest",
    "IDBTransaction",
    "IDBVersionChangeEvent",
    "IdleDetector",
    "ImageBitmap",
    "ImageBitmapRenderingContext",
    "ImageData",
    "ImageDecoder",
    "ImageTrack",
    "ImageTrackList",
    "Iterator",
    "Lock",
    "LockManager",
    "MediaCapabilities",
    "MediaSource",
    "MediaSourceHandle",
    "MessageChannel",
    "MessageEvent",
    "MessagePort",
    "NavigationPreloadManager",
    "NavigatorUAData",
    "NetworkInformation",
    "Notification",
    "OffscreenCanvas",
    "OffscreenCanvasRenderingContext2D",
    "PERSISTENT",
    "Path2D",
    "Performance",
    "PerformanceEntry",
    "PerformanceMark",
    "PerformanceMeasure",
    "PerformanceObserver",
    "PerformanceObserverEntryList",
    "PerformanceResourceTiming",
    "PerformanceServerTiming",
    "PeriodicSyncManager",
    "PermissionStatus",
    "Permissions",
    "ProgressEvent",
    "PromiseRejectionEvent",
    "PushManager",
    "PushSubscription",
    "PushSubscriptionOptions",
    "RTCEncodedAudioFrame",
    "RTCEncodedVideoFrame",
    "ReadableByteStreamController",
    "ReadableStream",
    "ReadableStreamBYOBReader",
    "ReadableStreamBYOBRequest",
    "ReadableStreamDefaultController",
    "ReadableStreamDefaultReader",
    "ReportingObserver",
    "Request",
    "Response",
    "Scheduler",
    "SecurityPolicyViolationEvent",
    "Serial",
    "SerialPort",
    "ServiceWorkerRegistration",
    "SourceBuffer",
    "SourceBufferList",
    "StorageBucket",
    "StorageBucketManager",
    "StorageManager",
    "SubtleCrypto",
    "SyncManager",
    "TEMPORARY",
    "TaskController",
    "TaskPriorityChangeEvent",
    "TaskSignal",
    "TextDecoder",
    "TextDecoderStream",
    "TextEncoder",
    "TextEncoderStream",
    "TextMetrics",
    "TransformStream",
    "TransformStreamDefaultController",
    "TrustedHTML",
    "TrustedScript",
    "TrustedScriptURL",
    "TrustedTypePolicy",
    "TrustedTypePolicyFactory",
    "URL",
    "URLPattern",
    "URLSearchParams",
    "USB",
    "USBAlternateInterface",
    "USBConfiguration",
    "USBConnectionEvent",
    "USBDevice",
    "USBEndpoint",
    "USBInTransferResult",
    "USBInterface",
    "USBIsochronousInTransferPacket",
    "USBIsochronousInTransferResult",
    "USBIsochronousOutTransferPacket",
    "USBIsochronousOutTransferResult",
    "USBOutTransferResult",
    "UserActivation",
    "VideoColorSpace",
    "VideoDecoder",
    "VideoEncoder",
    "VideoFrame",
    "WGSLLanguageFeatures",
    "WebAssembly",
    "WebGL2RenderingContext",
    "WebGLActiveInfo",
    "WebGLBuffer",
    "WebGLContextEvent",
    "WebGLFramebuffer",
    "WebGLProgram",
    "WebGLQuery",
    "WebGLRenderbuffer",
    "WebGLRenderingContext",
    "WebGLSampler",
    "WebGLShader",
    "WebGLShaderPrecisionFormat",
    "WebGLSync",
    "WebGLTexture",
    "WebGLTransformFeedback",
    "WebGLUniformLocation",
    "WebGLVertexArrayObject",
    "WebSocket",
    "WebTransport",
    "WebTransportBidirectionalStream",
    "WebTransportDatagramDuplexStream",
    "WebTransportError",
    "Worker",
    "WorkerGlobalScope",
    "WorkerLocation",
    "WorkerNavigator",
    "WritableStream",
    "WritableStreamDefaultController",
    "WritableStreamDefaultWriter",
    "XMLHttpRequest",
    "XMLHttpRequestEventTarget",
    "XMLHttpRequestUpload",
    "addEventListener",
    "atob",
    "btoa",
    "caches",
    "cancelAnimationFrame",
    "clearInterval",
    "clearTimeout",
    "close",
    "console",
    "createImageBitmap",
    "crossOriginIsolated",
    "crypto",
    "dispatchEvent",
    "fetch",
    "fonts",
    "importScripts",
    "indexedDB",
    "isSecureContext",
    "location",
    "name",
    "navigator",
    "onerror",
    "onlanguagechange",
    "onmessage",
    "onmessageerror",
    "onrejectionhandled",
    "onunhandledrejection",
    "origin",
    "performance",
    "postMessage",
    "queueMicrotask",
    "removeEventListener",
    "reportError",
    "requestAnimationFrame",
    "scheduler",
    "self",
    "setInterval",
    "setTimeout",
    "structuredClone",
    "trustedTypes",
    "webkitRequestFileSystem",
    "webkitRequestFileSystemSync",
    "webkitResolveLocalFileSystemSyncURL",
    "webkitResolveLocalFileSystemURL",
];

/// Returns `true` if `name` is a web global
///
/// ```
/// use biome_js_analyze::globals::javascript::web::is_global;
///
/// assert!(is_global(&"AbortController"));
/// ```
pub fn is_global(name: &str) -> bool {
    BROWSER.binary_search(&name).is_ok()
        || WEB_WORKER.binary_search(&name).is_ok()
        || SERVICE_WORKER.binary_search(&name).is_ok()
}

#[test]
fn test_order() {
    for items in BROWSER.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in SERVICE_WORKER.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in WEB_WORKER.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
