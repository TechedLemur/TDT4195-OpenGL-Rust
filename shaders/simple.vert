#version 450 core

in layout(location=0) vec3 position;

in layout(location=1) vec4 color;

in layout(location=2) vec3 normals;

out layout(location=3) vec4 outColor;

out layout(location=4) vec3 outNormals;

//vec3 mirrorVector = vec3(-1.0, -1.0, 1.0);
uniform float oscilator = 0.5;

uniform mat4x4 mvp = {{1.0,0.0,0.0,0.0}, {0.0,1.0,0.0,0.0}, {0.0,0.0,1.0,0.0}, {0.0,0.0,0.0,1.0}};
uniform mat4x4 model = {{1.0,0.0,0.0,0.0}, {0.0,1.0,0.0,0.0}, {0.0,0.0,1.0,0.0}, {0.0,0.0,0.0,1.0}};


void main()
{
    //gl_Position = vec4(position*scaler, 1.0f);
    gl_Position = mvp * vec4(position, 1.0f) ;
    outColor = color;
    outNormals = normalize(mat3(model)*normals);
}