precision highp float;
precision highp int;

varying vec3 vReflect;

uniform mat4 modelMatrix;
uniform mat4 modelViewMatrix;
uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform mat3 normalMatrix;
uniform vec3 cameraPosition;

uniform float mirrorReflection;
uniform sampler2D reflectionSampler;

void main() {
    vec4 cubeColor = texture2D( reflectionSampler, vec3( mirrorReflection * vReflect.x, vReflect.yz ).xy );
    //vec4 cubeColor = texture2D( reflectionSampler, vec2(1,0) ); 
    cubeColor.w = 1.0;
    gl_FragColor = cubeColor;
}
