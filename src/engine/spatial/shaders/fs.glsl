// fragment shader
in vec3 v_normal;
in vec3 frag_pos;
in vec4 light_frag_pos;

// we will output a single color
out vec4 frag_color;

uniform vec3 obj_color_diffuse;
uniform vec3 obj_color_specular;
uniform float obj_specular_coefficient;
uniform vec3 view_pos;
uniform vec3 light_pos;

uniform sampler2D shadow_map;

float shadow_calculation(vec4 light_frag_pos_matrix){
	vec3 proj_coords=(light_frag_pos_matrix.xyz/light_frag_pos_matrix.w)*.5+.5;
	
	float current_depth=proj_coords.z;
	float closest_depth=texture(shadow_map,proj_coords.xy).r;
	
	float shadow=current_depth>closest_depth?1.:0.;
	return shadow;
}

void main(){
	vec3 lightColor=vec3(1.,1.,1.);
	
	float ambientStrength=.5;
	vec3 ambient=ambientStrength*lightColor;
	
	//vec3 obj_color=ambient*obj_color_diffuse;
	
	// light direction
	vec3 lightPos=light_pos;
	
	// diffusion factor (hence the k)
	vec3 norm=normalize(v_normal);
	vec3 lightDir=normalize(lightPos-frag_pos);
	
	float diff=max(dot(norm,lightDir),0.);
	vec3 diffuse=diff*lightColor;
	
	// float specularStrength=.5;
	vec3 viewDir=normalize(view_pos-frag_pos);
	vec3 reflectDir=reflect(-lightDir,norm);
	
	float spec=pow(max(dot(viewDir,reflectDir),0.),obj_specular_coefficient);
	vec3 specular=obj_color_specular*spec*lightColor;
	
	float shadow = shadow_calculation(light_frag_pos);
	vec3 result=(ambient+(1.-shadow)*(diffuse+specular))*obj_color_diffuse;
	
	frag_color=vec4(result,1.);
}