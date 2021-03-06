use crate::prelude::*;
use crate::{Color, Data, Point, Rect};
use skia_bindings as sb;
use skia_bindings::{SkColor, SkPoint, SkVertices, SkVertices_Attribute, SkVertices_Builder};
use std::{ptr, slice};

#[deprecated(since = "0.29.0", note = "removed without replacement")]
pub type BoneIndices = [u32; 4];

#[deprecated(since = "0.29.0", note = "removed without replacement")]
pub type BoneWeights = [u32; 4];

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
#[deprecated(since = "0.29.0", note = "removed without replacement")]
pub struct Bone {
    values: [f32; 6],
}

pub use skia_bindings::SkVertices_VertexMode as VertexMode;
#[test]
fn test_vertices_vertex_mode_naming() {
    let _ = VertexMode::Triangles;
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AttributeType {
    Float = sb::SkVertices_Attribute_Type::Float as _,
    Float2 = sb::SkVertices_Attribute_Type::Float2 as _,
    Float3 = sb::SkVertices_Attribute_Type::Float3 as _,
    Float4 = sb::SkVertices_Attribute_Type::Float4 as _,
    Byte4UNorm = sb::SkVertices_Attribute_Type::Byte4_unorm as _,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Attribute {
    pub tp: AttributeType,
}

impl NativeTransmutable<SkVertices_Attribute> for Attribute {}

#[test]
fn test_attribute_layout() {
    Attribute::test_layout()
}

impl Default for Attribute {
    fn default() -> Self {
        Attribute::new(AttributeType::Float)
    }
}

impl Attribute {
    pub fn new(tp: AttributeType) -> Self {
        Self { tp }
    }

    pub fn channel_count(self) -> usize {
        unsafe { self.native().channelCount() }.try_into().unwrap()
    }

    pub fn bytes_per_vertex(self) -> usize {
        unsafe { self.native().bytesPerVertex() }
    }
}

pub type Vertices = RCHandle<SkVertices>;

impl NativeRefCounted for SkVertices {
    fn _ref(&self) {
        unsafe { sb::C_SkVertices_ref(self) }
    }

    fn _unref(&self) {
        unsafe { sb::C_SkVertices_unref(self) }
    }

    fn unique(&self) -> bool {
        unsafe { sb::C_SkVertices_unique(self) }
    }
}

impl RCHandle<SkVertices> {
    pub fn new_copy(
        mode: VertexMode,
        positions: &[Point],
        texs: &[Point],
        colors: &[Color],
        indices: Option<&[u16]>,
    ) -> Vertices {
        let vertex_count = positions.len();
        assert_eq!(texs.len(), vertex_count);
        assert_eq!(colors.len(), vertex_count);

        let indices_ptr = indices.map(|i| i.as_ptr()).unwrap_or(ptr::null());
        let indices_count = indices.map(|i| i.len()).unwrap_or(0);

        Vertices::from_ptr(unsafe {
            sb::C_SkVertices_MakeCopy(
                mode,
                vertex_count as _,
                positions.native().as_ptr(),
                texs.native().as_ptr(),
                colors.native().as_ptr(),
                indices_count.try_into().unwrap(),
                indices_ptr,
            )
        })
        .unwrap()
    }

    pub const MAX_CUSTOM_ATTRIBUTES: usize = 8;

    pub fn unique_id(&self) -> u32 {
        self.native().fUniqueID
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    pub fn mode(&self) -> VertexMode {
        self.native().fMode
    }

    pub fn bounds(&self) -> &Rect {
        Rect::from_native_ref(&self.native().fBounds)
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn has_colors(&self) -> bool {
        self.colors().is_some()
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn has_tex_coords(&self) -> bool {
        self.tex_coords().is_some()
    }

    #[deprecated(since = "0.29.0", note = "returns false")]
    pub fn has_bones(&self) -> bool {
        false
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn has_indices(&self) -> bool {
        self.indices().is_some()
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    pub fn vertex_count(&self) -> usize {
        self.native().fVertexCount.try_into().unwrap()
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    pub fn index_count(&self) -> usize {
        self.native().fIndexCount.try_into().unwrap()
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn positions(&self) -> &[Point] {
        let positions: *const SkPoint = self.native().fPositions;
        unsafe { slice::from_raw_parts(positions as _, self.vertex_count()) }
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn tex_coords(&self) -> Option<&[Point]> {
        let texs: *const SkPoint = self.native().fTexs.into_option()?;
        Some(unsafe { slice::from_raw_parts(texs as _, self.vertex_count()) })
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn colors(&self) -> Option<&[Color]> {
        let colors: *const SkColor = self.native().fColors.into_option()?;
        Some(unsafe { slice::from_raw_parts(colors as _, self.vertex_count()) })
    }

    #[deprecated(since = "0.29.0", note = "returns None")]
    #[allow(deprecated)]
    pub fn bone_indices(&self) -> Option<&[BoneIndices]> {
        None
    }

    #[deprecated(since = "0.29.0", note = "returns None")]
    #[allow(deprecated)]
    pub fn bone_weights(&self) -> Option<&[BoneWeights]> {
        None
    }

    #[deprecated(since = "0.29.0", note = "will be removed without replacement")]
    #[allow(deprecated)]
    pub fn indices(&self) -> Option<&[u16]> {
        let indices = self.native().fIndices.into_option()?;
        Some(unsafe { slice::from_raw_parts_mut(indices as _, self.index_count()) })
    }

    #[deprecated(since = "0.29.0", note = "returns false")]
    pub fn is_volatile(&self) -> bool {
        false
    }

    #[deprecated(since = "0.29.0", note = "removed without replacement")]
    #[allow(deprecated)]
    pub fn apply_bones(&self, _bones: &[Bone]) -> ! {
        unimplemented!("removed without replacement")
    }

    pub fn approximate_size(&self) -> usize {
        unsafe { self.native().approximateSize() }
    }

    pub fn decode(buffer: &[u8]) -> Option<Vertices> {
        Vertices::from_ptr(unsafe { sb::C_SkVertices_Decode(buffer.as_ptr() as _, buffer.len()) })
    }

    pub fn encode(&self) -> Data {
        Data::from_ptr(unsafe { sb::C_SkVertices_encode(self.native()) }).unwrap()
    }
}

bitflags! {
    pub struct BuilderFlags: u32 {
        const HAS_TEX_COORDS = sb::SkVertices_BuilderFlags_kHasTexCoords_BuilderFlag as u32;
        const HAS_COLORS = sb::SkVertices_BuilderFlags_kHasColors_BuilderFlag as u32;
    }
}

pub type Builder = Handle<SkVertices_Builder>;

impl NativeDrop for SkVertices_Builder {
    fn drop(&mut self) {
        unsafe { sb::C_SkVertices_Builder_destruct(self) }
    }
}

impl Handle<SkVertices_Builder> {
    pub fn new(
        mode: VertexMode,
        vertex_count: usize,
        index_count: usize,
        flags: BuilderFlags,
    ) -> Builder {
        Self::from_native(unsafe {
            SkVertices_Builder::new(
                mode,
                vertex_count.try_into().unwrap(),
                index_count.try_into().unwrap(),
                flags.bits(),
            )
        })
    }

    pub fn is_valid(&self) -> bool {
        !self.native().fVertices.fPtr.is_null()
    }

    pub fn positions(&mut self) -> &mut [Point] {
        unsafe {
            let vertices = &*self.native().fVertices.fPtr;
            slice::from_raw_parts_mut(
                Point::from_native_ref_mut(&mut *vertices.fPositions),
                vertices.fVertexCount.try_into().unwrap(),
            )
        }
    }

    pub fn indices(&mut self) -> Option<&mut [u16]> {
        unsafe {
            let vertices = &*self.native().fVertices.fPtr;
            let indices = vertices.fIndices.into_option()?;
            Some(slice::from_raw_parts_mut(
                indices,
                vertices.fIndexCount.try_into().unwrap(),
            ))
        }
    }

    // TODO: customData()

    pub fn tex_coords(&mut self) -> Option<&mut [Point]> {
        unsafe {
            let vertices = &*self.native().fVertices.fPtr;
            let coords = vertices.fTexs.into_option()?;
            Some(slice::from_raw_parts_mut(
                Point::from_native_ref_mut(&mut *coords),
                vertices.fVertexCount.try_into().unwrap(),
            ))
        }
    }

    pub fn colors(&mut self) -> Option<&mut [Color]> {
        unsafe {
            let vertices = &*self.native().fVertices.fPtr;
            let colors = vertices.fColors.into_option()?;
            Some(slice::from_raw_parts_mut(
                Color::from_native_ref_mut(&mut *colors),
                vertices.fVertexCount.try_into().unwrap(),
            ))
        }
    }

    #[deprecated(since = "0.29.0", note = "returns false")]
    pub fn is_volatile(&self) -> bool {
        false
    }

    #[deprecated(since = "0.29.0", note = "returns None")]
    #[allow(deprecated)]
    pub fn bone_indices(&mut self) -> Option<&mut [BoneIndices]> {
        None
    }

    #[deprecated(since = "0.29.0", note = "returns None")]
    #[allow(deprecated)]
    pub fn bone_weights(&mut self) -> Option<&mut [BoneWeights]> {
        None
    }

    pub fn detach(mut self) -> Vertices {
        Vertices::from_ptr(unsafe { sb::C_SkVertices_Builder_detach(self.native_mut()) }).unwrap()
    }
}
