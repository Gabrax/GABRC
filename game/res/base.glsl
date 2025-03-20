#version 330 core
in vec2 fragTexCoord;
out vec4 fragColor;

uniform float time;

void main() {
    float r = sin(time) * 0.5 + 0.5; // Red channel oscillates over time
    float g = cos(time) * 0.5 + 0.5; // Green channel oscillates over time
    fragColor = vec4(r, g, 1.0, 1.0); // Blue remains static
}
