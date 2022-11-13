#define STB_IMAGE_IMPLEMENTATION
#include <stb/stb_image.h>
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include <stb/stb_image_write.h>

#include <lilith/image.h>

img img_new(size_t width, size_t height, size_t channels){
	uint8_t* data = malloc(height * width * channels);
	if (data == NULL)
		return (img){0, 0, 0, NULL};
	
	return (img){width, height, channels, data};
}

img img_load(const char* filename){
	int width, height, channels;
	uint8_t* data = stbi_load(filename, &width, &height, &channels, 0);

	return (img){width, height, channels, data};
}

int image_write_png(img img, const char* path){
	return stbi_write_png(path, img.width, img.height, img.channels, img.data, img.width * img.channels);
}

int image_write_jpg(img img, const char* path){
	return stbi_write_jpg(path, img.width, img.height, img.channels, img.data, 100);
}

typedef int (*image_writer)(img, const char*);

const image_writer image_writers[] = {
	image_write_png,
	image_write_jpg,
};

int img_write(img img, img_fmt fmt, const char* path){
	return image_writers[fmt](img, path);
}

void img_clear(img img){
	free(img.data);
}
