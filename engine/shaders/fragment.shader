#version 330

in vec3 v_normal;

out vec4 color;

void main(){
	vec3 light = vec3(1., 0., 0.);

	float brightness = dot(normalize(v_normal), normalize(light));

	vec3 dark_color = vec3(0., 0., 0.);
	vec3 ligth_color = vec3(1., 1., 1.);

	color = vec4(mix(dark_color, ligth_color, brightness), 1.0);
}
