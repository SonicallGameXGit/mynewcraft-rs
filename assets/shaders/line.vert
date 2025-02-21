#version 130

in vec3 a_Position;
in uint a_Color;

uniform mat4 u_ProjectViewMatrix;
uniform ivec2 u_RenderOffset;

out vec4 v_Color;

void main() {
	gl_Position = u_ProjectViewMatrix * vec4(a_Position.x - u_RenderOffset.x, a_Position.y, a_Position.z - u_RenderOffset.y, 1.0);
	// gl_Position.z -= 0.00025;

	v_Color = vec4(
		float((a_Color >> 24) & uint(0xFF)) / 255.0,
		float((a_Color >> 16) & uint(0xFF)) / 255.0,
		float((a_Color >>  8) & uint(0xFF)) / 255.0,
		float( a_Color        & uint(0xFF)) / 255.0
	);
}