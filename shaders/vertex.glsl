#version 330

uniform float angle;

in vec2 position;

void main(){
	mat2 rotation_matrix = mat2(
		cos(angle), -sin(angle),
		sin(angle),  cos(angle)
	);
	vec2 p = rotation_matrix * position;
	gl_Position = vec4(p, 0.0, 1.0);
}
