// fragment shader
in vec3 v_normal;

// we will output a single color
out vec4 frag_color;

uniform vec3 obj_color_diffuse;
uniform vec3 obj_color_ambient;

void main(){
	vec3 obj_color = obj_color_diffuse * obj_color_ambient * 1.4;
	
	// light direction
	vec3 light_dir=vec3(0.,-1.,-.5)*4;
	
	// diffusion factor (hence the k)
	float kd=dot(v_normal,-light_dir);
	
	frag_color=vec4(obj_color*kd, 1.0);
}