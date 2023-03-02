#version 330

uniform mat4 matrix;

in vec3 pos;
in vec3 nor;
in vec2 tex;

void main(){
	gl_Position = matrix * vec4(pos, 1.0);
}
