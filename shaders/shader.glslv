#version 150 core
precision mediump float;

in vec4 a_Pos;
in vec4 a_Center;
in vec3 a_Color;

out vec4 v_Color;
out vec3 v_Center;

layout (std140)
uniform Locals {
	mat4 u_Transform;
};

void main() {
	v_Color = vec4(a_Color, 1.);
	gl_Position = u_Transform * a_Pos;
	gl_ClipDistance[0] = 1.;
}