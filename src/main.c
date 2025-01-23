/*
 * Copyright (C) 2025 Nicolás Ortega Froysa <nicolas@ortegas.org> All rights reserved.
 * Author: Nicolás Ortega Froysa <nicolas@ortegas.org>
 *
 * This software is provided 'as-is', without any express or implied
 * warranty. In no event will the authors be held liable for any damages
 * arising from the use of this software.
 *
 * Permission is granted to anyone to use this software for any purpose,
 * including commercial applications, and to alter it and redistribute it
 * freely, subject to the following restrictions:
 *
 * 1. The origin of this software must not be misrepresented; you must not
 *    claim that you wrote the original software. If you use this software
 *    in a product, an acknowledgment in the product documentation would be
 *    appreciated but is not required.
 *
 * 2. Altered source versions must be plainly marked as such, and must not be
 *    misrepresented as being the original software.
 *
 * 3. This notice may not be removed or altered from any source
 *    distribution.
 */

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
	const char *in_file_path = argv[1], *out_file_path = argv[2];
	FILE *in_file = NULL, *out_file = NULL;
	int ret = EXIT_SUCCESS;
	int tab_num = 0;
	char ch, last_ch = 0;

	if(argc != 3) {
		printf("Usage: %s <in-file> <out-file>\n", argv[0]);
		return EXIT_FAILURE;
	}

	in_file = fopen(in_file_path, "r");
	if(!in_file) {
		fprintf(stderr, "Failed to open file '%s'.", in_file_path);
		ret = EXIT_FAILURE;
		goto exit_err;
	}
	out_file = fopen(out_file_path, "w");
	if(!out_file) {
		fprintf(stderr, "Failed to open file '%s'.", out_file_path);
		ret = EXIT_FAILURE;
		goto exit_err;
	}

	ch = fgetc(in_file);

	while(ch != EOF) {
		switch(ch) {
		case '[':
		case '{':
			tab_num++;
			fprintf(out_file, "%c\n", ch);
			for(int i = 0; i < tab_num; ++i)
				fprintf(out_file, "  ");
			last_ch = ' ';
			break;
		case ']':
		case '}':
			tab_num--;
			fputc('\n', out_file);
			for(int i = 0; i < tab_num; ++i)
				fprintf(out_file, "  ");
			fputc(ch, out_file);
			break;
		case ',':
			fprintf(out_file, "%c\n", ch);
			for(int i = 0; i < tab_num; ++i)
				fprintf(out_file, "  ");
			last_ch = ' ';
			break;
		default:
			if(ch != ' ' || last_ch != ' ') {
				fputc(ch, out_file);
				last_ch = ch;
			}
			break;
		}

		ch = fgetc(in_file);
	}

exit_err:
	if(in_file)
		fclose(in_file);
	if(out_file)
		fclose(out_file);

	return ret;
}
