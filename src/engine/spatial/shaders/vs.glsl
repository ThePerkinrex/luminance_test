// vertex shader
in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 frag_pos;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;
uniform mat4 normal_m;

void main(){
	v_normal=mat3(transpose(normal_m))*normal;
	gl_Position=projection*view*model*vec4(position,1.);
	frag_pos=position;
}