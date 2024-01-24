#version 330

out vec4 color;

uniform vec2 dimensions;

void main() {
    float u = gl_FragCoord.x / dimensions.x;
    float v = gl_FragCoord.y / dimensions.y;
    color = vec4(u, v, 0.0, 1.0);
}
