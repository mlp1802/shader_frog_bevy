#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec2 v_Uv;
layout(location = 1) out float test_out;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
    mat4 View;
    mat4 InverseView;
    mat4 Projection;
    vec3 WorldPosition;
    float width;
    float height;
};

layout(set = 2, binding = 0) uniform Mesh {
    mat4 Model;
    mat4 InverseTransposeModel;
    uint flags;
};
layout(set = 1, binding = 0) uniform CustomMaterial {
  vec4 color;
  float z_coord;
};
void main() {
    v_Uv = Vertex_Uv;
    //this is a variable being passed on the the vertex shader
    test_out = 23;
    gl_Position = ViewProj * Model * vec4(Vertex_Position, z_coord); 
}
