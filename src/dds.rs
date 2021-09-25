// Copyright (c) 2021 Kyrylo Bazhenov
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[doc = "https://docs.microsoft.com/en-us/windows/win32/direct3ddds/dds-pixelformat"]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DirectDrawPixelFormat {
    pub size: u32,
    pub flags: u32,
    pub four_cc: [u8; 4],
    pub rgb_bit_count: u32,
    pub red_bit_mask: u32,
    pub green_bit_mask: u32,
    pub blue_bit_mask: u32,
    pub alpha_bit_mask: u32,
}

#[doc = "https://docs.microsoft.com/en-us/windows/win32/direct3ddds/dds-header-dxt10"]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DirectDrawHeader10 {
    pub dxgi_format: u32,
    pub resource_dimension: u32,
    pub misc_flag: u32,
    pub array_size: u32,
    pub misc_flags2: u32,
}

#[doc = "https://docs.microsoft.com/en-us/windows/win32/direct3ddds/dds-header"]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DirectDrawHeader {
    pub magic: [u8; 4],
    pub size: u32,
    pub flags: u32,
    pub height: u32,
    pub width: u32,
    pub pitch_or_linear_size: u32,
    pub depth: u32,
    pub mipmap_count: u32,
    pub reserved: [u32; 11],
    pub pixel_format: DirectDrawPixelFormat,
    pub caps: u32,
    pub caps2: u32,
    pub caps3: u32,
    pub caps4: u32,
    pub reserved2: u32,
    pub dxt10: DirectDrawHeader10,
}

unsafe impl bytemuck::Zeroable for DirectDrawHeader {}
unsafe impl bytemuck::Pod for DirectDrawHeader {}

// flags
#[allow(unused, clippy::unreadable_literal)]
mod header_flags {
    pub const DDSD_CAPS: u32 = 0x1;
    pub const DDSD_HEIGHT: u32 = 0x2;
    pub const DDSD_WIDTH: u32 = 0x4;
    pub const DDSD_PITCH: u32 = 0x8;
    pub const DDSD_PIXELFORMAT: u32 = 0x1000;
    pub const DDSD_MIPMAPCOUNT: u32 = 0x20000;
    pub const DDSD_LINEARSIZE: u32 = 0x80000;
    pub const DDSD_DEPTH: u32 = 0x800000;
}

// pixel_format flags
#[allow(unused)]
mod pixel_format_flags {
    pub const DDPF_ALPHAPIXELS: u32 = 0x1;
    pub const DDPF_ALPHA: u32 = 0x2;
    pub const DDPF_FOURCC: u32 = 0x4;
    pub const DDPF_RGB: u32 = 0x40;
    pub const DDPF_YUV: u32 = 0x200;
    pub const DDPF_LUMINANCE: u32 = 0x20000;
}

// caps flags
#[allow(unused, clippy::unreadable_literal)]
mod caps {
    pub const DDSCAPS_COMPLEX: u32 = 0x8;
    pub const DDSCAPS_MIPMAP: u32 = 0x400000;
    pub const DDSCAPS_TEXTURE: u32 = 0x1000;
}

// caps2 flags
#[allow(unused, clippy::unreadable_literal)]
mod caps2 {
    pub const DDSCAPS2_CUBEMAP: u32 = 0x200;
    pub const DDSCAPS2_CUBEMAP_POSITIVEX: u32 = 0x400;
    pub const DDSCAPS2_CUBEMAP_NEGATIVEX: u32 = 0x800;
    pub const DDSCAPS2_CUBEMAP_POSITIVEY: u32 = 0x1000;
    pub const DDSCAPS2_CUBEMAP_NEGATIVEY: u32 = 0x2000;
    pub const DDSCAPS2_CUBEMAP_POSITIVEZ: u32 = 0x4000;
    pub const DDSCAPS2_CUBEMAP_NEGATIVEZ: u32 = 0x8000;
    pub const DDSCAPS2_VOLUME: u32 = 0x200000;
}

// dxgi_format
pub mod dxgi_format {
    pub const DXGI_FORMAT_UNKNOWN: u32 = 0;
    pub const DXGI_FORMAT_R32G32B32A32_TYPELESS: u32 = 1;
    pub const DXGI_FORMAT_R32G32B32A32_FLOAT: u32 = 2;
    pub const DXGI_FORMAT_R32G32B32A32_UINT: u32 = 3;
    pub const DXGI_FORMAT_R32G32B32A32_SINT: u32 = 4;
    pub const DXGI_FORMAT_R32G32B32_TYPELESS: u32 = 5;
    pub const DXGI_FORMAT_R32G32B32_FLOAT: u32 = 6;
    pub const DXGI_FORMAT_R32G32B32_UINT: u32 = 7;
    pub const DXGI_FORMAT_R32G32B32_SINT: u32 = 8;
    pub const DXGI_FORMAT_R16G16B16A16_TYPELESS: u32 = 9;
    pub const DXGI_FORMAT_R16G16B16A16_FLOAT: u32 = 10;
    pub const DXGI_FORMAT_R16G16B16A16_UNORM: u32 = 11;
    pub const DXGI_FORMAT_R16G16B16A16_UINT: u32 = 12;
    pub const DXGI_FORMAT_R16G16B16A16_SNORM: u32 = 13;
    pub const DXGI_FORMAT_R16G16B16A16_SINT: u32 = 14;
    pub const DXGI_FORMAT_R32G32_TYPELESS: u32 = 15;
    pub const DXGI_FORMAT_R32G32_FLOAT: u32 = 16;
    pub const DXGI_FORMAT_R32G32_UINT: u32 = 17;
    pub const DXGI_FORMAT_R32G32_SINT: u32 = 18;
    pub const DXGI_FORMAT_R32G8X24_TYPELESS: u32 = 19;
    pub const DXGI_FORMAT_D32_FLOAT_S8X24_UINT: u32 = 20;
    pub const DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS: u32 = 21;
    pub const DXGI_FORMAT_X32_TYPELESS_G8X24_UINT: u32 = 22;
    pub const DXGI_FORMAT_R10G10B10A2_TYPELESS: u32 = 23;
    pub const DXGI_FORMAT_R10G10B10A2_UNORM: u32 = 24;
    pub const DXGI_FORMAT_R10G10B10A2_UINT: u32 = 25;
    pub const DXGI_FORMAT_R11G11B10_FLOAT: u32 = 26;
    pub const DXGI_FORMAT_R8G8B8A8_TYPELESS: u32 = 27;
    pub const DXGI_FORMAT_R8G8B8A8_UNORM: u32 = 28;
    pub const DXGI_FORMAT_R8G8B8A8_UNORM_SRGB: u32 = 29;
    pub const DXGI_FORMAT_R8G8B8A8_UINT: u32 = 30;
    pub const DXGI_FORMAT_R8G8B8A8_SNORM: u32 = 31;
    pub const DXGI_FORMAT_R8G8B8A8_SINT: u32 = 32;
    pub const DXGI_FORMAT_R16G16_TYPELESS: u32 = 33;
    pub const DXGI_FORMAT_R16G16_FLOAT: u32 = 34;
    pub const DXGI_FORMAT_R16G16_UNORM: u32 = 35;
    pub const DXGI_FORMAT_R16G16_UINT: u32 = 36;
    pub const DXGI_FORMAT_R16G16_SNORM: u32 = 37;
    pub const DXGI_FORMAT_R16G16_SINT: u32 = 38;
    pub const DXGI_FORMAT_R32_TYPELESS: u32 = 39;
    pub const DXGI_FORMAT_D32_FLOAT: u32 = 40;
    pub const DXGI_FORMAT_R32_FLOAT: u32 = 41;
    pub const DXGI_FORMAT_R32_UINT: u32 = 42;
    pub const DXGI_FORMAT_R32_SINT: u32 = 43;
    pub const DXGI_FORMAT_R24G8_TYPELESS: u32 = 44;
    pub const DXGI_FORMAT_D24_UNORM_S8_UINT: u32 = 45;
    pub const DXGI_FORMAT_R24_UNORM_X8_TYPELESS: u32 = 46;
    pub const DXGI_FORMAT_X24_TYPELESS_G8_UINT: u32 = 47;
    pub const DXGI_FORMAT_R8G8_TYPELESS: u32 = 48;
    pub const DXGI_FORMAT_R8G8_UNORM: u32 = 49;
    pub const DXGI_FORMAT_R8G8_UINT: u32 = 50;
    pub const DXGI_FORMAT_R8G8_SNORM: u32 = 51;
    pub const DXGI_FORMAT_R8G8_SINT: u32 = 52;
    pub const DXGI_FORMAT_R16_TYPELESS: u32 = 53;
    pub const DXGI_FORMAT_R16_FLOAT: u32 = 54;
    pub const DXGI_FORMAT_D16_UNORM: u32 = 55;
    pub const DXGI_FORMAT_R16_UNORM: u32 = 56;
    pub const DXGI_FORMAT_R16_UINT: u32 = 57;
    pub const DXGI_FORMAT_R16_SNORM: u32 = 58;
    pub const DXGI_FORMAT_R16_SINT: u32 = 59;
    pub const DXGI_FORMAT_R8_TYPELESS: u32 = 60;
    pub const DXGI_FORMAT_R8_UNORM: u32 = 61;
    pub const DXGI_FORMAT_R8_UINT: u32 = 62;
    pub const DXGI_FORMAT_R8_SNORM: u32 = 63;
    pub const DXGI_FORMAT_R8_SINT: u32 = 64;
    pub const DXGI_FORMAT_A8_UNORM: u32 = 65;
    pub const DXGI_FORMAT_R1_UNORM: u32 = 66;
    pub const DXGI_FORMAT_R9G9B9E5_SHAREDEXP: u32 = 67;
    pub const DXGI_FORMAT_R8G8_B8G8_UNORM: u32 = 68;
    pub const DXGI_FORMAT_G8R8_G8B8_UNORM: u32 = 69;
    pub const DXGI_FORMAT_BC1_TYPELESS: u32 = 70;
    pub const DXGI_FORMAT_BC1_UNORM: u32 = 71;
    pub const DXGI_FORMAT_BC1_UNORM_SRGB: u32 = 72;
    pub const DXGI_FORMAT_BC2_TYPELESS: u32 = 73;
    pub const DXGI_FORMAT_BC2_UNORM: u32 = 74;
    pub const DXGI_FORMAT_BC2_UNORM_SRGB: u32 = 75;
    pub const DXGI_FORMAT_BC3_TYPELESS: u32 = 76;
    pub const DXGI_FORMAT_BC3_UNORM: u32 = 77;
    pub const DXGI_FORMAT_BC3_UNORM_SRGB: u32 = 78;
    pub const DXGI_FORMAT_BC4_TYPELESS: u32 = 79;
    pub const DXGI_FORMAT_BC4_UNORM: u32 = 80;
    pub const DXGI_FORMAT_BC4_SNORM: u32 = 81;
    pub const DXGI_FORMAT_BC5_TYPELESS: u32 = 82;
    pub const DXGI_FORMAT_BC5_UNORM: u32 = 83;
    pub const DXGI_FORMAT_BC5_SNORM: u32 = 84;
    pub const DXGI_FORMAT_B5G6R5_UNORM: u32 = 85;
    pub const DXGI_FORMAT_B5G5R5A1_UNORM: u32 = 86;
    pub const DXGI_FORMAT_B8G8R8A8_UNORM: u32 = 87;
    pub const DXGI_FORMAT_B8G8R8X8_UNORM: u32 = 88;
    pub const DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM: u32 = 89;
    pub const DXGI_FORMAT_B8G8R8A8_TYPELESS: u32 = 90;
    pub const DXGI_FORMAT_B8G8R8A8_UNORM_SRGB: u32 = 91;
    pub const DXGI_FORMAT_B8G8R8X8_TYPELESS: u32 = 92;
    pub const DXGI_FORMAT_B8G8R8X8_UNORM_SRGB: u32 = 93;
    pub const DXGI_FORMAT_BC6H_TYPELESS: u32 = 94;
    pub const DXGI_FORMAT_BC6H_UF16: u32 = 95;
    pub const DXGI_FORMAT_BC6H_SF16: u32 = 96;
    pub const DXGI_FORMAT_BC7_TYPELESS: u32 = 97;
    pub const DXGI_FORMAT_BC7_UNORM: u32 = 98;
    pub const DXGI_FORMAT_BC7_UNORM_SRGB: u32 = 99;
    pub const DXGI_FORMAT_AYUV: u32 = 100;
    pub const DXGI_FORMAT_Y410: u32 = 101;
    pub const DXGI_FORMAT_Y416: u32 = 102;
    pub const DXGI_FORMAT_NV12: u32 = 103;
    pub const DXGI_FORMAT_P010: u32 = 104;
    pub const DXGI_FORMAT_P016: u32 = 105;
    pub const DXGI_FORMAT_420_OPAQUE: u32 = 106;
    pub const DXGI_FORMAT_YUY2: u32 = 107;
    pub const DXGI_FORMAT_Y210: u32 = 108;
    pub const DXGI_FORMAT_Y216: u32 = 109;
    pub const DXGI_FORMAT_NV11: u32 = 110;
    pub const DXGI_FORMAT_AI44: u32 = 111;
    pub const DXGI_FORMAT_IA44: u32 = 112;
    pub const DXGI_FORMAT_P8: u32 = 113;
    pub const DXGI_FORMAT_A8P8: u32 = 114;
    pub const DXGI_FORMAT_B4G4R4A4_UNORM: u32 = 115;
    pub const DXGI_FORMAT_P208: u32 = 116;
    pub const DXGI_FORMAT_V208: u32 = 117;
    pub const DXGI_FORMAT_V408: u32 = 118;
}

// resource_dimension
#[allow(unused)]
mod resource_dimension {
    pub const D3D10_RESOURCE_DIMENSION_UNKNOWN: u32 = 0;
    pub const D3D10_RESOURCE_DIMENSION_BUFFER: u32 = 1;
    pub const D3D10_RESOURCE_DIMENSION_TEXTURE1D: u32 = 2;
    pub const D3D10_RESOURCE_DIMENSION_TEXTURE2D: u32 = 3;
    pub const D3D10_RESOURCE_DIMENSION_TEXTURE3D: u32 = 4;
}

// misc_flag
#[allow(unused)]
mod misc_flag {
    pub const DDS_RESOURCE_MISC_TEXTURECUBE: u32 = 0x4;
}

// misc_flags2
#[allow(unused)]
mod misc_flags2 {
    pub const DDS_ALPHA_MODE_UNKNOWN: u32 = 0x0;
    pub const DDS_ALPHA_MODE_STRAIGHT: u32 = 0x1;
    pub const DDS_ALPHA_MODE_PREMULTIPLIED: u32 = 0x2;
    pub const DDS_ALPHA_MODE_OPAQUE: u32 = 0x3;
    pub const DDS_ALPHA_MODE_CUSTOM: u32 = 0x4;
}

pub use caps::*;
pub use caps2::*;
pub use dxgi_format::*;
pub use header_flags::*;
pub use misc_flag::*;
pub use misc_flags2::*;
pub use pixel_format_flags::*;
pub use resource_dimension::*;

pub fn is_block_compressed(dxgi_format: u32) -> bool {
    dxgi_format == DXGI_FORMAT_BC1_TYPELESS
        || dxgi_format == DXGI_FORMAT_BC1_UNORM
        || dxgi_format == DXGI_FORMAT_BC1_UNORM_SRGB
        || dxgi_format == DXGI_FORMAT_BC2_TYPELESS
        || dxgi_format == DXGI_FORMAT_BC2_UNORM
        || dxgi_format == DXGI_FORMAT_BC2_UNORM_SRGB
        || dxgi_format == DXGI_FORMAT_BC3_TYPELESS
        || dxgi_format == DXGI_FORMAT_BC3_UNORM
        || dxgi_format == DXGI_FORMAT_BC3_UNORM_SRGB
        || dxgi_format == DXGI_FORMAT_BC4_TYPELESS
        || dxgi_format == DXGI_FORMAT_BC4_UNORM
        || dxgi_format == DXGI_FORMAT_BC4_SNORM
        || dxgi_format == DXGI_FORMAT_BC5_TYPELESS
        || dxgi_format == DXGI_FORMAT_BC5_UNORM
        || dxgi_format == DXGI_FORMAT_BC5_SNORM
        || dxgi_format == DXGI_FORMAT_BC6H_TYPELESS
        || dxgi_format == DXGI_FORMAT_BC6H_UF16
        || dxgi_format == DXGI_FORMAT_BC6H_SF16
        || dxgi_format == DXGI_FORMAT_BC7_TYPELESS
        || dxgi_format == DXGI_FORMAT_BC7_UNORM
        || dxgi_format == DXGI_FORMAT_BC7_UNORM_SRGB
}

pub fn is_legacy_yuv(dxgi_format: u32) -> bool {
    dxgi_format == DXGI_FORMAT_AYUV
        || dxgi_format == DXGI_FORMAT_Y410
        || dxgi_format == DXGI_FORMAT_Y416
        || dxgi_format == DXGI_FORMAT_NV12
        || dxgi_format == DXGI_FORMAT_P010
        || dxgi_format == DXGI_FORMAT_P016
        || dxgi_format == DXGI_FORMAT_420_OPAQUE
        || dxgi_format == DXGI_FORMAT_YUY2
        || dxgi_format == DXGI_FORMAT_Y210
        || dxgi_format == DXGI_FORMAT_Y216
        || dxgi_format == DXGI_FORMAT_NV11
        || dxgi_format == DXGI_FORMAT_AI44
        || dxgi_format == DXGI_FORMAT_IA44
        || dxgi_format == DXGI_FORMAT_P8
        || dxgi_format == DXGI_FORMAT_A8P8
        || dxgi_format == DXGI_FORMAT_B4G4R4A4_UNORM
        || dxgi_format == DXGI_FORMAT_P208
        || dxgi_format == DXGI_FORMAT_V208
        || dxgi_format == DXGI_FORMAT_V408
}

pub fn bits_per_pixel(dxgi_format: u32) -> u32 {
    match dxgi_format {
        DXGI_FORMAT_R32G32B32A32_TYPELESS => 128,
        DXGI_FORMAT_R32G32B32A32_FLOAT => 128,
        DXGI_FORMAT_R32G32B32A32_UINT => 128,
        DXGI_FORMAT_R32G32B32A32_SINT => 128,

        DXGI_FORMAT_R32G32B32_TYPELESS => 96,
        DXGI_FORMAT_R32G32B32_FLOAT => 96,
        DXGI_FORMAT_R32G32B32_UINT => 96,
        DXGI_FORMAT_R32G32B32_SINT => 96,

        DXGI_FORMAT_R16G16B16A16_TYPELESS => 64,
        DXGI_FORMAT_R16G16B16A16_FLOAT => 64,
        DXGI_FORMAT_R16G16B16A16_UNORM => 64,
        DXGI_FORMAT_R16G16B16A16_UINT => 64,
        DXGI_FORMAT_R16G16B16A16_SNORM => 64,
        DXGI_FORMAT_R16G16B16A16_SINT => 64,
        DXGI_FORMAT_R32G32_TYPELESS => 64,
        DXGI_FORMAT_R32G32_FLOAT => 64,
        DXGI_FORMAT_R32G32_UINT => 64,
        DXGI_FORMAT_R32G32_SINT => 64,
        DXGI_FORMAT_R32G8X24_TYPELESS => 64,
        DXGI_FORMAT_D32_FLOAT_S8X24_UINT => 64,
        DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS => 64,
        DXGI_FORMAT_X32_TYPELESS_G8X24_UINT => 64,
        DXGI_FORMAT_Y416 => 64,
        DXGI_FORMAT_Y210 => 64,
        DXGI_FORMAT_Y216 => 64,

        DXGI_FORMAT_R10G10B10A2_TYPELESS => 32,
        DXGI_FORMAT_R10G10B10A2_UNORM => 32,
        DXGI_FORMAT_R10G10B10A2_UINT => 32,
        DXGI_FORMAT_R11G11B10_FLOAT => 32,
        DXGI_FORMAT_R8G8B8A8_TYPELESS => 32,
        DXGI_FORMAT_R8G8B8A8_UNORM => 32,
        DXGI_FORMAT_R8G8B8A8_UNORM_SRGB => 32,
        DXGI_FORMAT_R8G8B8A8_UINT => 32,
        DXGI_FORMAT_R8G8B8A8_SNORM => 32,
        DXGI_FORMAT_R8G8B8A8_SINT => 32,
        DXGI_FORMAT_R16G16_TYPELESS => 32,
        DXGI_FORMAT_R16G16_FLOAT => 32,
        DXGI_FORMAT_R16G16_UNORM => 32,
        DXGI_FORMAT_R16G16_UINT => 32,
        DXGI_FORMAT_R16G16_SNORM => 32,
        DXGI_FORMAT_R16G16_SINT => 32,
        DXGI_FORMAT_R32_TYPELESS => 32,
        DXGI_FORMAT_D32_FLOAT => 32,
        DXGI_FORMAT_R32_FLOAT => 32,
        DXGI_FORMAT_R32_UINT => 32,
        DXGI_FORMAT_R32_SINT => 32,
        DXGI_FORMAT_R24G8_TYPELESS => 32,
        DXGI_FORMAT_D24_UNORM_S8_UINT => 32,
        DXGI_FORMAT_R24_UNORM_X8_TYPELESS => 32,
        DXGI_FORMAT_X24_TYPELESS_G8_UINT => 32,
        DXGI_FORMAT_R9G9B9E5_SHAREDEXP => 32,
        DXGI_FORMAT_R8G8_B8G8_UNORM => 32,
        DXGI_FORMAT_G8R8_G8B8_UNORM => 32,
        DXGI_FORMAT_B8G8R8A8_UNORM => 32,
        DXGI_FORMAT_B8G8R8X8_UNORM => 32,
        DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM => 32,
        DXGI_FORMAT_B8G8R8A8_TYPELESS => 32,
        DXGI_FORMAT_B8G8R8A8_UNORM_SRGB => 32,
        DXGI_FORMAT_B8G8R8X8_TYPELESS => 32,
        DXGI_FORMAT_B8G8R8X8_UNORM_SRGB => 32,
        DXGI_FORMAT_AYUV => 32,
        DXGI_FORMAT_Y410 => 32,
        DXGI_FORMAT_YUY2 => 32,
        DXGI_FORMAT_P010 => 24,
        DXGI_FORMAT_P016 => 24,
        DXGI_FORMAT_R8G8_TYPELESS => 16,
        DXGI_FORMAT_R8G8_UNORM => 16,
        DXGI_FORMAT_R8G8_UINT => 16,
        DXGI_FORMAT_R8G8_SNORM => 16,
        DXGI_FORMAT_R8G8_SINT => 16,
        DXGI_FORMAT_R16_TYPELESS => 16,
        DXGI_FORMAT_R16_FLOAT => 16,
        DXGI_FORMAT_D16_UNORM => 16,
        DXGI_FORMAT_R16_UNORM => 16,
        DXGI_FORMAT_R16_UINT => 16,
        DXGI_FORMAT_R16_SNORM => 16,
        DXGI_FORMAT_R16_SINT => 16,
        DXGI_FORMAT_B5G6R5_UNORM => 16,
        DXGI_FORMAT_B5G5R5A1_UNORM => 16,
        DXGI_FORMAT_A8P8 => 16,
        DXGI_FORMAT_B4G4R4A4_UNORM => 16,
        DXGI_FORMAT_NV12 => 12,
        DXGI_FORMAT_420_OPAQUE => 12,
        DXGI_FORMAT_NV11 => 12,

        DXGI_FORMAT_R8_TYPELESS => 8,
        DXGI_FORMAT_R8_UNORM => 8,
        DXGI_FORMAT_R8_UINT => 8,
        DXGI_FORMAT_R8_SNORM => 8,
        DXGI_FORMAT_R8_SINT => 8,
        DXGI_FORMAT_A8_UNORM => 8,
        DXGI_FORMAT_AI44 => 8,
        DXGI_FORMAT_IA44 => 8,
        DXGI_FORMAT_P8 => 8,
        DXGI_FORMAT_R1_UNORM => 1,

        DXGI_FORMAT_BC1_TYPELESS => 4,
        DXGI_FORMAT_BC1_UNORM => 4,
        DXGI_FORMAT_BC1_UNORM_SRGB => 4,
        DXGI_FORMAT_BC4_TYPELESS => 4,
        DXGI_FORMAT_BC4_UNORM => 4,
        DXGI_FORMAT_BC4_SNORM => 4,

        DXGI_FORMAT_BC2_TYPELESS => 8,
        DXGI_FORMAT_BC2_UNORM => 8,
        DXGI_FORMAT_BC2_UNORM_SRGB => 8,
        DXGI_FORMAT_BC3_TYPELESS => 8,
        DXGI_FORMAT_BC3_UNORM => 8,
        DXGI_FORMAT_BC3_UNORM_SRGB => 8,
        DXGI_FORMAT_BC5_TYPELESS => 8,
        DXGI_FORMAT_BC5_UNORM => 8,
        DXGI_FORMAT_BC5_SNORM => 8,
        DXGI_FORMAT_BC6H_TYPELESS => 8,
        DXGI_FORMAT_BC6H_UF16 => 8,
        DXGI_FORMAT_BC6H_SF16 => 8,
        DXGI_FORMAT_BC7_TYPELESS => 8,
        DXGI_FORMAT_BC7_UNORM => 8,
        DXGI_FORMAT_BC7_UNORM_SRGB => 8,

        _ => unimplemented!("Format not implemented: {}", dxgi_format),
    }
}

#[doc = "https://docs.microsoft.com/en-us/windows/win32/direct3ddds/dx-graphics-dds-pguide"]
pub fn block_size(dxgi_format: u32) -> u32 {
    if dxgi_format == DXGI_FORMAT_BC1_TYPELESS
        || dxgi_format == DXGI_FORMAT_BC1_UNORM
        || dxgi_format == DXGI_FORMAT_BC1_UNORM_SRGB
    {
        return 8;
    }

    if dxgi_format == DXGI_FORMAT_BC4_TYPELESS
        || dxgi_format == DXGI_FORMAT_BC4_UNORM
        || dxgi_format == DXGI_FORMAT_BC4_SNORM
    {
        return 8;
    }

    16
}

#[doc = "https://docs.microsoft.com/en-us/windows/win32/direct3ddds/dx-graphics-dds-pguide"]
pub fn pitch_and_linear_size(width: u32, height: u32, dxgi_format: u32) -> (u32, u32) {
    if is_block_compressed(dxgi_format) {
        let row_pitch = 1.max((width + 3) / 4) * block_size(dxgi_format);
        let linear_size = row_pitch * 1.max((height + 3) / 4);

        return (row_pitch, linear_size);
    }

    if is_legacy_yuv(dxgi_format)
        || dxgi_format == DXGI_FORMAT_R8G8_B8G8_UNORM
        || dxgi_format == DXGI_FORMAT_G8R8_G8B8_UNORM
    {
        let row_pitch = ((width + 1) >> 1) * 4;
        let linear_size = row_pitch * height; // TODO: is this correct?

        return (row_pitch, linear_size);
    }

    let row_pitch = (width * bits_per_pixel(dxgi_format) + 7) / 8;
    let linear_size = row_pitch * height;

    (row_pitch, linear_size)
}
