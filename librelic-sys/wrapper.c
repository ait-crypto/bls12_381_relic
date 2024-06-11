#include "wrapper.h"

#include <assert.h>

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

void wrapper_bn_init(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_null(*bn);
    bn_new(*bn);
  }
  RLC_CATCH_ANY {
    bn_free(*bn);
    assert(false);
  }
}

void wrapper_bn_copy(wrapper_bn_t* dst, const wrapper_bn_t* src) {
  RLC_TRY {
    bn_copy(*dst, *src);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_bn_zero(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_zero(*bn);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_bn_one(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_set_dig(*bn, 1);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_get_order(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_null(*bn);
    bn_new(*bn);
    ep_curve_get_ord(*bn);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_bn_add_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_add(*dst, *dst, *rhs);
    bn_mod(*dst, *dst, order);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_bn_add(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_add(*dst, *lhs, *rhs);
    bn_mod(*dst, *dst, order);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_bn_double(wrapper_bn_t* dst, const wrapper_bn_t* src) {
  RLC_TRY {
    bn_dbl(*dst, *src);
    bn_mod(*dst, *dst, order);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_bn_neg(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_sub(*bn, order, *bn);
    bn_mod(*bn, *bn, order);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_bn_sub_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_sub(*dst, *dst, *rhs);
    bn_mod(*dst, *dst, order);
    if (bn_sign(*dst) == RLC_NEG) {
      bn_add(*dst, *dst, order);
    }
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_bn_sub(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_sub(*dst, *lhs, *rhs);
    bn_mod(*dst, *dst, order);
    if (bn_sign(*dst) == RLC_NEG) {
      bn_add(*dst, *dst, order);
    }
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_bn_mul_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_mul(*dst, *dst, *rhs);
    if (bn_sign(*dst) == RLC_NEG) {
      bn_add(*dst, *dst, order);
    } else {
      bn_mod(*dst, *dst, order);
    }
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_bn_mul(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_mul(*dst, *lhs, *rhs);
    if (bn_sign(*dst) == RLC_NEG) {
      bn_add(*dst, *dst, order);
    } else {
      bn_mod(*dst, *dst, order);
    }
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

int wrapper_bn_inv(wrapper_bn_t* val) {
  if (bn_is_zero(*val)) {
    return RLC_ERR;
  }
  RLC_TRY {
    bn_mod_inv(*val, *val, order);
    if (bn_sign(*val) == RLC_NEG) {
      bn_add(*val, *val, order);
    }
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }
  return RLC_OK;
}

size_t wrapper_bn_size_bin(const wrapper_bn_t* bn) {
  size_t size = 0;
  RLC_TRY {
    size = bn_size_bin(*bn);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
  return size;
}

void wrapper_bn_write_bin(uint8_t* dst, size_t len, const wrapper_bn_t* src) {
  RLC_TRY {
    bn_write_bin(dst, len, *src);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

int wrapper_bn_read_bin(wrapper_bn_t* dst, const uint8_t* src, size_t len, bool reduce) {
  RLC_TRY {
    bn_read_bin(*dst, src, len);
    if (reduce) {
      bn_mod(*dst, *dst, order);
    }
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }
  return RLC_OK;
}

void wrapper_bn_rand(wrapper_bn_t* dst, const uint8_t* src, size_t len) {
  RLC_TRY {
    bn_read_bin(*dst, src, len);
    bn_mod(*dst, *dst, order);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

bool wrapper_bn_is_zero(const wrapper_bn_t* value) {
  return bn_is_zero(*value) == 1;
}

bool wrapper_bn_is_odd(const wrapper_bn_t* value) {
  return bn_is_even(*value) == 0;
}

/* --- G1 --- */

void wrapper_g1_init(wrapper_g1_t* g1) {
  memset(g1, 0, sizeof(*g1));
}

void wrapper_g1_neutral(wrapper_g1_t* g1) {
  RLC_TRY {
    g1_set_infty(*g1);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g1_generator(wrapper_g1_t* g1) {
  RLC_TRY {
    g1_get_gen(*g1);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g1_hash_to_curve(wrapper_g1_t* g1, const uint8_t* msg, size_t len, const uint8_t* dst, size_t dst_len) {
  RLC_TRY {
    ep_map_dst(*g1, msg, len, dst, dst_len);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g1_rand(wrapper_g1_t* g1) {
  RLC_TRY {
    g1_rand(*g1);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g1_add_assign(wrapper_g1_t* dst, const wrapper_g1_t* rhs) {
  RLC_TRY {
    g1_add(*dst, *dst, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g1_add(wrapper_g1_t* dst, const wrapper_g1_t* lhs, const wrapper_g1_t* rhs) {
  RLC_TRY {
    g1_add(*dst, *lhs, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g1_double(wrapper_g1_t* dst, const wrapper_g1_t* src) {
  RLC_TRY {
    g1_dbl(*dst, *src);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g1_neg(wrapper_g1_t* g1) {
  RLC_TRY {
    g1_neg(*g1, *g1);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g1_sub_assign(wrapper_g1_t* dst, const wrapper_g1_t* rhs) {
  RLC_TRY {
    g1_sub(*dst, *dst, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g1_sub(wrapper_g1_t* dst, const wrapper_g1_t* lhs, const wrapper_g1_t* rhs) {
  RLC_TRY {
    g1_sub(*dst, *lhs, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g1_mul_assign(wrapper_g1_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    g1_mul(*dst, *dst, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g1_mul(wrapper_g1_t* dst, const wrapper_g1_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    g1_mul(*dst, *lhs, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g1_simmul(wrapper_g1_t* dst, const wrapper_g1_t* g1s, const wrapper_bn_t* scalars, size_t len) {
  RLC_TRY {
    g1_mul_sim_lot(*dst, g1s, scalars, len);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g1_norm(wrapper_g1_t* dst, const wrapper_g1_t* src) {
  RLC_TRY {
    g1_norm(*dst, *src);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

size_t wrapper_g1_size_bin(const wrapper_g1_t* g1) {
  size_t size = 0;
  RLC_TRY {
    size = g1_size_bin(*g1, 0);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
  return size;
}

void wrapper_g1_write_bin(uint8_t* dst, size_t len, const wrapper_g1_t* src) {
  RLC_TRY {
    g1_write_bin(dst, len, *src, 0);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

int wrapper_g1_read_bin(wrapper_g1_t* dst, const uint8_t* src, size_t len) {
  RLC_TRY {
    g1_read_bin(*dst, src, len);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

bool wrapper_g1_is_neutral(const wrapper_g1_t* value) {
  return g1_is_infty(*value) == 1;
}

bool wrapper_g1_is_valid(const wrapper_g1_t* value) {
  return g1_is_valid(*value) == 1;
}

bool wrapper_g1_is_equal(const wrapper_g1_t* lhs, const wrapper_g1_t* rhs) {
  return g1_cmp(*lhs, *rhs) == RLC_EQ;
}

/* --- G2 --- */

void wrapper_g2_init(wrapper_g2_t* g2) {
  memset(g2, 0, sizeof(*g2));
}

void wrapper_g2_neutral(wrapper_g2_t* g2) {
  RLC_TRY {
    g2_set_infty(*g2);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g2_generator(wrapper_g2_t* g2) {
  RLC_TRY {
    g2_get_gen(*g2);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g2_hash_to_curve(wrapper_g2_t* g2, const uint8_t* msg, size_t len, const uint8_t* dst, size_t dst_len) {
  RLC_TRY {
    ep2_map_dst(*g2, msg, len, dst, dst_len);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g2_rand(wrapper_g2_t* g2) {
  RLC_TRY {
    g2_rand(*g2);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g2_add_assign(wrapper_g2_t* dst, const wrapper_g2_t* rhs) {
  RLC_TRY {
    g2_add(*dst, *dst, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g2_add(wrapper_g2_t* dst, const wrapper_g2_t* lhs, const wrapper_g2_t* rhs) {
  RLC_TRY {
    g2_add(*dst, *lhs, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g2_double(wrapper_g2_t* dst, const wrapper_g2_t* src) {
  RLC_TRY {
    g2_dbl(*dst, *src);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g2_neg(wrapper_g2_t* g2) {
  RLC_TRY {
    g2_neg(*g2, *g2);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g2_sub_assign(wrapper_g2_t* dst, const wrapper_g2_t* rhs) {
  RLC_TRY {
    g2_sub(*dst, *dst, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g2_sub(wrapper_g2_t* dst, const wrapper_g2_t* lhs, const wrapper_g2_t* rhs) {
  RLC_TRY {
    g2_sub(*dst, *lhs, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g2_mul_assign(wrapper_g2_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    g2_mul(*dst, *dst, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g2_mul(wrapper_g2_t* dst, const wrapper_g2_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    g2_mul(*dst, *lhs, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g2_simmul(wrapper_g2_t* dst, const wrapper_g2_t* g2s, const wrapper_bn_t* scalars, size_t len) {
  RLC_TRY {
    g2_mul_sim_lot(*dst, g2s, scalars, len);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_g2_norm(wrapper_g2_t* dst, const wrapper_g2_t* src) {
  RLC_TRY {
    g2_norm(*dst, *src);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

size_t wrapper_g2_size_bin(const wrapper_g2_t* g2) {
  size_t size = 0;
  RLC_TRY {
    size = g2_size_bin(*g2, 0);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
  return size;
}

void wrapper_g2_write_bin(uint8_t* dst, size_t len, const wrapper_g2_t* src) {
  RLC_TRY {
    g2_write_bin(dst, len, *src, 0);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

int wrapper_g2_read_bin(wrapper_g2_t* dst, const uint8_t* src, size_t len) {
  RLC_TRY {
    g2_read_bin(*dst, src, len);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

bool wrapper_g2_is_neutral(const wrapper_g2_t* value) {
  return g2_is_infty(*value) == 1;
}

bool wrapper_g2_is_valid(const wrapper_g2_t* value) {
  return g2_is_valid(*value) == 1;
}

bool wrapper_g2_is_equal(const wrapper_g2_t* lhs, const wrapper_g2_t* rhs) {
  return g2_cmp(*lhs, *rhs) == RLC_EQ;
}

/* --- gt --- */

void wrapper_gt_init(wrapper_gt_t* gt) {
  memset(gt, 0, sizeof(*gt));
}

void wrapper_gt_neutral(wrapper_gt_t* gt) {
  RLC_TRY {
    gt_set_unity(*gt);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_gt_generator(wrapper_gt_t* gt) {
  RLC_TRY {
    gt_get_gen(*gt);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_gt_rand(wrapper_gt_t* gt) {
  RLC_TRY {
    gt_rand(*gt);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_gt_add_assign(wrapper_gt_t* dst, const wrapper_gt_t* rhs) {
  RLC_TRY {
    gt_mul(*dst, *dst, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_gt_add(wrapper_gt_t* dst, const wrapper_gt_t* lhs, const wrapper_gt_t* rhs) {
  RLC_TRY {
    gt_mul(*dst, *lhs, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_gt_double(wrapper_gt_t* dst, const wrapper_gt_t* src) {
  RLC_TRY {
    gt_sqr(*dst, *src);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_gt_neg(wrapper_gt_t* gt) {
  RLC_TRY {
    gt_inv(*gt, *gt);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_gt_mul_assign(wrapper_gt_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    gt_exp(*dst, *dst, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_gt_mul(wrapper_gt_t* dst, const wrapper_gt_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    gt_exp(*dst, *lhs, *rhs);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

size_t wrapper_gt_size_bin(const wrapper_gt_t* gt) {
  size_t size = 0;
  RLC_TRY {
    size = gt_size_bin(*(wrapper_gt_t*)gt, 0);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
  return size;
}

void wrapper_gt_write_bin(uint8_t* dst, size_t len, const wrapper_gt_t* src) {
  RLC_TRY {
    gt_write_bin(dst, len, *src, 0);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

int wrapper_gt_read_bin(wrapper_gt_t* dst, const uint8_t* src, size_t len) {
  RLC_TRY {
    gt_read_bin(*dst, src, len);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

bool wrapper_gt_is_neutral(const wrapper_gt_t* value) {
  return gt_is_unity(*value) == 1;
}

bool wrapper_gt_is_valid(const wrapper_gt_t* value) {
  return gt_is_valid(*value) == 1;
}

bool wrapper_gt_is_equal(const wrapper_gt_t* lhs, const wrapper_gt_t* rhs) {
  return gt_cmp(*lhs, *rhs) == RLC_EQ;
}

/* --- pairing --- */

void wrapper_pc_map(wrapper_gt_t* gt, const wrapper_g1_t* g1, const wrapper_g2_t* g2) {
  RLC_TRY {
    pc_map(*gt, *g1, *g2);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}

void wrapper_pc_map_sim(wrapper_gt_t* gt, const wrapper_g1_t* g1, const wrapper_g2_t* g2, size_t len) {
  RLC_TRY {
    pc_map_sim(*gt, g1, g2, len);
  }
  RLC_CATCH_ANY {
    assert(false);
  }
}
