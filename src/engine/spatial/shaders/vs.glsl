// vertex shader
in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 frag_pos;
out vec4 light_frag_pos;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;
uniform mat4 normal_m;

uniform mat4 light_view;

void main(){
	v_normal=mat3(transpose(normal_m))*normal;
	gl_Position=projection*view*model*vec4(position,1.);
	frag_pos=vec3(model*vec4(position, 1.0));
	light_frag_pos=light_view * vec4(frag_pos, 1.0);
}