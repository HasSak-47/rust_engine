#ifndef __LILITH_IMAGE_H__
#define __LILITH_IMAGE_H__

#ifdef __cpp
extern "C"{
#endif

#include <stdint.h>
#include <stddef.h>

typedef struct img{
	size_t width;
	size_t height;
	size_t channels;

	uint8_t* data;

} img;

typedef enum img_fmt{
	PNG = 0,
	JPG,
	KOI,
	QOI,
} img_fmt;

img img_new (size_t width, size_t height, size_t channels);
img img_load(const char* path);
int img_write(img image, img_fmt format, const char* path);
void img_clear(img image);

#ifdef __cpp
extern "C"{
#endif

#endif
