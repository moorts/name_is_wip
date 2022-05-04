#version 300 es
precision highp float;

in vec2 v_texcoord;

uniform sampler2D u_texture;
uniform float u_scale;

out vec4 outColor;

void main() {
  // funny hack to achieve 1-bit per pixel
  ivec2 uv = ivec2(gl_FragCoord.xy / u_scale);
  uv.x = (uv.x / int(8));
  int bit = int((gl_FragCoord.x / (8.0 * u_scale) - float(uv.x)) * 8.0);
  uint texel = uint(texelFetch(u_texture, uv, 0).r * 255.0);
  uint mask = 1u << bit;
  uint result = (texel & mask) != 0u ? 1u : 0u;
  outColor = vec4(float(result), float(result), float(result), 1.0);
}
