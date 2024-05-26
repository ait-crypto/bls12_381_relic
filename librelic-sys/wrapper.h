#ifndef RELIC_WRAPPER_H
#define RELIC_WRAPPER_H

#include <relic/relic.h>
#include <stdint.h>
#include <stdbool.h>

typedef bn_t wrapper_bn_t;

void wrapper_get_order(wrapper_bn_t* bn);

void wrapper_bn_init(wrapper_bn_t* bn);
void wrapper_bn_copy(wrapper_bn_t* dst, const wrapper_bn_t* src);
void wrapper_bn_zero(wrapper_bn_t* bn);
void wrapper_bn_one(wrapper_bn_t* bn);
void wrapper_bn_add_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs);
void wrapper_bn_add(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs);
void wrapper_bn_double(wrapper_bn_t* dst, const wrapper_bn_t* src);
void wrapper_bn_neg(wrapper_bn_t* bn);
void wrapper_bn_sub_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs);
void wrapper_bn_sub(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs);
void wrapper_bn_mul_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs);
void wrapper_bn_mul(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs);
int wrapper_bn_inv(wrapper_bn_t* val);
void wrapper_bn_size_bin(size_t* size, const wrapper_bn_t* bn);
void wrapper_bn_write_bin(uint8_t* dst, size_t len, const wrapper_bn_t* src);
int wrapper_bn_read_bin(wrapper_bn_t* dst, const uint8_t* src, size_t len, bool pack);
void wrapper_bn_rand(wrapper_bn_t* dst, const uint8_t* src, size_t len);
bool wrapper_bn_is_zero(const wrapper_bn_t* value);
bool wrapper_bn_is_odd(const wrapper_bn_t* value);

typedef g1_t wrapper_g1_t;

void wrapper_g1_init(wrapper_g1_t* g1);
void wrapper_g1_neutral(wrapper_g1_t* g1);
void wrapper_g1_generator(wrapper_g1_t* g1);
void wrapper_g1_hash_to_curve(wrapper_g1_t* g1, const uint8_t* msg, size_t len, const uint8_t* dst, size_t dst_len);
void wrapper_g1_rand(wrapper_g1_t* g1);
void wrapper_g1_add_assign(wrapper_g1_t* dst, const wrapper_g1_t* rhs);
void wrapper_g1_add(wrapper_g1_t* dst, const wrapper_g1_t* lhs, const wrapper_g1_t* rhs);
void wrapper_g1_double(wrapper_g1_t* dst, const wrapper_g1_t* src);
void wrapper_g1_neg(wrapper_g1_t* g1);
void wrapper_g1_sub_assign(wrapper_g1_t* dst, const wrapper_g1_t* rhs);
void wrapper_g1_sub(wrapper_g1_t* dst, const wrapper_g1_t* lhs, const wrapper_g1_t* rhs);
void wrapper_g1_mul_assign(wrapper_g1_t* dst, const wrapper_bn_t* rhs);
void wrapper_g1_mul(wrapper_g1_t* dst, const wrapper_g1_t* lhs, const wrapper_bn_t* rhs);
void wrapper_g1_simmul(wrapper_g1_t* dst, const wrapper_g1_t* g1s, const wrapper_bn_t* scalars, size_t len);
void wrapper_g1_norm(wrapper_g1_t* dst, const wrapper_g1_t* src);
void wrapper_g1_size_bin(size_t* size, const wrapper_g1_t* g1);
void wrapper_g1_write_bin(uint8_t* dst, size_t len, const wrapper_g1_t* src);
int wrapper_g1_read_bin(wrapper_g1_t* dst, const uint8_t* src, size_t len);
bool wrapper_g1_is_neutral(const wrapper_g1_t* value);
bool wrapper_g1_is_valid(const wrapper_g1_t* value);
bool wrapper_g1_is_equal(const wrapper_g1_t* lhs, const wrapper_g1_t* rhs);

typedef g2_t wrapper_g2_t;

void wrapper_g2_init(wrapper_g2_t* g2);
void wrapper_g2_neutral(wrapper_g2_t* g2);
void wrapper_g2_generator(wrapper_g2_t* g2);
void wrapper_g2_hash_to_curve(wrapper_g2_t* g2, const uint8_t* msg, size_t len, const uint8_t* dst, size_t dst_len);
void wrapper_g2_rand(wrapper_g2_t* g2);
void wrapper_g2_add_assign(wrapper_g2_t* dst, const wrapper_g2_t* rhs);
void wrapper_g2_add(wrapper_g2_t* dst, const wrapper_g2_t* lhs, const wrapper_g2_t* rhs);
void wrapper_g2_double(wrapper_g2_t* dst, const wrapper_g2_t* src);
void wrapper_g2_neg(wrapper_g2_t* g2);
void wrapper_g2_sub_assign(wrapper_g2_t* dst, const wrapper_g2_t* rhs);
void wrapper_g2_sub(wrapper_g2_t* dst, const wrapper_g2_t* lhs, const wrapper_g2_t* rhs);
void wrapper_g2_mul_assign(wrapper_g2_t* dst, const wrapper_bn_t* rhs);
void wrapper_g2_mul(wrapper_g2_t* dst, const wrapper_g2_t* lhs, const wrapper_bn_t* rhs);
void wrapper_g2_norm(wrapper_g2_t* dst, const wrapper_g2_t* src);
void wrapper_g2_size_bin(size_t* size, const wrapper_g2_t* g2);
void wrapper_g2_write_bin(uint8_t* dst, size_t len, const wrapper_g2_t* src);
int wrapper_g2_read_bin(wrapper_g2_t* dst, const uint8_t* src, size_t len);
bool wrapper_g2_is_neutral(const wrapper_g2_t* value);
bool wrapper_g2_is_valid(const wrapper_g2_t* value);
bool wrapper_g2_is_equal(const wrapper_g2_t* lhs, const wrapper_g2_t* rhs);

typedef gt_t wrapper_gt_t;

void wrapper_gt_init(wrapper_gt_t* gt);
void wrapper_gt_neutral(wrapper_gt_t* gt);
void wrapper_gt_generator(wrapper_gt_t* gt);
void wrapper_gt_rand(wrapper_gt_t* gt);
void wrapper_gt_add_assign(wrapper_gt_t* dst, const wrapper_gt_t* rhs);
void wrapper_gt_add(wrapper_gt_t* dst, const wrapper_gt_t* lhs, const wrapper_gt_t* rhs);
void wrapper_gt_double(wrapper_gt_t* dst, const wrapper_gt_t* src);
void wrapper_gt_neg(wrapper_gt_t* gt);
void wrapper_gt_mul_assign(wrapper_gt_t* dst, const wrapper_bn_t* rhs);
void wrapper_gt_mul(wrapper_gt_t* dst, const wrapper_gt_t* lhs, const wrapper_bn_t* rhs);
void wrapper_gt_size_bin(size_t* size, const wrapper_gt_t* gt);
void wrapper_gt_write_bin(uint8_t* dst, size_t len, const wrapper_gt_t* src);
int wrapper_gt_read_bin(wrapper_gt_t* dst, const uint8_t* src, size_t len);
bool wrapper_gt_is_neutral(const wrapper_gt_t* value);
bool wrapper_gt_is_valid(const wrapper_gt_t* value);
bool wrapper_gt_is_equal(const wrapper_gt_t* lhs, const wrapper_gt_t* rhs);

void wrapper_pc_map(wrapper_gt_t* gt, const wrapper_g1_t* g1, const wrapper_g2_t* g2);
void wrapper_pc_map_sim(wrapper_gt_t* gt, const wrapper_g1_t* g1, const wrapper_g2_t* g2, size_t len);

#endif
