#version 330

uniform mat4 matrix;

in vec3 position;
in vec3 normal;

out vec3 v_normal;

void main(){
	v_normal = transpose(inverse(mat3(matrix))) * normal;
	gl_Position = matrix * vec4(position, 1.0);
}
