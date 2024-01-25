#version 430

#define CELL_SIZE 1.0

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform vec2 dimensions;

layout(std430, binding = 2) buffer matrixBuffer {
    int ids[];
};
layout(std430, binding = 3) buffer matrixBufferCopy {
    int idsCopy[];
};

void main() {
    int width = int(floor(dimensions.x / CELL_SIZE));
    int height = int(floor(dimensions.y / CELL_SIZE));
    int i = int(gl_GlobalInvocationID.y) * width + int(gl_GlobalInvocationID.x);
    int id = idsCopy[i];
    if (gl_GlobalInvocationID.y < height-1) {
        if (id == 1 && gl_GlobalInvocationID.y < height-1) {
            if (idsCopy[i+width] == 0) {
                ids[i] = 0;
                ids[i+width] = 1;
            } else if (idsCopy[i+width-1] == 0 && gl_GlobalInvocationID.x > 0) {
                ids[i] = 0;
                ids[i+width-1] = 1;
            } else if (idsCopy[i+width+1] == 0 && gl_GlobalInvocationID.x < width-1) {
                ids[i] = 0;
                ids[i+width+1] = 1;
            }
        }/* else if (id == 0) {
            if (idsCopy[i-width] == 1) {
                ids[i] = 1;
            }
        }*/
    }
}
