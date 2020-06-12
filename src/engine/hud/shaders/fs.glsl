// #version 150

// this was the vertex shader output; it’s now our (rasterized and interpolated) input!
in vec2 v_uv;

out vec4 frag;

uniform sampler2D tex;
uniform sampler2D tex_floating;

uniform bool depth_tex;

void main(){
	if (depth_tex) {
		float v = texture(tex_floating,v_uv).r;
		frag = vec4(vec3(v), 1.0);
	} else {
		frag = texture(tex,v_uv);
	}
	//frag_color = vec3(0.5, 0, 0.5);
}