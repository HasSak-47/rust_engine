#include <stdint.h>

float perlin_noise(float x, float y, float z, uint64_t xwrap, uint64_t ywrap, uint64_t zwrap);
float perlin_noise_seed(float x, float y, float z, uint64_t xwrap, uint64_t ywrap, uint64_t zwrap, uint64_t seed);

