// #version 150

// this was the vertex shader output; it’s now our (rasterized and interpolated) input!
in vec2 v_uv;

out vec4 frag;

uniform sampler2D tex;

void main(){
	frag = texture(tex,v_uv);
	//frag_color = vec3(0.5, 0, 0.5);
}