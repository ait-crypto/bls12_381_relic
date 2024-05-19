#include <relic/relic.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct {
  bn_t value;
} wrapper_bn_t;

int wrapper_get_order(wrapper_bn_t* bn);

int wrapper_bn_init(wrapper_bn_t* bn);
int wrapper_bn_free(wrapper_bn_t* bn);
int wrapper_bn_copy(wrapper_bn_t* dst, const wrapper_bn_t* src);
int wrapper_bn_zero(wrapper_bn_t* bn);
int wrapper_bn_one(wrapper_bn_t* bn);
int wrapper_bn_add_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs);
int wrapper_bn_add(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs);
int wrapper_bn_double(wrapper_bn_t* dst, const wrapper_bn_t* src);
int wrapper_bn_neg(wrapper_bn_t* bn);
int wrapper_bn_sub_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs);
int wrapper_bn_sub(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs);
int wrapper_bn_mul_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs);
int wrapper_bn_mul(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs);
int wrapper_bn_inv(wrapper_bn_t* val);
int wrapper_bn_size_bin(size_t* size, const wrapper_bn_t* bn);
int wrapper_bn_write_bin(uint8_t* dst, size_t len, const wrapper_bn_t* src);
int wrapper_bn_read_bin(wrapper_bn_t* dst, const uint8_t* src, size_t len);
int wrapper_bn_rand(wrapper_bn_t* dst, const uint8_t* src, size_t len);
bool wrapper_bn_is_zero(const wrapper_bn_t* value);
bool wrapper_bn_is_odd(const wrapper_bn_t* value);