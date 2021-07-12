pub mod stl;
pub mod triangle;

use cgmath::{Matrix4, Point2, Point3, Transform, Vector3};

/// A mesh of triangles.
pub struct Mesh<'mtrx> {
    object_to_world: &'mtrx Matrix4<f32>,
    world_to_object: &'mtrx Matrix4<f32>,
    reverse_orientation: bool,

    /// The vertices in world space that make up the mesh.
    world_space_vertices: Vec<Point3<f32>>,

    /// An array that describes each triangle in the mesh. Each element of the
    /// array is a tuple that contains three indices into the `vertices` array.
    triangle_vertex_indices: Vec<(usize, usize, usize)>,

    /// An array containing a tangent vector for each vertex in the mesh.
    tangents: Option<Vec<Vector3<f32>>>,

    /// An array containing a normal vector for each vertex in the mesh.
    normals: Option<Vec<Vector3<f32>>>,

    /// An array containing a UV coordinate for each vertex in the mesh.
    uvs: Option<Vec<Point2<f32>>>,
}

pub struct MeshBuilder<'mtrx> {
    object_to_world: &'mtrx Matrix4<f32>,
    world_to_object: &'mtrx Matrix4<f32>,
    reverse_orientation: bool,
    object_space_vertices: Vec<Point3<f32>>,
    triangle_vertex_indices: Vec<(usize, usize, usize)>,
    tangents: Option<Vec<Vector3<f32>>>,
    normals: Option<Vec<Vector3<f32>>>,
    uvs: Option<Vec<Point2<f32>>>,
}

impl<'mtrx> MeshBuilder<'mtrx> {
    pub fn new(
        object_to_world: &'mtrx Matrix4<f32>,
        world_to_object: &'mtrx Matrix4<f32>,
        reverse_orientation: bool,
        object_space_vertices: Vec<Point3<f32>>,
        triangle_vertex_indices: Vec<(usize, usize, usize)>,
    ) -> Self {
        Self {
            object_to_world,
            world_to_object,
            reverse_orientation,
            object_space_vertices,
            triangle_vertex_indices,
            tangents: None,
            normals: None,
            uvs: None,
        }
    }

    pub fn tangents(mut self, tangents: Vec<Vector3<f32>>) -> Self {
        self.tangents = Some(tangents);
        self
    }

    pub fn normals(mut self, normals: Vec<Vector3<f32>>) -> Self {
        self.normals = Some(normals);
        self
    }

    pub fn uvs(mut self, uvs: Vec<Point2<f32>>) -> Self {
        self.uvs = Some(uvs);
        self
    }

    pub fn build(self) -> Mesh<'mtrx> {
        Mesh {
            object_to_world: self.object_to_world,
            world_to_object: self.world_to_object,
            reverse_orientation: self.reverse_orientation,
            world_space_vertices: self
                .object_space_vertices
                .iter()
                .map(|p| self.object_to_world.transform_point(*p))
                .collect(),
            triangle_vertex_indices: self.triangle_vertex_indices,
            tangents: self.tangents,
            normals: self.normals,
            uvs: self.uvs,
        }
    }
}
