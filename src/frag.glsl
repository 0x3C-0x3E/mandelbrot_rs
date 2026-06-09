#version 100

precision highp float;

varying vec4 color;
varying vec2 uv;

uniform vec2 iResolution;
uniform float zoom;
uniform vec2 pos;

#define ITERATIONS 2000

vec2 cmul(vec2 a, vec2 b) {
    return vec2(
        a.x * b.x - a.y * b.y,
        a.x * b.y + a.y * b.x
    );
}

vec2 cpow2(vec2 a) {
    return cmul(a, a);
}

void main() {
    vec2 uv = (gl_FragCoord.xy - .5 * iResolution.xy) / iResolution.y;
    vec3 col = vec3(0);

    vec2 z = vec2(0);

    vec2 c = vec2(uv.x * zoom - pos.x, uv.y * zoom - pos.y);
    int i;
    for (i = 0; i < ITERATIONS; ++i) {
        z = cpow2(z) + c;
        if (length(z) > 4.0) {
            break;
        }
    }

    if (!(i == ITERATIONS)) {
        float smooth_iter = float(i) + 1.0 - log2(log2(length(z) / 2.0));
        float t = smooth_iter / float(ITERATIONS);
        col = vec3(t);
    }

    gl_FragColor = vec4(col, 0.0);
}
