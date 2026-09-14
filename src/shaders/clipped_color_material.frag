uniform vec4 clipPlanes[MAX_CLIP_PLANES];
uniform int numClipPlanes;

in vec3 pos;
in vec4 col;

layout (location = 0) out vec4 outColor;

void main()
{
    for (int i = 0; i < MAX_CLIP_PLANES; ++i)
    {
        if (i >= numClipPlanes) break;

        vec4 plane = clipPlanes[i];
        if (dot(pos, plane.xyz) > plane.w) discard;
    }

    outColor = col;
    outColor.rgb = color_mapping(outColor.rgb);
}
