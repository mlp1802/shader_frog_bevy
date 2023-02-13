precision highp float;
precision highp int;

uniform mat4 modelMatrix;
uniform mat4 modelViewMatrix;
uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform mat3 normalMatrix;
uniform vec3 cameraPosition;


attribute vec3 position;
attribute vec3 normal;
attribute vec2 uv;
attribute vec2 uv2;

varying vec3 vReflect;

void main() {
    vec3 worldPosition = ( modelMatrix * vec4( position, 1.0 )).xyz;
    vec3 cameraToVertex = normalize( worldPosition - cameraPosition );
    vec3 worldNormal = normalize(
        mat3( modelMatrix[ 0 ].xyz, modelMatrix[ 1 ].xyz, modelMatrix[ 2 ].xyz ) * normal
    );
    vReflect = reflect( cameraToVertex, worldNormal );
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
