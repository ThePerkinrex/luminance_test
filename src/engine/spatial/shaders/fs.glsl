// fragment shader
in vec3 v_normal;
in vec3 frag_pos;

// we will output a single color
out vec4 frag_color;

uniform vec3 obj_color_diffuse;
uniform vec3 obj_color_specular;
uniform float obj_specular_coefficient;
uniform vec3 view_pos;

void main(){
	vec3 lightColor=vec3(1.,1.,1.);
	
	float ambientStrength=.75;
	vec3 ambient=ambientStrength*lightColor;
	
	//vec3 obj_color=ambient*obj_color_diffuse;
	
	// light direction
	vec3 lightPos=vec3(-1.1,2.,3.);
	
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
	
	vec3 result=(ambient+diffuse+specular)*obj_color_diffuse;
	
	frag_color=vec4(result,1.);
}