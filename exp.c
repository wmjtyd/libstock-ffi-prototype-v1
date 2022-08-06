#include "libstock.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    PriceDataField_t price_data_field = {
        (char*)"100.0",
        (char*)"30.0"
    };
    uint8_t buf[4096] = {0};
    size_t written_size = 0;

    // slice_mut_uint8_t slice = { buf, 4096 };

    // // int return_value = serialize_price_data_field(&price_data_field, &slice, &written_size);
    // printf("Return Value: %d; written_size: %zu\n", return_value, written_size);

    // PriceDataField_t* des_price_data_field = new_price_data_field();

    // slice_ref_uint8_t slice_ref = { buf, 4096 };

    // // int return_value_2 = deserialize_price_data_field(&slice_ref, des_price_data_field);
    // printf("Return Value: %d; F1 = %s, F2 = %s\n", return_value_2, des_price_data_field->price, des_price_data_field->quantity_base);

    // free_price_data_field(des_price_data_field);
    return 0;
}
