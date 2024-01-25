#version 430

#define CELL_SIZE 1.0

layout(origin_upper_left) in vec4 gl_FragCoord;

uniform vec2 dimensions;

layout(std430, binding = 2) buffer matrixBuffer {
    int ids[];
};

out vec4 color;

void main() {
    float x = floor(gl_FragCoord.x / CELL_SIZE);
    float y = floor(gl_FragCoord.y / CELL_SIZE);
    int i = int(floor((y * (dimensions.x / CELL_SIZE)) + x));
    int id = ids[i];
    /*if (int(gl_FragCoord.x) % int(CELL_SIZE) == 0 || int(gl_FragCoord.y) % int(CELL_SIZE) == 0) {
        color = vec4(0.1, 0.1, 0.1, 1.0);
    } else {*/
        if (id == 0) {
            color = vec4(0.0, 0.0, 0.0, 1.0);
        } else if (id == 1) {
            color = vec4(0.96, 0.773, 0.259, 1.0);
        } else if (id == 2) {
            color = vec4(0.5, 0.5, 0.5, 1.0);
        } else {
            color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    //}
}
