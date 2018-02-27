#version 150 core
precision mediump float;

in vec4 v_Color;
in vec3 v_Center;

void main() {
	gl_FragColor = v_Color;
}