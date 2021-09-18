#version 450 core

in layout(location=2) vec4 inColor;
uniform vec3 uColor = vec3(0.9f, 0.2f, 0.1f);
out vec4 color;

void main()
{
    color = inColor;
    //color = vec4(uColor[0], uColor[1], uColor[2], 1.0f);
    //color = inColor
}