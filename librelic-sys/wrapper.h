#include <relic/relic.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct {
  bn_t value;
} wrapper_bn_t;

int wrapper_get_order(wrapper_bn_t* bn);

int wrapper_bn_init(wrapper_bn_t* bn);
#if 0
int wrapper_bn_free(wrapper_bn_t* bn);
#endif
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

typedef struct {
  g1_t value;
} wrapper_g1_t;

int wrapper_g1_init(wrapper_g1_t* g1);
int wrapper_g1_neutral(wrapper_g1_t* g1);
int wrapper_g1_generator(wrapper_g1_t* g1);
int wrapper_g1_rand(wrapper_g1_t* g1);
int wrapper_g1_add_assign(wrapper_g1_t* dst, const wrapper_g1_t* rhs);
int wrapper_g1_add(wrapper_g1_t* dst, const wrapper_g1_t* lhs, const wrapper_g1_t* rhs);
int wrapper_g1_double(wrapper_g1_t* dst, const wrapper_g1_t* src);
int wrapper_g1_neg(wrapper_g1_t* g1);
int wrapper_g1_sub_assign(wrapper_g1_t* dst, const wrapper_g1_t* rhs);
int wrapper_g1_sub(wrapper_g1_t* dst, const wrapper_g1_t* lhs, const wrapper_g1_t* rhs);
int wrapper_g1_mul_assign(wrapper_g1_t* dst, const wrapper_bn_t* rhs);
int wrapper_g1_mul(wrapper_g1_t* dst, const wrapper_g1_t* lhs, const wrapper_bn_t* rhs);
int wrapper_g1_norm(wrapper_g1_t* dst, const wrapper_g1_t* src);
int wrapper_g1_size_bin(size_t* size, const wrapper_g1_t* g1);
int wrapper_g1_write_bin(uint8_t* dst, size_t len, const wrapper_g1_t* src);
int wrapper_g1_read_bin(wrapper_g1_t* dst, const uint8_t* src, size_t len);
bool wrapper_g1_is_neutral(const wrapper_g1_t* value);
bool wrapper_g1_is_valid(const wrapper_g1_t* value);
bool wrapper_g1_is_equal(const wrapper_g1_t* lhs, const wrapper_g1_t* rhs);