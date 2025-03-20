#version 330 core

// Input from vertex shader
in vec2 fragTexCoord;
out vec4 fragColor;

// Uniforms (passed from CPU)
uniform float mapData[64]; // 8x8 map
uniform float screen_width;
uniform float screen_height;

uniform float player_pos_x;
uniform float player_pos_y;
uniform float player_dir_x;
uniform float player_dir_y;
uniform float player_proj_x;
uniform float player_proj_y;

void main() {
    ivec2 gridSize = ivec2(8, 8); // Map is 8x8
    vec2 cellSize = vec2(1.0 / gridSize.x, 1.0 / gridSize.y);

    ivec2 cell = ivec2(fragTexCoord / cellSize); // Convert to grid coordinates
    int index = cell.y * gridSize.x + cell.x;    // Convert 2D to 1D index

    vec3 color = vec3(0.2); // Default background color

    // Draw walls
    if (index >= 0 && index < 64 && mapData[index] > 0.0) {
        color = vec3(1.0, 1.0, 1.0); // White for walls
    }

    // Draw player as a small circle
    vec2 playerPixel = vec2(player_pos_x / gridSize.x, player_pos_y / gridSize.y);
    if (distance(fragTexCoord, playerPixel) < 0.02) {
        color = vec3(1.0, 0.0, 0.0); // Red for player
    }

    fragColor = vec4(color, 1.0);
}
