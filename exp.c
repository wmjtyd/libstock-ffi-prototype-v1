#include "libstock.h"
#include <stdio.h>

int main() {
    PriceDataField_t price_data_field = {
        (char*)"100.0",
        (char*)"30.0"
    };
    uint8_t buf[4096] = {0};
    size_t written_size = 0;

    slice_mut_uint8_t slice = { buf, 4096 };

    int return_value = serialize_price_data_field(&price_data_field, &slice, &written_size);
    printf("Return Value: %d; written_size: %zu\n", return_value, written_size);

    buf[written_size] = '\0';
    printf("what we received: %s\n", buf);

    return 0;
}
