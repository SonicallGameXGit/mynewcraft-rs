#version 130

const vec3 c_Normals[6] = vec3[6](
    vec3(-1.0,  0.0,  0.0),
    vec3( 1.0,  0.0,  0.0),
    vec3( 0.0, -1.0,  0.0),
    vec3( 0.0,  1.0,  0.0),
    vec3( 0.0,  0.0, -1.0),
    vec3( 0.0,  0.0,  1.0)
);

in uint a_Data;

uniform mat4 u_MVPMatrix;
uniform vec2 u_AtlasScalar;

out vec2 v_TexCoord;
out vec3 v_Normal;

void main() {
    uint    x =  a_Data        & uint( 0x1F);
    uint    y = (a_Data >>  5) & uint(0x1FF);
    uint    z = (a_Data >> 14) & uint( 0x1F);
    
    uint    u = (a_Data >> 19) & uint( 0x1F);
    uint    v = (a_Data >> 24) & uint( 0x1F);

    uint face = (a_Data >> 29) & uint(  0x5);

	gl_Position = u_MVPMatrix * vec4(float(x), float(y), float(z), 1.0);

	v_TexCoord = vec2(float(u), float(v)) * u_AtlasScalar;
    v_TexCoord.y = 1.0 - v_TexCoord.y;

    v_Normal = c_Normals[face];
}