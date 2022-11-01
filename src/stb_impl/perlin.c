#include <lilith/perlin.h>

#define STB_PERLIN_IMPLEMENTATION
#include <stb/stb_perlin.h>

float perlin_noise(float x, float y, float z, uint64_t xwrap, uint64_t ywrap, uint64_t zwrap){
	return stb_perlin_noise3(x, y, z, xwrap, ywrap, zwrap);
}
float perlin_noise_seed(float x, float y, float z, uint64_t xwrap, uint64_t ywrap, uint64_t zwrap, uint64_t seed){
	return stb_perlin_noise3_seed(x, y, z, xwrap, ywrap, zwrap, seed);
}


