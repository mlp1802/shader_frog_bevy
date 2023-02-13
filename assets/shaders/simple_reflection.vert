//Original code https://shaderfrog.com/app/view/4568
precision highp float;
precision highp int;

// uniform mat4 modelMatrix;
// uniform mat4 modelViewMatrix;
// uniform mat4 projectionMatrix;
// uniform mat4 viewMatrix;
// uniform mat3 normalMatrix;
// uniform vec3 cameraPosition;

layout(set = 0, binding = 0) uniform CameraViewProj {
  mat4 projectionMatrix;
  mat4 modelViewMatrix;
  mat4 modelMatrix;
  mat4 viewMatrix;
  vec3 worldPosition;
  float width;
  float height;
};

// attribute vec3 position;
// attribute vec3 normal;
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

// attribute vec2 uv;  // NOT USED
// attribute vec2 uv2; //NOT USED



//varying vec3 vReflect;
layout(location = 0) out vec3  vReflect; 
layout(set = 1, binding = 0) uniform CustomMaterial {
  float mirror_reflection;
  vec3 cameraPosition;
};

void main() {
    vec3 worldPosition = ( modelMatrix * vec4( position, 1.0 )).xyz;
    vec3 cameraToVertex = normalize( worldPosition - cameraPosition );
//    vec3 worldNormal = normalize(
//        mat3( modelMatrix[0].xyz, modelMatrix[1].xyz, modelMatrix[2].xyz ) * normal
//    );
    vec3 worldNormal = normalize(
                                 mat3(vec3(modelMatrix[0]),
                                      vec3(modelMatrix[1]),
                                      vec3(modelMatrix[2])) * normal
                                 );

    vReflect = reflect( cameraToVertex, worldNormal );
    //gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
    gl_Position = projectionMatrix * viewMatrix * vec4(position,0.5);
}
