#version 300 es
precision highp float;

in vec2 v_texcoord;

uniform sampler2D u_texture;

out vec4 outColor;

void main() {
  vec4 col = texture(u_texture, v_texcoord);
  col.a = 1.0;
  outColor = col;
}
