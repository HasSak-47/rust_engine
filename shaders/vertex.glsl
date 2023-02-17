#version 330

uniform mat4 matrix;

in vec3 position;
in vec3 normal;

void main(){
	gl_Position = matrix * vec4(position, 0.0);
}
