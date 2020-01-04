//! Representation of mesh data with its vertices and all buffer data.

use crate::renderer::buffer::Buffer;
use crate::renderer::Material;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;
use web_sys::WebGlRenderingContext;
use nalgebra::{Vector3,Vector2};

/// Mesh data as the union of its `Buffers` and the number of vertices in the mesh
pub struct MeshData {
    /// Unique identifier for this MeshData
    id: String,

    /// Vector of the buffers associated with this mesh: vertex positions, weights, etc.
    buffers: Vec<Buffer>,

    /// Number of vertices in the mesh data.  
    /// The number is not the actual number of vertices in the mesh but rather the number
    /// of vertices in its vertex buffers, with data duplication.
    ///
    /// Ex: A cube has 6 faces, made up of two triangles, and each triangle is 3 vertices
    /// which makes 6 * 2 * 3 = 36 total vertices.
    vertex_count: i32,
}

impl MeshData {
    /// Constructor. The `vertex count` must be the number of vertices in the buffer as specified
    /// on the `Self.vertex_count` property, including duplicates.
    pub fn new(id: String, vertex_count: i32) -> MeshData {
        MeshData {
            id: id,
            buffers: Vec::new(),
            vertex_count: vertex_count,
        }
    }

    /// Add a buffer to this `MeshData`
    pub fn push_buffer(&mut self, buffer: Buffer) -> () {
        self.buffers.push(buffer);
    }

    /// Returns a slice of the available buffers
    pub fn get_buffers(&self) -> &[Buffer] {
        &self.buffers
    }

    pub fn get_buffer(&self, name: &str) -> Option<&Buffer> {
        for buffer in &self.buffers {
            if buffer.get_attribute_name() == name {
                return Some(buffer);
            }
        }
        None
    }

    /// Returns the number of vertices for this `MeshData`'s Buffers.
    pub fn get_vertex_count(&self) -> i32 {
        self.vertex_count
    }

    /// Getter for `id`
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Function to lookup the locations for this meshdata;
    pub fn lookup_locations(
        &self,
        context: &WebGlRenderingContext,
        material: Rc<RefCell<Material>>,
    ) -> () {
        for buffer in &self.buffers {
            material
                .borrow_mut()
                .register_new_attribute_location(context, buffer.get_attribute_name())
        }
    }

    fn compute_tangents(&mut self) {
        if let (Some(v_buffer), Some(u_buffer), Some(_)) = (
            self.get_buffer("a_position"),
            self.get_buffer("a_tex_coordinates"),
            self.get_buffer("a_normal"),
        ) {
            
        }
    }

    fn compute_tangent(positions : [Vector3<f32>;3], uvs : [Vector2<f32>;3], index : usize) -> Vector3<f32> {

        let (i1,i2,i3) = (index, (index + 1) % 3, (index + 2) % 3);

        let delta_pos_1 = positions[i2] - positions[i1];
        let delta_pos_2 = positions[i3] - positions[i1];
        let delta_uv_1 = uvs[i2] - uvs[i1];
        let delta_uv_2 = uvs[i3] - uvs[i1];
        
        let r = 1.0 /(delta_uv_1.x * delta_uv_2.y - delta_uv_1.y * delta_uv_2.x);
        (delta_pos_1 * delta_uv_2.y   - delta_pos_2 * delta_uv_1.y)*r
    }
}
