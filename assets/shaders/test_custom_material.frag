#version 450
layout(location = 0) in vec2 v_Uv;
layout(location = 1) in float testout;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform CustomMaterial {
    vec4 color;
    float z_coord;
};

layout(set = 1, binding = 1) uniform texture2D CustomMaterial_texture;
layout(set = 1, binding = 2) uniform sampler CustomMaterial_sampler;


void main() {
    o_Target = color * texture(sampler2D(CustomMaterial_texture,CustomMaterial_sampler), v_Uv);
}
