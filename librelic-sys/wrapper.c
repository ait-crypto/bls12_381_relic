#include "wrapper.h"

#include <stdbool.h>

#if ALLOC != AUTO
#error "Only relic with automatic allocation is supported."
#endif

static bool core_init_run = false;
static bn_t order;

__attribute__((constructor)) static void init_relic(void) {
  if (!core_get()) {
    core_init();
    core_init_run = true;
  }

  ep_param_set_any_pairf();

  bn_null(order);
  bn_new(order);
  ep_curve_get_ord(order);
}

__attribute__((destructor)) static void clean_relic(void) {
  if (core_init_run) {
    bn_free(order);

    core_init_run = false;
    core_clean();
  }
}

int wrapper_bn_init(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_null(bn->value);
    bn_new(bn->value);
  }
  RLC_CATCH_ANY {
    bn_free(bn->value);
    return RLC_ERR;
  }

  return RLC_OK;
}

#if 0
int wrapper_bn_free(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_free(bn->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }
  return RLC_OK;
}
#endif

int wrapper_bn_copy(wrapper_bn_t* dst, const wrapper_bn_t* src) {
  RLC_TRY {
    bn_copy(dst->value, src->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }
  return RLC_OK;
}

int wrapper_bn_zero(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_zero(bn->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_one(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_set_dig(bn->value, 1);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_get_order(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_null(bn->value);
    bn_new(bn->value);
    ep_curve_get_ord(bn->value);
  }
  RLC_CATCH_ANY {
    bn_free(bn->value);
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_add_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_add(dst->value, dst->value, rhs->value);
    bn_mod(dst->value, dst->value, order);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_add(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_add(dst->value, lhs->value, rhs->value);
    bn_mod(dst->value, dst->value, order);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_double(wrapper_bn_t* dst, const wrapper_bn_t* src) {
  RLC_TRY {
    bn_dbl(dst->value, src->value);
    bn_mod(dst->value, dst->value, order);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_neg(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_sub(bn->value, order, bn->value);
    bn_mod(bn->value, bn->value, order);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_sub_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_sub(dst->value, dst->value, rhs->value);
    bn_mod(dst->value, dst->value, order);
    if (bn_sign(dst->value) == RLC_NEG) {
      bn_add(dst->value, dst->value, order);
    }
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_sub(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_sub(dst->value, lhs->value, rhs->value);
    bn_mod(dst->value, dst->value, order);
    if (bn_sign(dst->value) == RLC_NEG) {
      bn_add(dst->value, dst->value, order);
    }
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_mul_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_mul(dst->value, dst->value, rhs->value);
    if (bn_sign(dst->value) == RLC_NEG) {
      bn_add(dst->value, dst->value, order);
    } else {
      bn_mod(dst->value, dst->value, order);
    }
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_mul(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_mul(dst->value, lhs->value, rhs->value);
    if (bn_sign(dst->value) == RLC_NEG) {
      bn_add(dst->value, dst->value, order);
    } else {
      bn_mod(dst->value, dst->value, order);
    }
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_inv(wrapper_bn_t* val) {
  if (bn_is_zero(val->value)) {
    return RLC_ERR;
  }
  RLC_TRY {
    bn_mod_inv(val->value, val->value, order);
    if (bn_sign(val->value) == RLC_NEG) {
      bn_add(val->value, val->value, order);
    }
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_size_bin(size_t* size, const wrapper_bn_t* bn) {
  RLC_TRY {
    *size = bn_size_bin(bn->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_write_bin(uint8_t* dst, size_t len, const wrapper_bn_t* src) {
  RLC_TRY {
    bn_write_bin(dst, len, src->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_read_bin(wrapper_bn_t* dst, const uint8_t* src, size_t len, bool reduce) {
  RLC_TRY {
    bn_read_bin(dst->value, src, len);
    if (reduce) {
      bn_mod(dst->value, dst->value, order);
    }
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_rand(wrapper_bn_t* dst, const uint8_t* src, size_t len) {
  RLC_TRY {
    bn_read_bin(dst->value, src, len);
    bn_mod(dst->value, dst->value, order);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

bool wrapper_bn_is_zero(const wrapper_bn_t* value) {
  return bn_is_zero(value->value) == 1;
}

bool wrapper_bn_is_odd(const wrapper_bn_t* value) {
  return bn_is_even(value->value) == 0;
}

/* --- G1 --- */

int wrapper_g1_init(wrapper_g1_t* g1) {
  RLC_TRY {
    g1_null(bn->value);
    g1_new(bn->value);
  }
  RLC_CATCH_ANY {
    g1_free(bn->value);
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_neutral(wrapper_g1_t* g1) {
  RLC_TRY {
    g1_set_infty(g1->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_generator(wrapper_g1_t* g1) {
  RLC_TRY {
    g1_get_gen(g1->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_hash_to_curve(wrapper_g1_t* g1, const uint8_t* msg, size_t len, const uint8_t* dst, size_t dst_len) {
  RLC_TRY {
    ep_map_dst(g1->value, msg, len, dst, dst_len);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_rand(wrapper_g1_t* g1) {
  RLC_TRY {
    g1_rand(g1->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_add_assign(wrapper_g1_t* dst, const wrapper_g1_t* rhs) {
  RLC_TRY {
    g1_add(dst->value, dst->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_add(wrapper_g1_t* dst, const wrapper_g1_t* lhs, const wrapper_g1_t* rhs) {
  RLC_TRY {
    g1_add(dst->value, lhs->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_double(wrapper_g1_t* dst, const wrapper_g1_t* src) {
  RLC_TRY {
    g1_dbl(dst->value, src->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_neg(wrapper_g1_t* g1) {
  RLC_TRY {
    g1_neg(g1->value, g1->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_sub_assign(wrapper_g1_t* dst, const wrapper_g1_t* rhs) {
  RLC_TRY {
    g1_sub(dst->value, dst->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_sub(wrapper_g1_t* dst, const wrapper_g1_t* lhs, const wrapper_g1_t* rhs) {
  RLC_TRY {
    g1_sub(dst->value, lhs->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_mul_assign(wrapper_g1_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    g1_mul(dst->value, dst->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_mul(wrapper_g1_t* dst, const wrapper_g1_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    g1_mul(dst->value, lhs->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_norm(wrapper_g1_t* dst, const wrapper_g1_t* src) {
  RLC_TRY {
    g1_norm(dst->value, src->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_size_bin(size_t* size, const wrapper_g1_t* g1) {
  RLC_TRY {
    *size = g1_size_bin(g1->value, 0);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_write_bin(uint8_t* dst, size_t len, const wrapper_g1_t* src) {
  RLC_TRY {
    g1_write_bin(dst, len, src->value, 0);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g1_read_bin(wrapper_g1_t* dst, const uint8_t* src, size_t len) {
  RLC_TRY {
    g1_read_bin(dst->value, src, len);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

bool wrapper_g1_is_neutral(const wrapper_g1_t* value) {
  return g1_is_infty(value->value) == 1;
}

bool wrapper_g1_is_valid(const wrapper_g1_t* value) {
  return g1_is_valid(value->value) == 1;
}

bool wrapper_g1_is_equal(const wrapper_g1_t* lhs, const wrapper_g1_t* rhs) {
  return g1_cmp(lhs->value, rhs->value) == RLC_EQ;
}

/* --- G2 --- */

int wrapper_g2_init(wrapper_g2_t* g2) {
  RLC_TRY {
    g2_null(bn->value);
    g2_new(bn->value);
  }
  RLC_CATCH_ANY {
    g2_free(bn->value);
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_neutral(wrapper_g2_t* g2) {
  RLC_TRY {
    g2_set_infty(g2->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_generator(wrapper_g2_t* g2) {
  RLC_TRY {
    g2_get_gen(g2->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_hash_to_curve(wrapper_g2_t* g2, const uint8_t* msg, size_t len, const uint8_t* dst, size_t dst_len) {
  RLC_TRY {
    ep2_map_dst(g2->value, msg, len, dst, dst_len);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_rand(wrapper_g2_t* g2) {
  RLC_TRY {
    g2_rand(g2->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_add_assign(wrapper_g2_t* dst, const wrapper_g2_t* rhs) {
  RLC_TRY {
    g2_add(dst->value, dst->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_add(wrapper_g2_t* dst, const wrapper_g2_t* lhs, const wrapper_g2_t* rhs) {
  RLC_TRY {
    g2_add(dst->value, lhs->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_double(wrapper_g2_t* dst, const wrapper_g2_t* src) {
  RLC_TRY {
    g2_dbl(dst->value, src->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_neg(wrapper_g2_t* g2) {
  RLC_TRY {
    g2_neg(g2->value, g2->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_sub_assign(wrapper_g2_t* dst, const wrapper_g2_t* rhs) {
  RLC_TRY {
    g2_sub(dst->value, dst->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_sub(wrapper_g2_t* dst, const wrapper_g2_t* lhs, const wrapper_g2_t* rhs) {
  RLC_TRY {
    g2_sub(dst->value, lhs->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_mul_assign(wrapper_g2_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    g2_mul(dst->value, dst->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_mul(wrapper_g2_t* dst, const wrapper_g2_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    g2_mul(dst->value, lhs->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_norm(wrapper_g2_t* dst, const wrapper_g2_t* src) {
  RLC_TRY {
    g2_norm(dst->value, src->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_size_bin(size_t* size, const wrapper_g2_t* g2) {
  RLC_TRY {
    *size = g2_size_bin(g2->value, 0);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_write_bin(uint8_t* dst, size_t len, const wrapper_g2_t* src) {
  RLC_TRY {
    g2_write_bin(dst, len, src->value, 0);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_g2_read_bin(wrapper_g2_t* dst, const uint8_t* src, size_t len) {
  RLC_TRY {
    g2_read_bin(dst->value, src, len);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

bool wrapper_g2_is_neutral(const wrapper_g2_t* value) {
  return g2_is_infty(value->value) == 1;
}

bool wrapper_g2_is_valid(const wrapper_g2_t* value) {
  return g2_is_valid(value->value) == 1;
}

bool wrapper_g2_is_equal(const wrapper_g2_t* lhs, const wrapper_g2_t* rhs) {
  return g2_cmp(lhs->value, rhs->value) == RLC_EQ;
}

/* --- gt --- */

int wrapper_gt_init(wrapper_gt_t* gt) {
  RLC_TRY {
    gt_null(bn->value);
    gt_new(bn->value);
  }
  RLC_CATCH_ANY {
    gt_free(bn->value);
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_gt_neutral(wrapper_gt_t* gt) {
  RLC_TRY {
    gt_set_unity(gt->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_gt_generator(wrapper_gt_t* gt) {
  RLC_TRY {
    gt_get_gen(gt->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_gt_rand(wrapper_gt_t* gt) {
  RLC_TRY {
    gt_rand(gt->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_gt_add_assign(wrapper_gt_t* dst, const wrapper_gt_t* rhs) {
  RLC_TRY {
    gt_mul(dst->value, dst->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_gt_add(wrapper_gt_t* dst, const wrapper_gt_t* lhs, const wrapper_gt_t* rhs) {
  RLC_TRY {
    gt_mul(dst->value, lhs->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_gt_double(wrapper_gt_t* dst, const wrapper_gt_t* src) {
  RLC_TRY {
    gt_sqr(dst->value, src->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_gt_neg(wrapper_gt_t* gt) {
  RLC_TRY {
    gt_inv(gt->value, gt->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_gt_mul_assign(wrapper_gt_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    gt_exp(dst->value, dst->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_gt_mul(wrapper_gt_t* dst, const wrapper_gt_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    gt_exp(dst->value, lhs->value, rhs->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_gt_size_bin(size_t* size, const wrapper_gt_t* gt) {
  RLC_TRY {
    *size = gt_size_bin(((wrapper_gt_t*)gt)->value, 0);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_gt_write_bin(uint8_t* dst, size_t len, const wrapper_gt_t* src) {
  RLC_TRY {
    gt_write_bin(dst, len, src->value, 0);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_gt_read_bin(wrapper_gt_t* dst, const uint8_t* src, size_t len) {
  RLC_TRY {
    gt_read_bin(dst->value, src, len);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

bool wrapper_gt_is_neutral(const wrapper_gt_t* value) {
  return gt_is_unity(value->value) == 1;
}

bool wrapper_gt_is_valid(const wrapper_gt_t* value) {
  return gt_is_valid(value->value) == 1;
}

bool wrapper_gt_is_equal(const wrapper_gt_t* lhs, const wrapper_gt_t* rhs) {
  return gt_cmp(lhs->value, rhs->value) == RLC_EQ;
}

/* --- pairing --- */

int wrapper_pc_map(wrapper_gt_t* gt, const wrapper_g1_t* g1, const wrapper_g2_t* g2) {
  RLC_TRY {
    pc_map(gt->value, g1->value, g2->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}
