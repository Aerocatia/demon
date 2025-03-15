#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(deprecated)]

pub mod c;

pub const GLOBAL_D3D_DEVICE: VariableProvider<Option<&mut IDirect3DDevice9>> = variable! {
    name: "global_d3d_device",
    cache_address: 0x00DFB330
};

pub unsafe fn d3d_device() -> &'static mut IDirect3DDevice9 {
    GLOBAL_D3D_DEVICE.get_copied().expect("global_d3d_device not loaded")
}

use windows_sys::Win32::Foundation::{BOOL, HANDLE, HWND};
use core::ffi::{c_uint, c_ulong, c_int, c_void, c_long, c_ushort, c_char};
use tag_structs::primitives::color::Pixel32;
use crate::util::VariableProvider;

#[repr(transparent)]
pub struct IDirect3D9([u8; 0]);
#[repr(transparent)]
pub struct IDirect3DQuery9([u8; 0]);
#[repr(transparent)]
pub struct IDirect3DVertexShader9([u8; 0]);
#[repr(transparent)]
pub struct IDirect3DVertexDeclaration9([u8; 0]);
#[repr(transparent)]
pub struct IDirect3DPixelShader9([u8; 0]);
#[repr(transparent)]
pub struct IDirect3DTexture9([u8; 0]);
#[repr(transparent)]
pub struct IDirect3DVertexBuffer9([u8; 0]);
#[repr(transparent)]
pub struct IDirect3DVolumeTexture9([u8; 0]);
#[repr(transparent)]
pub struct IDirect3DIndexBuffer9([u8; 0]);
#[repr(transparent)]
pub struct IDirect3DCubeTexture9([u8; 0]);
#[repr(transparent)]
pub struct IDirect3DBaseTexture9([u8; 0]);
#[repr(transparent)]
pub struct IDirect3DSurface9([u8; 0]);
#[repr(transparent)]
pub struct IDirect3DSwapChain9([u8; 0]);
#[repr(transparent)]
pub struct IDirect3DStateBlock9([u8; 0]);

#[repr(transparent)]
#[deprecated(note = "Not actually deprecated, but a warning before you use this: you need to fill out this struct with actual fields, or else fun things will happen!")]
pub struct D3DCAPS9([u8; 0]);
#[repr(transparent)]
#[deprecated(note = "Not actually deprecated, but a warning before you use this: you need to fill out this struct with actual fields, or else fun things will happen!")]
pub struct D3DPRESENT_PARAMETERS([u8; 0]);
#[repr(transparent)]
#[deprecated(note = "Not actually deprecated, but a warning before you use this: you need to fill out this struct with actual fields, or else fun things will happen!")]
pub struct RGNDATA([u8; 0]);
#[repr(transparent)]
#[deprecated(note = "Not actually deprecated, but a warning before you use this: you need to fill out this struct with actual fields, or else fun things will happen!")]
pub struct D3DRASTER_STATUS([u8; 0]);
#[repr(transparent)]
#[deprecated(note = "Not actually deprecated, but a warning before you use this: you need to fill out this struct with actual fields, or else fun things will happen!")]
pub struct D3DMATRIX([u8; 0]);
#[repr(transparent)]
#[deprecated(note = "Not actually deprecated, but a warning before you use this: you need to fill out this struct with actual fields, or else fun things will happen!")]
pub struct D3D9VIEWPORT9([u8; 0]);
#[repr(transparent)]
#[deprecated(note = "Not actually deprecated, but a warning before you use this: you need to fill out this struct with actual fields, or else fun things will happen!")]
pub struct D3DMATERIAL9([u8; 0]);
#[repr(transparent)]
#[deprecated(note = "Not actually deprecated, but a warning before you use this: you need to fill out this struct with actual fields, or else fun things will happen!")]
pub struct D3DLIGHT9([u8; 0]);
#[repr(transparent)]
#[deprecated(note = "Not actually deprecated, but a warning before you use this: you need to fill out this struct with actual fields, or else fun things will happen!")]
pub struct PALETTEENTRY([u8; 0]);
#[repr(transparent)]
#[deprecated(note = "Not actually deprecated, but a warning before you use this: you need to fill out this struct with actual fields, or else fun things will happen!")]
pub struct D3DVERTEXELEMENT9([u8; 0]);
#[repr(transparent)]
#[deprecated(note = "Not actually deprecated, but a warning before you use this: you need to fill out this struct with actual fields, or else fun things will happen!")]
pub struct D3DRECTPATCH_INFO([u8; 0]);
#[repr(transparent)]
#[deprecated(note = "Not actually deprecated, but a warning before you use this: you need to fill out this struct with actual fields, or else fun things will happen!")]
pub struct D3DTRIPATCH_INFO([u8; 0]);

#[repr(C)]
pub struct GUID {
    Data1: c_ulong,
    Data2: c_ushort,
    Data3: c_ushort,
    Data4: [c_char; 8]
}
pub type REFIID = *mut GUID;
#[repr(C)]
pub struct D3DDEVICE_CREATION_PARAMETERS {
    pub AdapterOrdinal: c_uint,
    pub DeviceType: D3DDEVICETYPE,
    pub hFocusWindow: HWND,
    pub BehaviorFlags: u32
}

#[repr(C)]
pub struct D3DGAMMARAMP {
    pub red: [u16; 256],
    pub green: [u16; 256],
    pub blue: [u16; 256],
}

#[repr(C)]
pub struct D3DDISPLAYMODE {
    pub width: c_uint,
    pub height: c_uint,
    pub refresh_rate: c_uint,
    pub format: D3DFORMAT
}
#[repr(C)]
pub struct D3DCLIPSTATUS9 {
    pub ClipUnion: u32,
    pub ClipIntersection: u32
}
#[repr(C)]
pub struct D3DRECT {
    pub x1: c_long,
    pub y1: c_long,
    pub x2: c_long,
    pub y2: c_long,
}
#[repr(C)]
pub struct RECT {
    left: c_long,
    top: c_long,
    right: c_long,
    bottom: c_long
}

pub type D3DPOOL = u32;
pub type D3DTEXTUREFILTERTYPE = u32;
pub type D3DTRANSFORMSTATETYPE = u32;
pub type D3DMULTISAMPLE_TYPE = u32;
pub type D3DCOLOR = Pixel32;
pub type D3DDEVICETYPE = u32;
pub type D3DQUERYTYPE = u32;
pub type D3DTEXTURESTAGESTATETYPE = u32;
pub type D3DSAMPLERSTATETYPE = u32;
pub type D3DPRIMITIVETYPE = u32;
pub type D3DRENDERSTATETYPE = u32;
pub type D3DSTATEBLOCKTYPE = u32;
pub type D3DBACKBUFFER_TYPE = u32;
pub type D3DFORMAT = u32;

#[repr(C)]
pub struct IDirect3DDevice9 {
    vtable: *mut D3D9DeviceExVtable
}
impl IDirect3DDevice9 {
    pub unsafe fn QueryInterface(&mut self, riid: REFIID, ppvObj: *mut *mut c_void) -> () {
        ((&*self.vtable).QueryInterface)(self, riid, ppvObj)
    }
    pub unsafe fn AddRef(&mut self) -> c_ulong {
        ((&*self.vtable).AddRef)(self)
    }
    pub unsafe fn Release(&mut self) -> c_ulong {
        ((&*self.vtable).Release)(self)
    }
    pub unsafe fn TestCooperativeLevel(&mut self) -> () {
        ((&*self.vtable).TestCooperativeLevel)(self)
    }
    pub unsafe fn GetAvailableTextureMem(&mut self) -> c_uint {
        ((&*self.vtable).GetAvailableTextureMem)(self)
    }
    pub unsafe fn EvictManagedResources(&mut self) -> () {
        ((&*self.vtable).EvictManagedResources)(self)
    }
    pub unsafe fn GetDirect3D(&mut self, ppD3D9: *mut *mut IDirect3D9) -> () {
        ((&*self.vtable).GetDirect3D)(self, ppD3D9)
    }
    pub unsafe fn GetDeviceCaps(&mut self, pCaps: *mut D3DCAPS9) -> () {
        ((&*self.vtable).GetDeviceCaps)(self, pCaps)
    }
    pub unsafe fn GetDisplayMode(&mut self, iSwapChain: c_uint, pMode: *mut D3DDISPLAYMODE) -> () {
        ((&*self.vtable).GetDisplayMode)(self, iSwapChain, pMode)
    }
    pub unsafe fn GetCreationParameters(&mut self, pParameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> () {
        ((&*self.vtable).GetCreationParameters)(self, pParameters)
    }
    pub unsafe fn SetCursorProperties(&mut self, XHotSpot: c_uint, YHotSpot: c_uint, pCursorBitmap: *mut IDirect3DSurface9) -> () {
        ((&*self.vtable).SetCursorProperties)(self, XHotSpot, YHotSpot, pCursorBitmap)
    }
    pub unsafe fn SetCursorPosition(&mut self, X: c_int, Y: c_int, Flags: u32) -> c_void {
        ((&*self.vtable).SetCursorPosition)(self, X, Y, Flags)
    }
    pub unsafe fn ShowCursor(&mut self, bShow: BOOL) -> BOOL {
        ((&*self.vtable).ShowCursor)(self, bShow)
    }
    pub unsafe fn CreateAdditionalSwapChain(&mut self, pPresentationParameters: *mut D3DPRESENT_PARAMETERS, pSwapChain: *mut *mut IDirect3DSwapChain9) -> () {
        ((&*self.vtable).CreateAdditionalSwapChain)(self, pPresentationParameters, pSwapChain)
    }
    pub unsafe fn GetSwapChain(&mut self, iSwapChain: c_uint, pSwapChain: *mut *mut IDirect3DSwapChain9) -> () {
        ((&*self.vtable).GetSwapChain)(self, iSwapChain, pSwapChain)
    }
    pub unsafe fn GetNumberOfSwapChains(&mut self) -> c_uint {
        ((&*self.vtable).GetNumberOfSwapChains)(self)
    }
    pub unsafe fn Reset(&mut self, pPresentationParameters: *mut D3DPRESENT_PARAMETERS) -> () {
        ((&*self.vtable).Reset)(self, pPresentationParameters)
    }
    pub unsafe fn Present(&mut self, pSourceRect: *const RECT, pDestRect: *const RECT, hDestWindowOverride: HWND, pDirtyRegion: *const RGNDATA) -> () {
        ((&*self.vtable).Present)(self, pSourceRect, pDestRect, hDestWindowOverride, pDirtyRegion)
    }
    pub unsafe fn GetBackBuffer(&mut self, iSwapChain: c_uint, iBackBuffer: c_uint, Type: D3DBACKBUFFER_TYPE, ppBackBuffer: *mut *mut IDirect3DSurface9) -> () {
        ((&*self.vtable).GetBackBuffer)(self, iSwapChain, iBackBuffer, Type, ppBackBuffer)
    }
    pub unsafe fn GetRasterStatus(&mut self, iSwapChain: c_uint, pRasterStatus: *mut D3DRASTER_STATUS) -> () {
        ((&*self.vtable).GetRasterStatus)(self, iSwapChain, pRasterStatus)
    }
    pub unsafe fn SetDialogBoxMode(&mut self, bEnableDialogs: BOOL) -> () {
        ((&*self.vtable).SetDialogBoxMode)(self, bEnableDialogs)
    }
    pub unsafe fn SetGammaRamp(&mut self, iSwapChain: c_uint, Flags: u32, pRamp: &D3DGAMMARAMP) -> () {
        ((&*self.vtable).SetGammaRamp)(self, iSwapChain, Flags, pRamp)
    }
    pub unsafe fn GetGammaRamp(&mut self, iSwapChain: c_uint, pRamp: &mut D3DGAMMARAMP) -> () {
        ((&*self.vtable).GetGammaRamp)(self, iSwapChain, pRamp)
    }
    pub unsafe fn CreateTexture(&mut self, Width: c_uint, Height: c_uint, Levels: c_uint, Usage: u32, Format: D3DFORMAT, Pool: D3DPOOL, ppTexture: *mut *mut IDirect3DTexture9, pSharedHandle: *mut HANDLE) -> () {
        ((&*self.vtable).CreateTexture)(self, Width, Height, Levels, Usage, Format, Pool, ppTexture, pSharedHandle)
    }
    pub unsafe fn CreateVolumeTexture(&mut self, Width: c_uint, Height: c_uint, Depth: c_uint, Levels: c_uint, Usage: u32, Format: D3DFORMAT, Pool: D3DPOOL, ppVolumeTexture: *mut *mut IDirect3DVolumeTexture9, pSharedHandle: *mut HANDLE) -> () {
        ((&*self.vtable).CreateVolumeTexture)(self, Width, Height, Depth, Levels, Usage, Format, Pool, ppVolumeTexture, pSharedHandle)
    }
    pub unsafe fn CreateCubeTexture(&mut self, EdgeLength: c_uint, Levels: c_uint, Usage: u32, Format: D3DFORMAT, Pool: D3DPOOL, ppCubeTexture: *mut *mut IDirect3DCubeTexture9, pSharedHandle: *mut HANDLE) -> () {
        ((&*self.vtable).CreateCubeTexture)(self, EdgeLength, Levels, Usage, Format, Pool, ppCubeTexture, pSharedHandle)
    }
    pub unsafe fn CreateVertexBuffer(&mut self, Length: c_uint, Usage: u32, FVF: u32, Pool: D3DPOOL, ppVertexBuffer: *mut *mut IDirect3DVertexBuffer9, pSharedHandle: *mut HANDLE) -> () {
        ((&*self.vtable).CreateVertexBuffer)(self, Length, Usage, FVF, Pool, ppVertexBuffer, pSharedHandle)
    }
    pub unsafe fn CreateIndexBuffer(&mut self, Length: c_uint, Usage: u32, Format: D3DFORMAT, Pool: D3DPOOL, ppIndexBuffer: *mut *mut IDirect3DIndexBuffer9, pSharedHandle: *mut HANDLE) -> () {
        ((&*self.vtable).CreateIndexBuffer)(self, Length, Usage, Format, Pool, ppIndexBuffer, pSharedHandle)
    }
    pub unsafe fn CreateRenderTarget(&mut self, Width: c_uint, Height: c_uint, Format: D3DFORMAT, MultiSample: D3DMULTISAMPLE_TYPE, MultisampleQuality: u32, Lockable: BOOL, ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut HANDLE) -> () {
        ((&*self.vtable).CreateRenderTarget)(self, Width, Height, Format, MultiSample, MultisampleQuality, Lockable, ppSurface, pSharedHandle)
    }
    pub unsafe fn CreateDepthStencilSurface(&mut self, Width: c_uint, Height: c_uint, Format: D3DFORMAT, MultiSample: D3DMULTISAMPLE_TYPE, MultisampleQuality: u32, Discard: BOOL, ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut HANDLE) -> () {
        ((&*self.vtable).CreateDepthStencilSurface)(self, Width, Height, Format, MultiSample, MultisampleQuality, Discard, ppSurface, pSharedHandle)
    }
    pub unsafe fn UpdateSurface(&mut self, pSourceSurface: *mut IDirect3DSurface9, pSourceRect: *const RECT, pDestinationSurface: *mut IDirect3DSurface9, pDestPoint: *const RECT) -> () {
        ((&*self.vtable).UpdateSurface)(self, pSourceSurface, pSourceRect, pDestinationSurface, pDestPoint)
    }
    pub unsafe fn UpdateTexture(&mut self, pSourceTexture: *mut IDirect3DBaseTexture9, pDestinationTexture: *mut IDirect3DBaseTexture9) -> () {
        ((&*self.vtable).UpdateTexture)(self, pSourceTexture, pDestinationTexture)
    }
    pub unsafe fn GetRenderTargetData(&mut self, pRenderTarget: *mut IDirect3DSurface9, pDestSurface: *mut IDirect3DSurface9) -> () {
        ((&*self.vtable).GetRenderTargetData)(self, pRenderTarget, pDestSurface)
    }
    pub unsafe fn GetFrontBufferData(&mut self, iSwapChain: c_uint, pDestSurface: *mut IDirect3DSurface9) -> () {
        ((&*self.vtable).GetFrontBufferData)(self, iSwapChain, pDestSurface)
    }
    pub unsafe fn StretchRect(&mut self, pSourceSurface: *mut IDirect3DSurface9, pSourceRect: *const RECT, pDestSurface: *mut IDirect3DSurface9, pDestRect: *const RECT, Filter: D3DTEXTUREFILTERTYPE) -> () {
        ((&*self.vtable).StretchRect)(self, pSourceSurface, pSourceRect, pDestSurface, pDestRect, Filter)
    }
    pub unsafe fn ColorFill(&mut self, pSurface: *mut IDirect3DSurface9, pRect: *const RECT, color: D3DCOLOR) -> () {
        ((&*self.vtable).ColorFill)(self, pSurface, pRect, color)
    }
    pub unsafe fn CreateOffscreenPlainSurface(&mut self, Width: c_uint, Height: c_uint, Format: D3DFORMAT, Pool: D3DPOOL, ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut HANDLE) -> () {
        ((&*self.vtable).CreateOffscreenPlainSurface)(self, Width, Height, Format, Pool, ppSurface, pSharedHandle)
    }
    pub unsafe fn SetRenderTarget(&mut self, RenderTargetIndex: u32, pRenderTarget: *mut IDirect3DSurface9) -> () {
        ((&*self.vtable).SetRenderTarget)(self, RenderTargetIndex, pRenderTarget)
    }
    pub unsafe fn GetRenderTarget(&mut self, RenderTargetIndex: u32, ppRenderTarget: *mut *mut IDirect3DSurface9) -> () {
        ((&*self.vtable).GetRenderTarget)(self, RenderTargetIndex, ppRenderTarget)
    }
    pub unsafe fn SetDepthStencilSurface(&mut self, pNewZStencil: *mut IDirect3DSurface9) -> () {
        ((&*self.vtable).SetDepthStencilSurface)(self, pNewZStencil)
    }
    pub unsafe fn GetDepthStencilSurface(&mut self, ppZStencilSurface: *mut *mut IDirect3DSurface9) -> () {
        ((&*self.vtable).GetDepthStencilSurface)(self, ppZStencilSurface)
    }
    pub unsafe fn BeginScene(&mut self) -> () {
        ((&*self.vtable).BeginScene)(self)
    }
    pub unsafe fn EndScene(&mut self) -> () {
        ((&*self.vtable).EndScene)(self)
    }
    pub unsafe fn Clear(&mut self, Count: u32, pRects: *const D3DRECT, Flags: u32, Color: D3DCOLOR, Z: f32, Stencil: u32) -> () {
        ((&*self.vtable).Clear)(self, Count, pRects, Flags, Color, Z, Stencil)
    }
    pub unsafe fn SetTransform(&mut self, State: D3DTRANSFORMSTATETYPE, pMatrix: *const D3DMATRIX) -> () {
        ((&*self.vtable).SetTransform)(self, State, pMatrix)
    }
    pub unsafe fn GetTransform(&mut self, State: D3DTRANSFORMSTATETYPE, pMatrix: *const D3DMATRIX) -> () {
        ((&*self.vtable).GetTransform)(self, State, pMatrix)
    }
    pub unsafe fn MultiplyTransform(&mut self, State: D3DTRANSFORMSTATETYPE, pMatrix: *const D3DMATRIX) -> () {
        ((&*self.vtable).MultiplyTransform)(self, State, pMatrix)
    }
    pub unsafe fn SetViewport(&mut self, pViewport: *const D3D9VIEWPORT9) -> () {
        ((&*self.vtable).SetViewport)(self, pViewport)
    }
    pub unsafe fn GetViewport(&mut self, pViewport: *mut D3D9VIEWPORT9) -> () {
        ((&*self.vtable).GetViewport)(self, pViewport)
    }
    pub unsafe fn SetMaterial(&mut self, pMaterial: *const D3DMATERIAL9) -> () {
        ((&*self.vtable).SetMaterial)(self, pMaterial)
    }
    pub unsafe fn GetMaterial(&mut self, pMaterial: *mut D3DMATERIAL9) -> () {
        ((&*self.vtable).GetMaterial)(self, pMaterial)
    }
    pub unsafe fn SetLight(&mut self, Index: u32, light: *const D3DLIGHT9) -> () {
        ((&*self.vtable).SetLight)(self, Index, light)
    }
    pub unsafe fn GetLight(&mut self, Index: u32, light: *mut D3DLIGHT9) -> () {
        ((&*self.vtable).GetLight)(self, Index, light)
    }
    pub unsafe fn LightEnable(&mut self, Index: u32, Enable: BOOL) -> () {
        ((&*self.vtable).LightEnable)(self, Index, Enable)
    }
    pub unsafe fn GetLightEnable(&mut self, Index: u32, pEnable: *mut BOOL) -> () {
        ((&*self.vtable).GetLightEnable)(self, Index, pEnable)
    }
    pub unsafe fn SetClipPlane(&mut self, Index: u32, pPlane: *const f32) -> () {
        ((&*self.vtable).SetClipPlane)(self, Index, pPlane)
    }
    pub unsafe fn GetClipPlane(&mut self, Index: u32, pPlane: *mut f32) -> () {
        ((&*self.vtable).GetClipPlane)(self, Index, pPlane)
    }
    pub unsafe fn SetRenderState(&mut self, State: D3DRENDERSTATETYPE, Value: u32) -> () {
        ((&*self.vtable).SetRenderState)(self, State, Value)
    }
    pub unsafe fn GetRenderState(&mut self, State: D3DRENDERSTATETYPE, pValue: *mut u32) -> () {
        ((&*self.vtable).GetRenderState)(self, State, pValue)
    }
    pub unsafe fn CreateStateBlock(&mut self, Type: D3DSTATEBLOCKTYPE, ppSB: *mut *mut IDirect3DStateBlock9) -> () {
        ((&*self.vtable).CreateStateBlock)(self, Type, ppSB)
    }
    pub unsafe fn BeginStateBlock(&mut self) -> () {
        ((&*self.vtable).BeginStateBlock)(self)
    }
    pub unsafe fn EndStateBlock(&mut self, ppSB: *mut *mut IDirect3DStateBlock9) -> () {
        ((&*self.vtable).EndStateBlock)(self, ppSB)
    }
    pub unsafe fn SetClipStatus(&mut self, pClipStatus: *const D3DCLIPSTATUS9) -> () {
        ((&*self.vtable).SetClipStatus)(self, pClipStatus)
    }
    pub unsafe fn GetClipStatus(&mut self, pClipStatus: *mut D3DCLIPSTATUS9) -> () {
        ((&*self.vtable).GetClipStatus)(self, pClipStatus)
    }
    pub unsafe fn GetTexture(&mut self, Stage: u32, ppTexture: *mut *mut IDirect3DBaseTexture9) -> () {
        ((&*self.vtable).GetTexture)(self, Stage, ppTexture)
    }
    pub unsafe fn SetTexture(&mut self, Stage: u32, pTexture: *mut IDirect3DBaseTexture9) -> () {
        ((&*self.vtable).SetTexture)(self, Stage, pTexture)
    }
    pub unsafe fn GetTextureStageState(&mut self, Stage: u32, Type: D3DTEXTURESTAGESTATETYPE, pValue: *mut u32) -> () {
        ((&*self.vtable).GetTextureStageState)(self, Stage, Type, pValue)
    }
    pub unsafe fn SetTextureStageState(&mut self, Stage: u32, Type: D3DTEXTURESTAGESTATETYPE, Value: u32) -> () {
        ((&*self.vtable).SetTextureStageState)(self, Stage, Type, Value)
    }
    pub unsafe fn GetSamplerState(&mut self, Sampler: u32, Type: D3DSAMPLERSTATETYPE, pValue: *mut u32) -> () {
        ((&*self.vtable).GetSamplerState)(self, Sampler, Type, pValue)
    }
    pub unsafe fn SetSamplerState(&mut self, Sampler: u32, Type: D3DSAMPLERSTATETYPE, Value: u32) -> () {
        ((&*self.vtable).SetSamplerState)(self, Sampler, Type, Value)
    }
    pub unsafe fn ValidateDevice(&mut self, pNumPasses: *mut u32) -> () {
        ((&*self.vtable).ValidateDevice)(self, pNumPasses)
    }
    pub unsafe fn SetPaletteEntries(&mut self, PaletteNumber: c_uint, pEntries: *const PALETTEENTRY) -> () {
        ((&*self.vtable).SetPaletteEntries)(self, PaletteNumber, pEntries)
    }
    pub unsafe fn GetPaletteEntries(&mut self, PaletteNumber: c_uint, pEntries: *mut PALETTEENTRY) -> () {
        ((&*self.vtable).GetPaletteEntries)(self, PaletteNumber, pEntries)
    }
    pub unsafe fn SetCurrentTexturePalette(&mut self, PaletteNumber: c_uint) -> () {
        ((&*self.vtable).SetCurrentTexturePalette)(self, PaletteNumber)
    }
    pub unsafe fn GetCurrentTexturePalette(&mut self, PaletteNumber: *mut c_uint) -> () {
        ((&*self.vtable).GetCurrentTexturePalette)(self, PaletteNumber)
    }
    pub unsafe fn SetScissorRect(&mut self, pRect: *const RECT) -> () {
        ((&*self.vtable).SetScissorRect)(self, pRect)
    }
    pub unsafe fn GetScissorRect(&mut self, pRect: *mut RECT) -> () {
        ((&*self.vtable).GetScissorRect)(self, pRect)
    }
    pub unsafe fn SetSoftwareVertexProcessing(&mut self, bSoftware: BOOL) -> () {
        ((&*self.vtable).SetSoftwareVertexProcessing)(self, bSoftware)
    }
    pub unsafe fn GetSoftwareVertexProcessing(&mut self) -> BOOL {
        ((&*self.vtable).GetSoftwareVertexProcessing)(self)
    }
    pub unsafe fn SetNPatchMode(&mut self, nSegments: f32) -> () {
        ((&*self.vtable).SetNPatchMode)(self, nSegments)
    }
    pub unsafe fn GetNPatchMode(&mut self) -> f32 {
        ((&*self.vtable).GetNPatchMode)(self)
    }
    pub unsafe fn DrawPrimitive(&mut self, PrimitiveType: D3DPRIMITIVETYPE, StartVertex: c_uint, PrimitiveCount: c_uint) -> () {
        ((&*self.vtable).DrawPrimitive)(self, PrimitiveType, StartVertex, PrimitiveCount)
    }
    pub unsafe fn DrawIndexedPrimitive(&mut self, PrimitiveType: D3DPRIMITIVETYPE, BaseVertexIndex: c_int, MinVertexIndex: c_uint, NumVertices: c_uint, startIndex: c_uint, primCount: c_uint) -> () {
        ((&*self.vtable).DrawIndexedPrimitive)(self, PrimitiveType, BaseVertexIndex, MinVertexIndex, NumVertices, startIndex, primCount)
    }
    pub unsafe fn DrawPrimitiveUP(&mut self, PrimitiveType: D3DPRIMITIVETYPE, PrimitiveCount: c_uint, pVertexStreamZeroData: *const c_void, VertexStreamZeroStride: c_uint) -> () {
        ((&*self.vtable).DrawPrimitiveUP)(self, PrimitiveType, PrimitiveCount, pVertexStreamZeroData, VertexStreamZeroStride)
    }
    pub unsafe fn DrawIndexedPrimitiveUP(&mut self, PrimitiveType: D3DPRIMITIVETYPE, MinVertexIndex: c_uint, NumVertices: c_uint, PrimitiveCount: c_uint, pIndexData: *const c_void, IndexDataFormat: D3DFORMAT, pVertexStreamZeroData: *const c_void, VertexStreamZeroStride: c_uint) -> () {
        ((&*self.vtable).DrawIndexedPrimitiveUP)(self, PrimitiveType, MinVertexIndex, NumVertices, PrimitiveCount, pIndexData, IndexDataFormat, pVertexStreamZeroData, VertexStreamZeroStride)
    }
    pub unsafe fn ProcessVertices(&mut self, SrcStartIndex: c_uint, DestIndex: c_uint, VertexCount: c_uint, pDestBuffer: *mut IDirect3DVertexBuffer9, pVertexDecl: *mut IDirect3DVertexDeclaration9, Flags: u32) -> () {
        ((&*self.vtable).ProcessVertices)(self, SrcStartIndex, DestIndex, VertexCount, pDestBuffer, pVertexDecl, Flags)
    }
    pub unsafe fn CreateVertexDeclaration(&mut self, pVertexElements: *const D3DVERTEXELEMENT9, ppDecl: *mut *mut IDirect3DVertexDeclaration9) -> () {
        ((&*self.vtable).CreateVertexDeclaration)(self, pVertexElements, ppDecl)
    }
    pub unsafe fn SetVertexDeclaration(&mut self, pDecl: *mut IDirect3DVertexDeclaration9) -> () {
        ((&*self.vtable).SetVertexDeclaration)(self, pDecl)
    }
    pub unsafe fn GetVertexDeclaration(&mut self, ppDecl: *mut *mut IDirect3DVertexDeclaration9) -> () {
        ((&*self.vtable).GetVertexDeclaration)(self, ppDecl)
    }
    pub unsafe fn SetFVF(&mut self, FVF: u32) -> () {
        ((&*self.vtable).SetFVF)(self, FVF)
    }
    pub unsafe fn GetFVF(&mut self, pFVF: *mut u32) -> () {
        ((&*self.vtable).GetFVF)(self, pFVF)
    }
    pub unsafe fn CreateVertexShader(&mut self, pFunction: *const u32, ppShader: *mut *mut IDirect3DVertexShader9) -> () {
        ((&*self.vtable).CreateVertexShader)(self, pFunction, ppShader)
    }
    pub unsafe fn SetVertexShader(&mut self, pShader: *mut IDirect3DVertexShader9) -> () {
        ((&*self.vtable).SetVertexShader)(self, pShader)
    }
    pub unsafe fn GetVertexShader(&mut self, ppShader: *mut *mut IDirect3DVertexShader9) -> () {
        ((&*self.vtable).GetVertexShader)(self, ppShader)
    }
    pub unsafe fn SetVertexShaderConstantF(&mut self, StartRegister: c_uint, pConstantData: *const f32, Vector4fCount: c_uint) -> () {
        ((&*self.vtable).SetVertexShaderConstantF)(self, StartRegister, pConstantData, Vector4fCount)
    }
    pub unsafe fn GetVertexShaderConstantF(&mut self, StartRegister: c_uint, pConstantData: *mut f32, Vector4fCount: c_uint) -> () {
        ((&*self.vtable).GetVertexShaderConstantF)(self, StartRegister, pConstantData, Vector4fCount)
    }
    pub unsafe fn SetVertexShaderConstantI(&mut self, StartRegister: c_uint, pConstantData: *const c_int, Vector4iCount: c_uint) -> () {
        ((&*self.vtable).SetVertexShaderConstantI)(self, StartRegister, pConstantData, Vector4iCount)
    }
    pub unsafe fn GetVertexShaderConstantI(&mut self, StartRegister: c_uint, pConstantData: *mut c_int, Vector4iCount: c_uint) -> () {
        ((&*self.vtable).GetVertexShaderConstantI)(self, StartRegister, pConstantData, Vector4iCount)
    }
    pub unsafe fn SetVertexShaderConstantB(&mut self, StartRegister: c_uint, pConstantData: *const BOOL, BoolCount: c_uint) -> () {
        ((&*self.vtable).SetVertexShaderConstantB)(self, StartRegister, pConstantData, BoolCount)
    }
    pub unsafe fn GetVertexShaderConstantB(&mut self, StartRegister: c_uint, pConstantData: *mut BOOL, BoolCount: c_uint) -> () {
        ((&*self.vtable).GetVertexShaderConstantB)(self, StartRegister, pConstantData, BoolCount)
    }
    pub unsafe fn SetStreamSource(&mut self, StreamNumber: c_uint, pStreamData: *mut IDirect3DVertexBuffer9, OffsetInBytes: c_uint, Stride: c_uint) -> () {
        ((&*self.vtable).SetStreamSource)(self, StreamNumber, pStreamData, OffsetInBytes, Stride)
    }
    pub unsafe fn GetStreamSource(&mut self, StreamNumber: c_uint, ppStreamData: *mut *mut IDirect3DVertexBuffer9, pOffsetInBytes: *mut c_uint, pStride: *mut c_uint) -> () {
        ((&*self.vtable).GetStreamSource)(self, StreamNumber, ppStreamData, pOffsetInBytes, pStride)
    }
    pub unsafe fn SetStreamSourceFreq(&mut self, StreamNumber: c_uint, Setting: c_uint) -> () {
        ((&*self.vtable).SetStreamSourceFreq)(self, StreamNumber, Setting)
    }
    pub unsafe fn GetStreamSourceFreq(&mut self, StreamNumber: c_uint, pSetting: *mut c_uint) -> () {
        ((&*self.vtable).GetStreamSourceFreq)(self, StreamNumber, pSetting)
    }
    pub unsafe fn SetIndices(&mut self, pIndexData: *mut IDirect3DIndexBuffer9) -> () {
        ((&*self.vtable).SetIndices)(self, pIndexData)
    }
    pub unsafe fn GetIndices(&mut self, ppIndexData: *mut *mut IDirect3DIndexBuffer9) -> () {
        ((&*self.vtable).GetIndices)(self, ppIndexData)
    }
    pub unsafe fn CreatePixelShader(&mut self, pFunction: *const u32, ppShader: *mut *mut IDirect3DPixelShader9) -> () {
        ((&*self.vtable).CreatePixelShader)(self, pFunction, ppShader)
    }
    pub unsafe fn SetPixelShader(&mut self, pShader: *mut IDirect3DPixelShader9) -> () {
        ((&*self.vtable).SetPixelShader)(self, pShader)
    }
    pub unsafe fn GetPixelShader(&mut self, ppShader: *mut *mut IDirect3DPixelShader9) -> () {
        ((&*self.vtable).GetPixelShader)(self, ppShader)
    }
    pub unsafe fn SetPixelShaderConstantF(&mut self, StartRegister: c_uint, pConstantData: *const f32, Vector4fCount: c_uint) -> () {
        ((&*self.vtable).SetPixelShaderConstantF)(self, StartRegister, pConstantData, Vector4fCount)
    }
    pub unsafe fn GetPixelShaderConstantF(&mut self, StartRegister: c_uint, pConstantData: *mut f32, Vector4fCount: c_uint) -> () {
        ((&*self.vtable).GetPixelShaderConstantF)(self, StartRegister, pConstantData, Vector4fCount)
    }
    pub unsafe fn SetPixelShaderConstantI(&mut self, StartRegister: c_uint, pConstantData: *const c_int, Vector4iCount: c_uint) -> () {
        ((&*self.vtable).SetPixelShaderConstantI)(self, StartRegister, pConstantData, Vector4iCount)
    }
    pub unsafe fn GetPixelShaderConstantI(&mut self, StartRegister: c_uint, pConstantData: *mut c_int, Vector4iCount: c_uint) -> () {
        ((&*self.vtable).GetPixelShaderConstantI)(self, StartRegister, pConstantData, Vector4iCount)
    }
    pub unsafe fn SetPixelShaderConstantB(&mut self, StartRegister: c_uint, pConstantData: *const BOOL, BoolCount: c_uint) -> () {
        ((&*self.vtable).SetPixelShaderConstantB)(self, StartRegister, pConstantData, BoolCount)
    }
    pub unsafe fn GetPixelShaderConstantB(&mut self, StartRegister: c_uint, pConstantData: *mut BOOL, BoolCount: c_uint) -> () {
        ((&*self.vtable).GetPixelShaderConstantB)(self, StartRegister, pConstantData, BoolCount)
    }
    pub unsafe fn DrawRectPatch(&mut self, Handle: c_uint, pNumSegs: *const f32, pRectPatchInfo: *mut D3DRECTPATCH_INFO) -> () {
        ((&*self.vtable).DrawRectPatch)(self, Handle, pNumSegs, pRectPatchInfo)
    }
    pub unsafe fn DrawTriPatch(&mut self, Handle: c_uint, pNumSegs: *const f32, pTriPatchInfo: *mut D3DTRIPATCH_INFO) -> () {
        ((&*self.vtable).DrawTriPatch)(self, Handle, pNumSegs, pTriPatchInfo)
    }
    pub unsafe fn DeletePatch(&mut self, Handle: c_uint) -> () {
        ((&*self.vtable).DeletePatch)(self, Handle)
    }
    pub unsafe fn CreateQuery(&mut self, Type: D3DQUERYTYPE, ppQuery: *mut *mut IDirect3DQuery9) -> () {
        ((&*self.vtable).CreateQuery)(self, Type, ppQuery)
    }
}

#[repr(C)]
#[allow(non_snake_case)]
struct D3D9DeviceExVtable {
    QueryInterface: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, riid: REFIID, ppvObj: *mut *mut c_void) -> (),
    AddRef: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9) -> c_ulong,
    Release: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9) -> c_ulong,
    TestCooperativeLevel: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9) -> (),
    GetAvailableTextureMem: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9) -> c_uint,
    EvictManagedResources: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9) -> (),
    GetDirect3D: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, ppD3D9: *mut *mut IDirect3D9) -> (),
    GetDeviceCaps: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pCaps: *mut D3DCAPS9) -> (),
    GetDisplayMode: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, iSwapChain: c_uint, pMode: *mut D3DDISPLAYMODE) -> (),
    GetCreationParameters: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pParameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> (),
    SetCursorProperties: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, XHotSpot: c_uint, YHotSpot: c_uint, pCursorBitmap: *mut IDirect3DSurface9) -> (),
    SetCursorPosition: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, X: c_int, Y: c_int, Flags: u32) -> c_void,
    ShowCursor: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, bShow: BOOL) -> BOOL,
    CreateAdditionalSwapChain: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pPresentationParameters: *mut D3DPRESENT_PARAMETERS, pSwapChain: *mut *mut IDirect3DSwapChain9) -> (),
    GetSwapChain: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, iSwapChain: c_uint, pSwapChain: *mut *mut IDirect3DSwapChain9) -> (),
    GetNumberOfSwapChains: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9) -> c_uint,
    Reset: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pPresentationParameters: *mut D3DPRESENT_PARAMETERS) -> (),
    Present: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pSourceRect: *const RECT, pDestRect: *const RECT, hDestWindowOverride: HWND, pDirtyRegion: *const RGNDATA) -> (),
    GetBackBuffer: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, iSwapChain: c_uint, iBackBuffer: c_uint, Type: D3DBACKBUFFER_TYPE, ppBackBuffer: *mut *mut IDirect3DSurface9) -> (),
    GetRasterStatus: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, iSwapChain: c_uint, pRasterStatus: *mut D3DRASTER_STATUS) -> (),
    SetDialogBoxMode: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, bEnableDialogs: BOOL) -> (),
    SetGammaRamp: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, iSwapChain: c_uint, Flags: u32, pRamp: *const D3DGAMMARAMP) -> (),
    GetGammaRamp: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, iSwapChain: c_uint, pRamp: *mut D3DGAMMARAMP) -> (),
    CreateTexture: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Width: c_uint, Height: c_uint, Levels: c_uint, Usage: u32, Format: D3DFORMAT, Pool: D3DPOOL, ppTexture: *mut *mut IDirect3DTexture9, pSharedHandle: *mut HANDLE) -> (),
    CreateVolumeTexture: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Width: c_uint, Height: c_uint, Depth: c_uint, Levels: c_uint, Usage: u32, Format: D3DFORMAT, Pool: D3DPOOL, ppVolumeTexture: *mut *mut IDirect3DVolumeTexture9, pSharedHandle: *mut HANDLE) -> (),
    CreateCubeTexture: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, EdgeLength: c_uint, Levels: c_uint, Usage: u32, Format: D3DFORMAT, Pool: D3DPOOL, ppCubeTexture: *mut *mut IDirect3DCubeTexture9, pSharedHandle: *mut HANDLE) -> (),
    CreateVertexBuffer: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Length: c_uint, Usage: u32, FVF: u32, Pool: D3DPOOL, ppVertexBuffer: *mut *mut IDirect3DVertexBuffer9, pSharedHandle: *mut HANDLE) -> (),
    CreateIndexBuffer: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Length: c_uint, Usage: u32, Format: D3DFORMAT, Pool: D3DPOOL, ppIndexBuffer: *mut *mut IDirect3DIndexBuffer9, pSharedHandle: *mut HANDLE) -> (),
    CreateRenderTarget: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Width: c_uint, Height: c_uint, Format: D3DFORMAT, MultiSample: D3DMULTISAMPLE_TYPE, MultisampleQuality: u32, Lockable: BOOL, ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut HANDLE) -> (),
    CreateDepthStencilSurface: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Width: c_uint, Height: c_uint, Format: D3DFORMAT, MultiSample: D3DMULTISAMPLE_TYPE, MultisampleQuality: u32, Discard: BOOL, ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut HANDLE) -> (),
    UpdateSurface: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pSourceSurface: *mut IDirect3DSurface9, pSourceRect: *const RECT, pDestinationSurface: *mut IDirect3DSurface9, pDestPoint: *const RECT) -> (),
    UpdateTexture: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pSourceTexture: *mut IDirect3DBaseTexture9, pDestinationTexture: *mut IDirect3DBaseTexture9) -> (),
    GetRenderTargetData: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pRenderTarget: *mut IDirect3DSurface9, pDestSurface: *mut IDirect3DSurface9) -> (),
    GetFrontBufferData: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, iSwapChain: c_uint, pDestSurface: *mut IDirect3DSurface9) -> (),
    StretchRect: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pSourceSurface: *mut IDirect3DSurface9, pSourceRect: *const RECT, pDestSurface: *mut IDirect3DSurface9, pDestRect: *const RECT, Filter: D3DTEXTUREFILTERTYPE) -> (),
    ColorFill: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pSurface: *mut IDirect3DSurface9, pRect: *const RECT, color: D3DCOLOR) -> (),
    CreateOffscreenPlainSurface: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Width: c_uint, Height: c_uint, Format: D3DFORMAT, Pool: D3DPOOL, ppSurface: *mut *mut IDirect3DSurface9, pSharedHandle: *mut HANDLE) -> (),
    SetRenderTarget: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, RenderTargetIndex: u32, pRenderTarget: *mut IDirect3DSurface9) -> (),
    GetRenderTarget: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, RenderTargetIndex: u32, ppRenderTarget: *mut *mut IDirect3DSurface9) -> (),
    SetDepthStencilSurface: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pNewZStencil: *mut IDirect3DSurface9) -> (),
    GetDepthStencilSurface: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, ppZStencilSurface: *mut *mut IDirect3DSurface9) -> (),
    BeginScene: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9) -> (),
    EndScene: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9) -> (),
    Clear: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Count: u32, pRects: *const D3DRECT, Flags: u32, Color: D3DCOLOR, Z: f32, Stencil: u32) -> (),
    SetTransform: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, State: D3DTRANSFORMSTATETYPE, pMatrix: *const D3DMATRIX) -> (),
    GetTransform: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, State: D3DTRANSFORMSTATETYPE, pMatrix: *const D3DMATRIX) -> (),
    MultiplyTransform: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, State: D3DTRANSFORMSTATETYPE, pMatrix: *const D3DMATRIX) -> (),
    SetViewport: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pViewport: *const D3D9VIEWPORT9) -> (),
    GetViewport: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pViewport: *mut D3D9VIEWPORT9) -> (),
    SetMaterial: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pMaterial: *const D3DMATERIAL9) -> (),
    GetMaterial: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pMaterial: *mut D3DMATERIAL9) -> (),
    SetLight: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Index: u32, light: *const D3DLIGHT9) -> (),
    GetLight: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Index: u32, light: *mut D3DLIGHT9) -> (),
    LightEnable: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Index: u32, Enable: BOOL) -> (),
    GetLightEnable: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Index: u32, pEnable: *mut BOOL) -> (),
    SetClipPlane: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Index: u32, pPlane: *const f32) -> (),
    GetClipPlane: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Index: u32, pPlane: *mut f32) -> (),
    SetRenderState: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, State: D3DRENDERSTATETYPE, Value: u32) -> (),
    GetRenderState: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, State: D3DRENDERSTATETYPE, pValue: *mut u32) -> (),
    CreateStateBlock: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Type: D3DSTATEBLOCKTYPE, ppSB: *mut *mut IDirect3DStateBlock9) -> (),
    BeginStateBlock: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9) -> (),
    EndStateBlock: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, ppSB: *mut *mut IDirect3DStateBlock9) -> (),
    SetClipStatus: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pClipStatus: *const D3DCLIPSTATUS9) -> (),
    GetClipStatus: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pClipStatus: *mut D3DCLIPSTATUS9) -> (),
    GetTexture: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Stage: u32, ppTexture: *mut *mut IDirect3DBaseTexture9) -> (),
    SetTexture: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Stage: u32, pTexture: *mut IDirect3DBaseTexture9) -> (),
    GetTextureStageState: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Stage: u32, Type: D3DTEXTURESTAGESTATETYPE, pValue: *mut u32) -> (),
    SetTextureStageState: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Stage: u32, Type: D3DTEXTURESTAGESTATETYPE, Value: u32) -> (),
    GetSamplerState: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Sampler: u32, Type: D3DSAMPLERSTATETYPE, pValue: *mut u32) -> (),
    SetSamplerState: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Sampler: u32, Type: D3DSAMPLERSTATETYPE, Value: u32) -> (),
    ValidateDevice: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pNumPasses: *mut u32) -> (),
    SetPaletteEntries: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, PaletteNumber: c_uint, pEntries: *const PALETTEENTRY) -> (),
    GetPaletteEntries: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, PaletteNumber: c_uint, pEntries: *mut PALETTEENTRY) -> (),
    SetCurrentTexturePalette: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, PaletteNumber: c_uint) -> (),
    GetCurrentTexturePalette: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, PaletteNumber: *mut c_uint) -> (),
    SetScissorRect: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pRect: *const RECT) -> (),
    GetScissorRect: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pRect: *mut RECT) -> (),
    SetSoftwareVertexProcessing: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, bSoftware: BOOL) -> (),
    GetSoftwareVertexProcessing: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9) -> BOOL,
    SetNPatchMode: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, nSegments: f32) -> (),
    GetNPatchMode: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9) -> f32,
    DrawPrimitive: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, PrimitiveType: D3DPRIMITIVETYPE, StartVertex: c_uint, PrimitiveCount: c_uint) -> (),
    DrawIndexedPrimitive: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, PrimitiveType: D3DPRIMITIVETYPE, BaseVertexIndex: c_int, MinVertexIndex: c_uint, NumVertices: c_uint, startIndex: c_uint, primCount: c_uint) -> (),
    DrawPrimitiveUP: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, PrimitiveType: D3DPRIMITIVETYPE, PrimitiveCount: c_uint, pVertexStreamZeroData: *const c_void, VertexStreamZeroStride: c_uint) -> (),
    DrawIndexedPrimitiveUP: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, PrimitiveType: D3DPRIMITIVETYPE, MinVertexIndex: c_uint, NumVertices: c_uint, PrimitiveCount: c_uint, pIndexData: *const c_void, IndexDataFormat: D3DFORMAT, pVertexStreamZeroData: *const c_void, VertexStreamZeroStride: c_uint) -> (),
    ProcessVertices: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, SrcStartIndex: c_uint, DestIndex: c_uint, VertexCount: c_uint, pDestBuffer: *mut IDirect3DVertexBuffer9, pVertexDecl: *mut IDirect3DVertexDeclaration9, Flags: u32) -> (),
    CreateVertexDeclaration: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pVertexElements: *const D3DVERTEXELEMENT9, ppDecl: *mut *mut IDirect3DVertexDeclaration9) -> (),
    SetVertexDeclaration: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pDecl: *mut IDirect3DVertexDeclaration9) -> (),
    GetVertexDeclaration: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, ppDecl: *mut *mut IDirect3DVertexDeclaration9) -> (),
    SetFVF: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, FVF: u32) -> (),
    GetFVF: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pFVF: *mut u32) -> (),
    CreateVertexShader: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pFunction: *const u32, ppShader: *mut *mut IDirect3DVertexShader9) -> (),
    SetVertexShader: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pShader: *mut IDirect3DVertexShader9) -> (),
    GetVertexShader: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, ppShader: *mut *mut IDirect3DVertexShader9) -> (),
    SetVertexShaderConstantF: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StartRegister: c_uint, pConstantData: *const f32, Vector4fCount: c_uint) -> (),
    GetVertexShaderConstantF: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StartRegister: c_uint, pConstantData: *mut f32, Vector4fCount: c_uint) -> (),
    SetVertexShaderConstantI: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StartRegister: c_uint, pConstantData: *const c_int, Vector4iCount: c_uint) -> (),
    GetVertexShaderConstantI: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StartRegister: c_uint, pConstantData: *mut c_int, Vector4iCount: c_uint) -> (),
    SetVertexShaderConstantB: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StartRegister: c_uint, pConstantData: *const BOOL, BoolCount: c_uint) -> (),
    GetVertexShaderConstantB: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StartRegister: c_uint, pConstantData: *mut BOOL, BoolCount: c_uint) -> (),
    SetStreamSource: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StreamNumber: c_uint, pStreamData: *mut IDirect3DVertexBuffer9, OffsetInBytes: c_uint, Stride: c_uint) -> (),
    GetStreamSource: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StreamNumber: c_uint, ppStreamData: *mut *mut IDirect3DVertexBuffer9, pOffsetInBytes: *mut c_uint, pStride: *mut c_uint) -> (),
    SetStreamSourceFreq: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StreamNumber: c_uint, Setting: c_uint) -> (),
    GetStreamSourceFreq: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StreamNumber: c_uint, pSetting: *mut c_uint) -> (),
    SetIndices: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pIndexData: *mut IDirect3DIndexBuffer9) -> (),
    GetIndices: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, ppIndexData: *mut *mut IDirect3DIndexBuffer9) -> (),
    CreatePixelShader: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pFunction: *const u32, ppShader: *mut *mut IDirect3DPixelShader9) -> (),
    SetPixelShader: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, pShader: *mut IDirect3DPixelShader9) -> (),
    GetPixelShader: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, ppShader: *mut *mut IDirect3DPixelShader9) -> (),
    SetPixelShaderConstantF: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StartRegister: c_uint, pConstantData: *const f32, Vector4fCount: c_uint) -> (),
    GetPixelShaderConstantF: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StartRegister: c_uint, pConstantData: *mut f32, Vector4fCount: c_uint) -> (),
    SetPixelShaderConstantI: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StartRegister: c_uint, pConstantData: *const c_int, Vector4iCount: c_uint) -> (),
    GetPixelShaderConstantI: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StartRegister: c_uint, pConstantData: *mut c_int, Vector4iCount: c_uint) -> (),
    SetPixelShaderConstantB: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StartRegister: c_uint, pConstantData: *const BOOL, BoolCount: c_uint) -> (),
    GetPixelShaderConstantB: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, StartRegister: c_uint, pConstantData: *mut BOOL, BoolCount: c_uint) -> (),
    DrawRectPatch: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Handle: c_uint, pNumSegs: *const f32, pRectPatchInfo: *mut D3DRECTPATCH_INFO) -> (),
    DrawTriPatch: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Handle: c_uint, pNumSegs: *const f32, pTriPatchInfo: *mut D3DTRIPATCH_INFO) -> (),
    DeletePatch: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Handle: c_uint) -> (),
    CreateQuery: unsafe extern "stdcall" fn (device: *mut IDirect3DDevice9, Type: D3DQUERYTYPE, ppQuery: *mut *mut IDirect3DQuery9) -> (),
}
