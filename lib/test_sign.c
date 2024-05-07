#include <stdio.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "sign.h"

void main() {
    char *data = "test";
    char *PIN = "12345678";
    char *key_pair_id = "7e4a9abc9-403b-";

    size_t result_size = 0;
    uint8_t *result = perform_signing(data, strlen(data), &result_size, PIN, strlen(PIN), key_pair_id, strlen(key_pair_id));
    if (result == NULL || result_size == 0){
        puts("perform_signing failed");
    } else {
        free(result);
		puts("test passed");
	}
}
