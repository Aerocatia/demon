use c_mine::c_mine;
use crate::rasterizer::d3d9::{d3d_device, IDirect3DBaseTexture9, IDirect3DPixelShader9, IDirect3DVertexShader9, D3DRENDERSTATETYPE, D3DSAMPLERSTATETYPE, D3DTEXTURESTAGESTATETYPE};

#[c_mine]
pub unsafe extern "C" fn D3D9DeviceEx_SetRenderState(State: D3DRENDERSTATETYPE, Value: u32) {
    d3d_device().SetRenderState(State, Value);
}

#[c_mine]
pub unsafe extern "C" fn D3D9DeviceEx_SetTexture(Stage: u32, pTexture: *mut IDirect3DBaseTexture9) {
    d3d_device().SetTexture(Stage, pTexture);
}

#[c_mine]
pub unsafe extern "C" fn D3D9DeviceEx_SetTextureStageState(Stage: u32, Type: D3DTEXTURESTAGESTATETYPE, Value: u32) {
    d3d_device().SetTextureStageState(Stage, Type, Value);
}

#[c_mine]
pub unsafe extern "C" fn D3D9DeviceEx_SetPixelShader(pShader: *mut IDirect3DPixelShader9) {
    d3d_device().SetPixelShader(pShader);
}

#[c_mine]
pub unsafe extern "C" fn D3D9DeviceEx_SetSamplerState(Sampler: u32, Type: D3DSAMPLERSTATETYPE, Value: u32) {
    d3d_device().SetSamplerState(Sampler, Type, Value);
}

#[c_mine]
pub unsafe extern "C" fn D3D9DeviceEx_SetVertexShader(pShader: *mut IDirect3DVertexShader9) {
    d3d_device().SetVertexShader(pShader);
}
