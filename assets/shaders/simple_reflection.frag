//Original code https://shaderfrog.com/app/view/4568
precision highp float;
precision highp int;

//varying vec3 vReflect;
layout(location = 0) in vec3  vReflect; 
layout(location = 0) out vec4 o_Target;
//uniform mat4 modelMatrix;
//uniform mat4 modelViewMatrix;
//uniform mat4 projectionMatrix;
//uniform mat4 viewMatrix;
//uniform mat3 normalMatrix;
//uniform vec3 cameraPosition;

layout(set = 0, binding = 0) uniform CameraViewProj {
  mat4 projectionMatrix;
  mat4 modelViewMatrix;
  mat4 modelMatrix;
  mat4 viewMatrix;
  vec3 worldPosition;
  float width;
  float height;
};
layout(set = 1, binding = 0) uniform CustomMaterial {
  float mirror_reflection;
  vec3 cameraPosition;
};
//uniform sampler2D reflectionSampler;

layout(set = 1, binding = 1) uniform texture2D CustomMaterial_texture;
layout(set = 1, binding = 2) uniform sampler CustomMaterial_sampler;

void main() {
     //vec4 cubeColor = texture2D( CustomMaterial_sampler, vec3( mirror_reflection * vReflect.x, vReflect.yz ).xy );

     //vec4 cubeColor = texture2D( CustomMaterial_sampler);
    vec4 cubeColor = texture(sampler2D(CustomMaterial_texture,CustomMaterial_sampler),vec3( mirror_reflection * vReflect.x, vReflect.yz ).xy);
    cubeColor.w = 1.0;
    o_Target = cubeColor;
    //vec4 cubeColor = texture2D( reflectionSampler, vec2(1,0) ); 
 //   o_Target = cubeColor;
}
