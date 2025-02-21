#version 120

varying vec2 v_TexCoord;
varying vec3 v_Normal;

uniform sampler2D u_ColorSampler;

uniform vec3 u_SunDirection;
uniform vec3 u_SkyColor, u_SunColor;

void main() {
	gl_FragColor = texture2D(u_ColorSampler, v_TexCoord);

	float diffuse = dot(v_Normal, -u_SunDirection);
	diffuse = max(diffuse, 0.0);

	const float SHADOW_AMBIENT = 0.3; // [MIN=0.0], [MAX=1.0]
	const float SKY_AMBIENT = 0.6; // [MIN=0.0], [MAX=1.0]

	gl_FragColor.rgb = mix(
		gl_FragColor.rgb * u_SkyColor * (diffuse * (1.0 - SHADOW_AMBIENT) + SHADOW_AMBIENT),
		gl_FragColor.rgb * u_SunColor,

		vec3(diffuse * (1.0 - SKY_AMBIENT) + SKY_AMBIENT)
	);
}