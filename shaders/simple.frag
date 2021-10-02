#version 450 core

in layout(location=3) vec4 inColor;
in layout(location=4) vec3 inNormals;

uniform vec3 uColor = vec3(0.9f, 0.2f, 0.1f);
out vec4 color;
vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));

void main()
{
    float scalar =max(dot(-lightDirection, inNormals),0);
    color = vec4(scalar*inColor.xyz, inColor[3]);
    //color = vec4(inNormals, 1.0f);
}